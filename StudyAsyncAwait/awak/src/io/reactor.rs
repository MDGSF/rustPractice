use std::collections::BTreeMap;
use std::io;
use std::mem;
use std::os::unix::io::RawFd;
use std::panic;
use std::sync::{
  atomic::{AtomicUsize, Ordering},
  Arc,
};
use std::task::{Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

use super::poller::{Event, Poller};
use crate::parking;

use concurrent_queue::ConcurrentQueue;
use futures_util::future::poll_fn;
use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};
use slab::Slab;

enum TimerOp {
  Insert(Instant, usize, Waker),
  Remove(Instant, usize),
}

pub(crate) struct Reactor {
  pub block_on_count: AtomicUsize,
  pub unparker: parking::Unparker,
  ticker: AtomicUsize,
  poller: Poller,
  sources: Mutex<Slab<Arc<Source>>>,
  events: Mutex<Vec<Event>>,
  timer_ops: ConcurrentQueue<TimerOp>,
  timers: Mutex<BTreeMap<(Instant, usize), Waker>>,
}

impl Reactor {
  pub fn get() -> &'static Reactor {
    static REACTOR: Lazy<Reactor> = Lazy::new(|| {
      let (parker, unparker) = parking::pair();

      thread::spawn(move || {
        let reactor = Reactor::get();

        let mut last_tick = 0;
        let mut sleeps = 0u64;

        loop {
          let tick = reactor.ticker.load(Ordering::SeqCst);

          if last_tick == tick {
            let reactor_lock = if sleeps >= 10 {
              Some(Reactor::get().lock())
            } else {
              Reactor::get().try_lock()
            };

            if let Some(mut reactor_lock) = reactor_lock {
              let _ = reactor_lock.react(None);
              last_tick = Reactor::get().ticker.load(Ordering::SeqCst);
              sleeps = 0;
            }
          } else {
            last_tick = tick;
          }

          if Reactor::get().block_on_count.load(Ordering::SeqCst) > 0 {
            let delay_us = [50, 75, 100, 250, 500, 750, 1000, 2500, 5000]
              .get(sleeps as usize)
              .unwrap_or(&10_000);

            if !parker.park_timeout(Some(Duration::from_nanos(*delay_us))) {
              sleeps += 1;
            } else {
              sleeps = 0;
              last_tick = Reactor::get().ticker.load(Ordering::SeqCst);
            }
          }
        }
      });

      Reactor {
        block_on_count: AtomicUsize::new(0),
        unparker,
        ticker: AtomicUsize::new(0),
        poller: Poller::new(),
        sources: Mutex::new(Slab::new()),
        events: Mutex::new(Vec::new()),
        timer_ops: ConcurrentQueue::bounded(1000),
        timers: Mutex::new(BTreeMap::new()),
      }
    });

    &REACTOR
  }

  fn lock(&self) -> ReactorLock<'_> {
    let reactor = self;
    let events = self.events.lock();
    ReactorLock { reactor, events }
  }

  pub fn try_lock(&self) -> Option<ReactorLock<'_>> {
    self.events.try_lock().map(|events| {
      let reactor = self;
      ReactorLock { reactor, events }
    })
  }

  fn interest(&self, raw: RawFd, key: usize, read: bool, write: bool) -> io::Result<()> {
    self.poller.interest(raw, key, read, write)
  }

  pub fn insert_io(&self, raw: RawFd) -> io::Result<Arc<Source>> {
    self.poller.insert(raw)?;

    let mut sources = self.sources.lock();
    let entry = sources.vacant_entry();
    let key = entry.key();

    let source = Arc::new(Source {
      raw,
      key,
      wakers: Mutex::new(Wakers {
        readers: Vec::new(),
        writers: Vec::new(),
        tick_readable: 0,
        tick_writable: 0,
      }),
    });

    entry.insert(source.clone());

    Ok(source)
  }

  pub fn remove_io(&self, source: &Source) -> io::Result<()> {
    let mut sources = self.sources.lock();
    sources.remove(source.key);
    self.poller.remove(source.raw)
  }

  pub fn insert_timer(&self, when: Instant, waker: &Waker) -> usize {
    static ID_GENERATOR: AtomicUsize = AtomicUsize::new(1);
    let id = ID_GENERATOR.fetch_add(1, Ordering::Relaxed);

    while self
      .timer_ops
      .push(TimerOp::Insert(when, id, waker.clone()))
      .is_err()
    {
      let mut timers = self.timers.lock();
      self.process_timer_ops(&mut timers);
    }
    self.notify();

    id
  }

  pub fn remove_timer(&self, when: Instant, id: usize) {
    while self.timer_ops.push(TimerOp::Remove(when, id)).is_err() {
      let mut timers = self.timers.lock();
      self.process_timer_ops(&mut timers);
    }
  }

  fn process_timers(&self, wakers: &mut Vec<Waker>) -> Option<Duration> {
    let mut timers = self.timers.lock();
    self.process_timer_ops(&mut timers);

    let now = Instant::now();

    let pending = timers.split_off(&(now, 0));

    let ready = mem::replace(&mut *timers, pending);

    let dur = if ready.is_empty() {
      timers
        .keys()
        .next()
        .map(|(when, _)| when.saturating_duration_since(now))
    } else {
      Some(Duration::from_secs(0))
    };

    for (_, waker) in ready {
      wakers.push(waker);
    }

    dur
  }

  fn process_timer_ops(&self, timers: &mut MutexGuard<'_, BTreeMap<(Instant, usize), Waker>>) {
    for _ in 0..self.timer_ops.capacity().unwrap() {
      match self.timer_ops.pop() {
        Ok(TimerOp::Insert(when, id, waker)) => {
          timers.insert((when, id), waker);
        }
        Ok(TimerOp::Remove(when, id)) => {
          timers.remove(&(when, id));
        }
        Err(_) => break,
      }
    }
  }

  pub fn notify(&self) {
    self.poller.notify().expect("failed to notify reactor");
  }
}

impl Drop for Reactor {
  fn drop(&mut self) {
    self.unparker.unpark();
  }
}

pub(crate) struct ReactorLock<'a> {
  reactor: &'a Reactor,
  events: MutexGuard<'a, Vec<Event>>,
}

impl ReactorLock<'_> {
  pub(crate) fn react(&mut self, timeout: Option<Duration>) -> io::Result<()> {
    let mut wakers = Vec::new();

    let next_timer = self.reactor.process_timers(&mut wakers);

    let timeout = match (next_timer, timeout) {
      (None, None) => None,
      (Some(t), None) | (None, Some(t)) => Some(t),
      (Some(a), Some(b)) => Some(a.min(b)),
    };

    let tick = self
      .reactor
      .ticker
      .fetch_add(1, Ordering::SeqCst)
      .wrapping_add(1);

    self.events.clear();

    let res = match self.reactor.poller.wait(&mut self.events, timeout) {
      Ok(0) => {
        if timeout != Some(Duration::from_secs(0)) {
          self.reactor.process_timers(&mut wakers);
        }
        Ok(())
      }
      Ok(_) => {
        let sources = self.reactor.sources.lock();

        for ev in self.events.iter() {
          if let Some(source) = sources.get(ev.key) {
            let mut w = source.wakers.lock();

            if ev.readable {
              w.tick_readable = tick;
              wakers.append(&mut w.readers);
            }

            if ev.writable {
              w.tick_writable = tick;
              wakers.append(&mut w.writers);
            }

            if !(w.readers.is_empty() && w.writers.is_empty()) {
              self.reactor.poller.interest(
                source.raw,
                source.key,
                !w.readers.is_empty(),
                !w.writers.is_empty(),
              )?;
            }
          }
        }

        Ok(())
      }
      Err(err) if err.kind() == io::ErrorKind::Interrupted => Ok(()),
      Err(err) => Err(err),
    };

    for waker in wakers {
      let _ = panic::catch_unwind(|| waker.wake());
    }

    res
  }
}

#[derive(Debug)]
pub(crate) struct Source {
  pub raw: RawFd,
  key: usize,
  wakers: Mutex<Wakers>,
}

#[derive(Debug)]
struct Wakers {
  readers: Vec<Waker>,
  writers: Vec<Waker>,
  tick_readable: usize,
  tick_writable: usize,
}

impl Source {
  pub async fn readable(&self) -> io::Result<()> {
    let mut ticks = None;

    poll_fn(|cx| {
      let mut w = self.wakers.lock();

      if let Some((a, b)) = ticks {
        if w.tick_readable != a && w.tick_readable != b {
          return Poll::Ready(Ok(()));
        }
      }

      if w.readers.is_empty() {
        Reactor::get().interest(self.raw, self.key, true, !w.writers.is_empty())?;
      }

      if w.readers.iter().all(|w| w.will_wake(cx.waker())) {
        w.readers.push(cx.waker().clone());
      }

      if ticks.is_none() {
        ticks = Some((
          Reactor::get().ticker.load(Ordering::SeqCst),
          w.tick_readable,
        ));
      }

      Poll::Pending
    })
    .await
  }

  pub async fn writable(&self) -> io::Result<()> {
    let mut ticks = None;

    poll_fn(|cx| {
      let mut w = self.wakers.lock();

      if let Some((a, b)) = ticks {
        if w.tick_writable != a && w.tick_writable != b {
          return Poll::Ready(Ok(()));
        }
      }

      if w.writers.is_empty() {
        Reactor::get().interest(self.raw, self.key, !w.readers.is_empty(), true)?;
      }

      if w.writers.iter().all(|w| !w.will_wake(cx.waker())) {
        w.writers.push(cx.waker().clone());
      }

      if ticks.is_none() {
        ticks = Some((
          Reactor::get().ticker.load(Ordering::SeqCst),
          w.tick_writable,
        ));
      }

      Poll::Pending
    })
    .await
  }
}

//#![allow(dead_code)]
//#![allow(unused_imports)]
//Serializer::with(wr, StructMapWriter)

extern crate chrono;
extern crate structopt;
extern crate ws;

use chrono::{TimeZone, Utc};
use std::cmp::min;
use std::collections::HashMap;
use structopt::StructOpt;
use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};

use rust_msgpack::decode;
use rust_msgpack::encode;
use value::from_value::FromValue;
use value::into_value::IntoValue;
use value_derive::*;

use flowout::*;

#[derive(Debug, Default, FromValue, IntoValue)]
struct TMessage {
    source: String,
    topic: String,
    data: Vec<u8>,
    time: i64,
}

struct Client<'a> {
    out: Sender,
    opt: &'a Opt,
    message_counter: i32,
}

impl<'a> Client<'a> {
    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Text(data) => println!("unexpected message: {}", data),
            Message::Binary(data) => {
                self.handle_binary_message(&data);
            }
        }
    }

    fn handle_binary_message(&mut self, data: &[u8]) {
        //println!("data = {:x?}", data);

        let v = decode::decode_to_value(&data).unwrap();
        let msg: TMessage = v.from_value();

        let ts = Utc.timestamp(
            (msg.time / 1000000) as i64,
            (msg.time % 1000000 * 1000) as u32,
        );
        print!(
            "[{}] {:?} @{}: \"{}\" {}\n",
            self.message_counter,
            ts,
            msg.source,
            msg.topic,
            msg.data.len(),
        );

        self.show_binary_message(&msg);
    }

    fn show_binary_message(&mut self, msg: &TMessage) {
        let total_size = msg.data.len();
        let slice_size = min(total_size, 1024);
        let result = hex::hexcode::hexdump(&msg.data[..slice_size]);
        println!("{}", result);
    }
}

impl<'a> Handler for Client<'a> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!(
            "connecting to {} success, send subscribe to server..",
            self.opt.server
        );

        let subs = self.opt.sub.clone();
        let topics: Vec<&str> = subs.split(',').collect();
        for &topic in topics.iter() {
            let mut sub = HashMap::new();
            sub.insert("source", &self.opt.name[..]);
            sub.insert("topic", "subscribe");
            sub.insert("data", topic);

            println!("send subscribe topic {:?}", sub);

            let bin = encode::encode(&sub).unwrap();
            self.out.send(bin)?;
        }

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.message_counter += 1;

        self.handle_message(msg);

        if self.opt.limit > 0 && self.message_counter >= self.opt.limit {
            self.out.close(CloseCode::Normal)?;
        }

        Ok(())
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "flowout",
    about = "Used to subscribe data from libflow server."
)]
struct Opt {
    #[structopt(short = "s", long = "server", default_value = "127.0.0.1:24012")]
    server: String,

    #[structopt(short = "t", long = "sub", default_value = "*")]
    sub: String,

    #[structopt(short = "e", long = "text", default_value = "")]
    text: String,

    #[structopt(short = "m", long = "msgpack", default_value = "")]
    msgpack: String,

    #[structopt(short = "l", long = "limit", default_value = "0")]
    limit: i32,

    #[structopt(short = "n", long = "name", default_value = "flowout")]
    name: String,
}

fn main() {
    let opt = Opt::from_args();
    let serveraddr = String::from("ws://") + &opt.server;
    connect(serveraddr, |out| Client {
        out: out,
        opt: &opt,
        message_counter: 0,
    })
    .unwrap();
}

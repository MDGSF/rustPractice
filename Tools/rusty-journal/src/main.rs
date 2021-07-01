use structopt::StructOpt;
mod cli;
mod tasks;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
  home::home_dir().map(|mut path| {
    path.push(".rusty-journal.json");
    path
  })
}

fn main() -> anyhow::Result<()> {
  let CommandLineArgs {
    action,
    journal_file,
  } = CommandLineArgs::from_args();

  let journal_file = journal_file
    .or_else(find_default_journal_file)
    .ok_or(anyhow!("Failed to find journal file"))?;

  match action {
    Add { text } => tasks::add_task(journal_file, Task::new(text)),
    Done { position } => tasks::complete_task(journal_file, position),
    List => tasks::list_tasks(journal_file),
  }?;

  Ok(())
}

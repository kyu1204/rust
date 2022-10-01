mod cli;
mod tasks;

use structopt::StructOpt;
use std::path::PathBuf;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn find_default_journal_home() -> Option<PathBuf> {
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
        .or_else(find_default_journal_home)
        .expect("Failed to find journal file");

    match action {
        Add {text} => tasks::add_task(journal_file, Task::new(text)),
        Done {position} => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file)

    }?;
    Ok(())
}
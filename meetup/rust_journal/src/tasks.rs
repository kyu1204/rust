use std::fmt;
use chrono::{serde::ts_seconds, DateTime, Utc, Local};
use serde::{Serialize, Deserialize};
use fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task {
            text,
            created_at
        }
    }
}
impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open File
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Read Data
    let mut tasks: Vec<Task> = collect_task(&file)?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Read Data
    let mut tasks: Vec<Task> = collect_task(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(journal_path)?;

    let tasks: Vec<Task> = collect_task(&file)?;

    if tasks.is_empty() {
        println!("Task is Empty!");
    }
    else {
        let mut number = 1;
        for task in tasks {
            println!("{}: {}", number, task);
            number += 1;
        }
    }

    Ok(())
}

pub fn collect_task(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.

    Ok(tasks)
}
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: usize,
    description: String,
}

impl Task {
    pub fn new(id: usize, desc: String) -> Self {
        Self {
            id: id,
            description: desc,
        }
    }
}

pub struct IOManager<'a> {
    file_path: &'a Path,
}

impl IOManager<'_> {
    fn new() -> Self {
        Self {
            file_path: Path::new("./tasks.json"),
        }
    }

    fn get_file(&self) -> Option<File> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(self.file_path);

        if let Ok(file) = file {
            Some(file)
        } else {
            dbg!("Unable to find file. Creating new at: {:?}", self.file_path);
            dbg!("Unable creating new at: {:?}", self.file_path);
            None
        }
    }

    fn get_all_tasks(&self) -> Option<Vec<Task>> {
        if let Some(mut file) = self.get_file() {
            let mut data_string = String::new();
            file.read_to_string(&mut data_string);
            if data_string.len() > 0 {
                let data_string: Vec<&str> = data_string.split('>').collect();
                let (data_string, _) = data_string.split_at(data_string.len() - 1);
                dbg!(data_string);
                let task_json_fmt = format!("[ {} ]", data_string.join(","));
                return Some(serde_json::from_str(&task_json_fmt).unwrap());
            }
        }
        None
    }

    pub fn write_task(&self, tasks: Vec<Task>) {
        if let Some(mut file) = self.get_file() {
            for task in tasks {
                let task = serde_json::to_string(&task).unwrap();
                writeln!(file, "{}>", task);
            }
        }
    }

    pub fn remove_task(&self, id: usize) -> Option<Task> {
        None
    }
}

#[test]
fn write_tasks() {
    let t1 = Task::new(45, "Add a new task of day 1".to_string());
    let t2 = Task::new(45, "Add a new task of day 2".to_string());
    let t3 = Task::new(45, "Add a new task of day 3".to_string());

    let io_manager = IOManager::new();
    io_manager.write_task(vec![t1, t2, t3]);
    dbg!(io_manager.get_all_tasks());
}

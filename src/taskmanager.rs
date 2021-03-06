use colored::*;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: String,
    description: String,
}

impl Task {
    pub fn new(desc: String) -> Self {
        let mut id = Uuid::new_v4().to_string();
        id.truncate(8);
        Self {
            id: id,
            description: desc,
        }
    }

    fn print_task(&self) {
        println!("[{}]: {}", self.id.yellow(), self.description.green());
    }
}

pub struct TaskManager<'a> {
    file_path: &'a Path,
}

impl<'a> TaskManager<'a> {
    pub fn new(file_path: &'a Path) -> Self {
        Self {
            file_path: file_path,
        }
    }

    fn remove_file(&self) {
        match std::fs::remove_file(self.file_path) {
            Ok(ok) => ok,
            Err(e) => println!("An error occured on removing tasks file: {}", e),
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
            println!("Unable to find file. Creating new at: {:?}", self.file_path);
            println!("Unable creating new at: {:?}", self.file_path);
            None
        }
    }

    fn get_all_tasks(&self) -> Result<Option<Vec<Task>>, Error> {
        if let Some(mut file) = self.get_file() {
            let mut data_string = String::new();
            file.read_to_string(&mut data_string)?;
            if data_string.len() > 0 {
                let data_string: Vec<&str> = data_string.split('>').collect();
                let (data_string, _) = data_string.split_at(data_string.len() - 1);
                let task_json_fmt = format!("[ {} ]", data_string.join(","));
                return Ok(Some(serde_json::from_str(&task_json_fmt).unwrap()));
            }
        }
        Ok(None)
    }

    pub fn write_task(&self, tasks: Vec<Task>) -> Result<(), Error> {
        if let Some(mut file) = self.get_file() {
            for task in tasks {
                let task = serde_json::to_string(&task).unwrap();
                writeln!(file, "{}>", task)?;
            }
        }
        Ok(())
    }

    pub fn remove_tasks(&self, ids: Vec<String>) -> Result<(), Error> {
        match self.get_all_tasks() {
            Ok(Some(mut tasks)) => {
                let old_count = tasks.len();
                tasks.retain(|x| !ids.contains(&x.id));
                let new_count = tasks.len();
                if old_count == new_count {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("No task found with give ID: {:?}", ids),
                    ));
                }
                self.remove_file();
                if tasks.len() > 0 {
                    return self.write_task(tasks);
                }

                Ok(())
            }
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    pub fn print_task_to_cli(&self) {
        match self.get_all_tasks() {
            Ok(tasks) => {
                if let Some(tasks) = tasks {
                    for task in tasks {
                        task.print_task();
                    }
                } else {
                    println!("There is no task created yet.");
                }
            }
            Err(e) => println!("An error occured on geting tasks: {}", e),
        }
    }
}

mod test {
    #[allow(unused)]
    use super::{Path, Task, TaskManager};

    #[test]
    fn write_tasks() {
        let t1 = Task::new("write_tasks 1 ".to_string());
        let t2 = Task::new("write_tasks 2".to_string());

        let task_manager = TaskManager::new(&Path::new("./tasks1.json"));
        task_manager.write_task(vec![t1, t2]).unwrap();
        let al_task_from_file = task_manager.get_all_tasks().unwrap().unwrap();

        assert_eq!(al_task_from_file[0].description, "write_tasks 1 ");
        assert_eq!(al_task_from_file[1].description, "write_tasks 2");
        task_manager.remove_file();
    }

    #[test]
    fn remove_tasks() {
        let t1 = Task::new("remove_tasks 1".to_string());
        let t2 = Task::new("remove_tasks 2".to_string());

        let task_manager = TaskManager::new(&Path::new("./tasks2.json"));
        task_manager.write_task(vec![t1, t2]).unwrap();
        let al_task_from_file = task_manager.get_all_tasks().unwrap().unwrap();
        let id = &al_task_from_file[0].id;
        task_manager.remove_tasks(vec![id.to_owned()]).unwrap();

        let al_task_from_file = task_manager.get_all_tasks().unwrap().unwrap();
        assert_eq!(al_task_from_file[0].description, "remove_tasks 2");
        task_manager.remove_file();
    }
}

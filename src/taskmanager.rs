use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: String,
    description: String,
}

impl Task {
    pub fn new(id: String, desc: String) -> Self {
        Self {
            id: id,
            description: desc,
        }
    }
}

pub struct IOManager<'a> {
    file_path: &'a Path,
}

impl<'a> IOManager<'a> {
    fn new(file_path: &'a Path) -> Self {
        Self {
            file_path: file_path,
        }
    }

    fn remove_file(&self){
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
            dbg!("Unable to find file. Creating new at: {:?}", self.file_path);
            dbg!("Unable creating new at: {:?}", self.file_path);
            None
        }
    }

    fn get_all_tasks(&self) -> Result<Option<Vec<Task>>, std::io::Error> {
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

    pub fn write_task(&self, tasks: Vec<Task>) {
        if let Some(mut file) = self.get_file() {
            for task in tasks {
                let task = serde_json::to_string(&task).unwrap();
                if let Ok(_) = writeln!(file, "{}>", task) {
                    println!("Successfully write task {:?} to file: {:?}", task, file);
                }
            }
        }
    }

    pub fn remove_tasks(&self, ids: Vec<String>) {
        match self.get_all_tasks() {
            Ok(Some(mut tasks)) => {
                tasks.retain(|x| !ids.contains(&x.id));
                self.remove_file();
                if tasks.len() > 0 {
                    self.write_task(tasks);
                }
            }
            Ok(_) => {}
            Err(e) => println!("An error occured on geting tasks: {}", e),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn write_tasks() {
        let t1 = Task::new(String::from("45"), "write_tasks 1 ".to_string());
        let t2 = Task::new(String::from("42"), "write_tasks 2".to_string());

        let io_manager = IOManager::new(&Path::new("./tasks1.json"));
        io_manager.write_task(vec![t1, t2]);
        
        let al_task_from_file = io_manager.get_all_tasks().unwrap().unwrap();
        
        assert_eq!(al_task_from_file[0].id, "45");
        assert_eq!(al_task_from_file[1].id, "42");
        io_manager.remove_file();
    }

    #[test]
    fn remove_tasks() {
        let t1 = Task::new(String::from("42"), "remove_tasks 1".to_string());
        let t2 = Task::new(String::from("45"), "remove_tasks 2".to_string());

        let io_manager = IOManager::new(&Path::new("./tasks2.json"));
        io_manager.write_task(vec![t1, t2]);
        io_manager.remove_tasks(vec!["45".to_string()]);
        
        let al_task_from_file = io_manager.get_all_tasks().unwrap().unwrap();
        
        assert_eq!(al_task_from_file[0].id, "42");
        io_manager.remove_file();
    }
}

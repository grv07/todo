mod taskmanager;

use clap::{App, Arg};
use colored::*;
use std::path::Path;
use taskmanager::{TaskManager, Task};

fn main() {
    parse_args();
}

fn parse_args() {
    let matches = App::new("A simple task manager for CLI")
        .version("0.1")
        .author("Gaurav Tyagi")
        .about("A simple task manager for CLI")
        .arg(
            Arg::with_name("task")
                .short("t")
                .long("task")
                .value_name("Text")
                .help("Create new task"),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .value_name("Text")
                .help("Input ID of task to drop task"),
        )
        .get_matches();

    let task_manager = TaskManager::new(&Path::new("./tasks.json"));

    if let Some(id) = matches.value_of("delete") {
        match task_manager.remove_tasks(vec![id.to_string()]) {
            Ok(_) => {
                println!("{}", "Successfully remove task :)".green());
            }
            Err(e) => println!("{} {}", "Error: ".red(), e.to_string().magenta()),
        }
    }

    match matches.value_of("task") {
        Some(task) => {
            let task = Task::new(task.to_string());
            // TODO: Create task obj should not required here.
            if let Ok(_) = task_manager.write_task(vec![task]) {
                println!("{}", "Successfully write task :)".green());
            }
        }
        None => {
            task_manager.print_task_to_cli();
        }
    }
}

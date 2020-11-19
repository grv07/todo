mod taskmanager;

use clap::{App, Arg};
use std::path::Path;
use taskmanager::{IOManager, Task};

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

    let io_manager = IOManager::new(&Path::new("./tasks.json"));

    if let Some(id) = matches.value_of("delete") {
        match io_manager.remove_tasks(vec![id.to_string()]) {
            Ok(_) => {
                println!("Successfully remove task :)");
            },
            Err(e) => println!("Error in removing task: {}",e)
        }
    }

    match matches.value_of("task") {
        Some(task) => {
            let task = Task::new(task.to_string());
            // TODO: Create task obj should not required here.
            if let Ok(_) = io_manager.write_task(vec![task]) {
                println!("Successfully write task :)");
            }
        }
        None => {
            io_manager.print_task_to_cli();
        }
    }
}

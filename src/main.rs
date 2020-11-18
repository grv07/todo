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
            Arg::with_name("description")
                .short("d")
                .long("desc")
                .value_name("Text")
                .help("Sets task description"),
        )
        .get_matches();

    let io_manager = IOManager::new(&Path::new("./tasks.json"));

    match matches.value_of("description") {
        Some(task) => {
            let task = Task::new(task.to_string());
            // TODO: Create task obj should not required here.
            io_manager.write_task(vec![task])
        }
        None => {
            io_manager.print_task_to_cli();
        }
    }
}

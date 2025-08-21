use std::io::{self};

use crate::commands::CommandExecutor;
use crate::utils::parse_command;

pub struct Shell {
    executor: CommandExecutor,
    running: bool,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            executor: CommandExecutor::new(),
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!();
                    println!("Goodby hoomie!");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();
                    // execute the command if it's not empty >>> CommandExecuter
                    if input != "" {
                        self.execute_command(input);
                    }
                    println!("Your input is: {}", input);
                }
                Err(error) => {
                    println!("Error reading input: {:?}", error);
                }
            }
        }
    }

    fn execute_command(&mut self, input: &str) {
        let commands: Vec<&str> = input.split(";").collect();

        for cmd in commands {
            let cmd = cmd.trim();
            if cmd != "" {
                let (command, args) = parse_command(cmd);

                if command == "exit" {
                    println!("GoodBy Hoomie!");
                    self.running = false;
                    return;
                }

                self.executor.execute(&command, &args);
            }
        }
    }
}

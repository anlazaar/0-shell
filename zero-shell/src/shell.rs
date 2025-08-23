use std::io::{ self };

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
                    println!("Goodbye!");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();
                    if !input.is_empty() {
                        // COMMIT 3: Task - Execute parsed commands
                        self.execute_command(input);
                    }
                }
                // COMMIT 2: Task - Handle input errors
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
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

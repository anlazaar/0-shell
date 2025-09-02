use std::io::{ self, Write };

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
            self.display_current();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!();
                    println!("Goodbye mate!");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();
                    if !input.is_empty() {
                        self.execute_command(input);
                    }
                }
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

    fn display_current(&self) {
        let current_dir = std::env
            ::current_dir()
            .map(|path| {
                if let Some(home) = std::env::var("HOME").ok() {
                    path.to_string_lossy().replace(&home, "~")
                } else {
                    path.to_string_lossy().to_string()
                }
            })
            .unwrap();

        print!("\x1b[32m{}\x1b[0m $ ", current_dir);
        io::stdout().flush().unwrap();
    }
}

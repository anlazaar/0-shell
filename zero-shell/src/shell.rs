use std::io::{ self, Write };

use crate::commands::CommandExecutor;
use crate::helpers::clean_input;
use crate::utils::parse_command;
use std::{ env };

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
    fn display_banner(&self) {
        println!("\x1b[36m"); // Cyan color
        println!(
            r#"
 ██████╗       ███████╗██╗  ██╗███████╗██╗     ██╗     
██╔═████╗      ██╔════╝██║  ██║██╔════╝██║     ██║     
██║██╔██║█████╗███████╗███████║█████╗  ██║     ██║     
████╔╝██║╚════╝╚════██║██╔══██║██╔══╝  ██║     ██║     
╚██████╔╝      ███████║██║  ██║███████╗███████╗███████╗
 ╚═════╝       ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
"#
        );
        println!("\x1b[0m");
        println!("\x1b[33m🚀 Welcome to 0-Shell\x1b[0m");
        println!("\x1b[90mType '--help' for available commands or 'exit' to quit\x1b[0m");
        println!();
    }

    pub fn run(&mut self) {
        self.display_banner();
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
                    let input = clean_input(input.trim());
                    if !input.is_empty() {
                        if input.starts_with("echo") {
                            let mut content = input
                                .strip_prefix("echo")
                                .unwrap()
                                .trim()
                                .to_string();

                            let in_single = content.starts_with("'");
                            let in_double = content.starts_with('"');

                            let mut c_quotes = if content.starts_with("'") {
                                content.matches('\'').count() % 2 != 0
                            } else if content.starts_with("\"") {
                                content.matches('\"').count() % 2 != 0
                            } else {
                                false
                            };

                            while c_quotes {
                                print!("dquote> ");
                                content.push('\n');

                                io::stdout().flush().unwrap();

                                let mut to_echo = String::new();
                                match io::stdin().read_line(&mut to_echo) {
                                    Ok(0) => {
                                        break;
                                    }
                                    Ok(_) => {
                                        content.push_str(to_echo.trim());
                                        c_quotes = if content.starts_with("'") {
                                            content.matches('\'').count() % 2 != 0
                                        } else if content.starts_with("\"") {
                                            content.matches('\"').count() % 2 != 0
                                        } else {
                                            false
                                        };
                                    }
                                    Err(_) => {
                                        break;
                                    }
                                }
                            }

                            let mut cleaned = content;
                            if in_single {
                                cleaned = cleaned.replace("\"", "");
                            } else if in_double {
                                cleaned = cleaned.replace("'", "");
                            }

                            let args: Vec<String> = cleaned
                                .split(|c| {
                                    match c {
                                        ' ' | '\t' => true,
                                        _ => false,
                                    }
                                })
                                .map(|s| s.to_string())
                                .collect();

                            self.executor.execute("echo", &args);
                        } else {
                            self.execute_command(input.as_str());
                        }
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
        let mut current_dir = std::env
            ::current_dir()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or(env::var("PWD").unwrap_or("~".to_string()));

        if let Some(home) = std::env::var("HOME").ok() {
            current_dir = current_dir.replace(&home, "~");
        }
        print!("\x1b[32m{}\x1b[0m $ ", current_dir);
        io::stdout().flush().unwrap();
    }
}

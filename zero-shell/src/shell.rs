use std::io::{ self, Write };

use crate::commands::CommandExecutor;
use crate::helpers::clean_input;

use crate::parser::{ parse_command, ParseResult };
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
                        self.handle_input(input);
                    }
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                }
            }
        }
    }

    fn handle_input(&mut self, mut input: String) {
        loop {
            match parse_command(&input) {
                Ok(ParseResult::Complete(words)) => {
                    if !words.is_empty() {
                        self.execute_parsed_command(words);
                    }
                    break;
                }
                Ok(ParseResult::NeedsContinuation(signal)) => {
                    print!("{}", signal);
                    io::stdout().flush().unwrap();

                    let mut continuation = String::new();
                    match io::stdin().read_line(&mut continuation) {
                        Ok(0) => {
                            eprintln!("syntax error: unexpected end of file");
                            break;
                        }
                        Ok(_) => {
                            input.push('\n');
                            input.push_str(continuation.trim());
                        }
                        Err(_) => {
                            eprintln!("Error reading continuation");
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    break;
                }
            }
        }
    }

    fn execute_parsed_command(&mut self, words: Vec<String>) {
        if words.is_empty() {
            return;
        }

        let command = &words[0];
        let args = &words[1..];

        if command == "exit" {
            println!("GoodBy Hoomie!");
            self.running = false;
            return;
        }

        self.executor.execute(command, args);
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

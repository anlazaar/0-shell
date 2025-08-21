use std::io;

pub struct Shell {
    running: bool,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
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
                    // execute the command if it's not empty >>> Command Executer
                    println!("Your input is: {}", input);
                }
                Err(error) => {
                    println!("Error reading input: {:?}", error);
                }
            }
        }
    }
}

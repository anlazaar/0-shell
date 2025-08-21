use std::collections::HashMap;

pub struct CommandExecutor {
    commands: HashMap<String, Box<dyn Fn(&[String])>>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        // next this will be mut for insert [command <-> Fn]
        let commands: HashMap<String, Box<dyn Fn(&[String])>> = HashMap::new();

        // Thinking of making a HashMap that containes every command with it's function >>> like handleFunc or a Multiplexere
        CommandExecutor { commands }
    }

    pub fn execute(&self, command: &str, args: &[String]) {
        if let Some(function) = self.commands.get(command) {
            function(args);
        } else {
            println!("Command {} not found", command);
        }
    }
}

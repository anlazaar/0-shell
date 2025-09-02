mod builtin;

use builtin::{ echo, pwd, cd, ls, mkdir, rm, help, clear };
use std::collections::HashMap;

pub struct CommandExecutor {
    commands: HashMap<String, Box<dyn Fn(&[String])>>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        // next this will be mut for insert [command <-> Fn]
        let mut commands: HashMap<String, Box<dyn Fn(&[String])>> = HashMap::new();

        // echo
        commands.insert("echo".to_string(), Box::new(echo));
        // pwd
        commands.insert("pwd".to_string(), Box::new(pwd));
        // cd
        commands.insert("cd".to_string(), Box::new(cd));
        // ls
        commands.insert("ls".to_string(), Box::new(ls));
        // mkdir
        commands.insert("mkdir".to_string(), Box::new(mkdir));
        // rm
        commands.insert("rm".to_string(), Box::new(rm));
        // clear
        commands.insert("clear".to_string(), Box::new(clear));
        // rm
        commands.insert("--help".to_string(), Box::new(help));

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

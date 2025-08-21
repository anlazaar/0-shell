mod shell;
mod utils;
mod commands;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}

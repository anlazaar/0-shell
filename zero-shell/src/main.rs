mod shell;
mod utils;
mod commands;
mod helpers;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}

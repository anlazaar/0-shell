mod commands;
mod helpers;
mod shell;
mod utils;

use shell::Shell;
use std::mem::{ zeroed };


type SigHandler = extern "C" fn(i32);
#[cfg(unix)]
unsafe extern "C" {
    unsafe fn signal(signum: i32, handler: SigHandler) -> SigHandler;
}

extern "C" fn ignore_sigint(_sig: i32) {}

fn main() {
    unsafe {
        signal(2, ignore_sigint); // SIGINT = 2
        let mut term: libc::termios = zeroed();
        libc::tcgetattr(0, &mut term); // 0 = stdin
        term.c_lflag &= !libc::ISIG; // Turn off ISIG
        libc::tcsetattr(0, libc::TCSANOW, &term);
    }

    let mut shell = Shell::new();
    shell.run();
}

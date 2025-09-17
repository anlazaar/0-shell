use crate::helpers::{
    blocks512_for_path, format_permissions, format_time, gid_to_groupname, uid_to_username,
};
use std::ffi::{CStr, CString};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::{env, usize};

use libc::{self, DIR, PATH_MAX, c_char, closedir, lstat, opendir, readdir, readlink, stat, major, minor};

pub fn echo(args: &[String]) {
    if args.len() == 0 {
        println!();
        return;
    }
    let output = args.join(" ");
    println!("{}", output);
}

pub fn pwd(_args: &[String]) {
    // using what env stand for >> environment-related which return a Result<PathBuf, std::io::Error>
    match env::current_dir() {
        // Using path.display cause the path is a PathBuf which does not inpliment Display, but has it's own display Method --Helper--
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn cd(args: &[String]) {
    if args.len() > 1 {
        println!("cd: too many arguments");
        return;
    }

    let path: String;
    if args.len() == 1 && args[0] == "-" {
        path = env::var("OLDPWD").unwrap_or("/".to_string());
    } else if args.len() == 0 || (args.len() == 1 && args[0] == "~") {
        path = env::var("HOME").unwrap_or("/".to_string());
    } else {
        path = args[0].clone();
    }
    let old: String;
    match env::current_dir() {
        Ok(path) => {
            old = path
                .into_os_string()
                .into_string()
                .unwrap_or("".to_string())
        }
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    if let Err(_) = env::set_current_dir(&path) {
        println!("cd: -- {} -- No such a file or dir", path);
    } else {
        unsafe {
            env::set_var("OLDPWD", &old);
        }
    }
}

pub fn ls(args: &[String]) {
    let mut a_flag = false;
    let mut f_flag = false;
    let mut l_flag = false;
    let mut paths = Vec::new();

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => {
                        a_flag = true;
                    }
                    'l' => {
                        l_flag = true;
                    }
                    'F' => {
                        f_flag = true;
                    }
                    _ => {
                        println!("ls: invalid flag '{}'", ch);
                        return;
                    }
                }
            }
        } else {
            paths.push(arg.clone());
        }
    }
    if paths.len() == 0 {
        paths.push(".".to_string());
    }

    for (i, path) in paths.iter().enumerate() {
        list_dir(&path, a_flag, l_flag, f_flag);
        if i != paths.len() - 1 {
            println!();
        }
    }
}

fn list_dir(path: &str, a_flag: bool, l_flag: bool, f_flag: bool) {
    let c_path = CString::new(path).unwrap();
    let pth = Path::new(path);
    unsafe {
        let mut entries = Vec::new();

        if pth.is_dir() {
            let dir: *mut DIR = opendir(c_path.as_ptr());
            if dir.is_null() {
                println!("Error: cannot open directory {}", path);
                return;
            }

            loop {
                let entry = readdir(dir);
                if entry.is_null() {
                    break;
                }

                let d_name = CStr::from_ptr((*entry).d_name.as_ptr());
                let name = d_name.to_string_lossy().into_owned();

                if !a_flag && name.starts_with('.') {
                    continue;
                }

                entries.push(name);
            }
            closedir(dir);

            entries.sort_by(|a, b| match (a.as_str(), b.as_str()) {
                (".", _) => std::cmp::Ordering::Less,
                (_, ".") => std::cmp::Ordering::Greater,
                ("..", _) if a != "." => std::cmp::Ordering::Less,
                (_, "..") if b != "." => std::cmp::Ordering::Greater,
                _ => {
                    let a_name = if a.starts_with('.') && a != "." && a != ".." {
                        &a[1..]
                    } else {
                        &a
                    };
                    let b_name = if b.starts_with('.') && b != "." && b != ".." {
                        &b[1..]
                    } else {
                        &b
                    };
                    a_name.to_lowercase().cmp(&b_name.to_lowercase())
                }
            });
        } else {
            entries.push(path.to_string());
        }

        if l_flag {
            if pth.is_dir() {
                println!("{path}:");

                let mut total_blocks_512: u64 = 0;
                for name in &entries {
                    let full_path = format!("{}/{}", path, name);
                    if let Some(b512) = blocks512_for_path(&full_path) {
                        total_blocks_512 += b512;
                    }
                }

                let total_1k = (total_blocks_512 + 1) / 2;
                println!("total {}", total_1k);
            }

            for mut name in entries {
                let full_path: String;
                if pth.is_dir() {
                    full_path = format!("{}/{}", path, name);
                } else {
                    full_path = path.to_string();
                }

                if f_flag {
                    let mut st: stat = std::mem::zeroed();
                    let c_full = CString::new(full_path.clone()).unwrap();

                    if lstat(c_full.as_ptr(), &mut st) == 0
                        && (st.st_mode & libc::S_IFMT) == libc::S_IFLNK
                    {
                        let mut buf = vec![0u8; PATH_MAX as usize];
                        let len = readlink(
                            c_full.as_ptr(),
                            buf.as_mut_ptr() as *mut c_char,
                            PATH_MAX as usize,
                        );
                        if len >= 0 {
                            let target_path =
                                String::from_utf8_lossy(&buf[..len as usize]).to_string();
                            name.push_str(" -> ");
                            name.push_str(&target_path);
                        }
                    }

                    if stat(c_full.as_ptr(), &mut st) == 0 {
                        if (st.st_mode & libc::S_IFMT) == libc::S_IFDIR {
                            name.push('/');
                        } else if (st.st_mode & 0o111) != 0 {
                            name.push('*');
                        } else if (st.st_mode & libc::S_IFMT) == libc::S_IFSOCK {
                            name.push('=');
                        }
                    }
                }
                print_long_format(&full_path, &name);
            }
        } else {
            for name in entries {
                let full_path = format!("{}/{}", path, name);

                let mut display_name = name.clone();
                let mut st: stat = std::mem::zeroed();
                let c_full = CString::new(full_path.clone()).unwrap();

                if f_flag {
                    if lstat(c_full.as_ptr(), &mut st) == 0 {
                        if (st.st_mode & libc::S_IFMT) == libc::S_IFLNK {
                            display_name.push('@');
                        } else if (st.st_mode & libc::S_IFMT) == libc::S_IFDIR {
                            display_name.push('/');
                        } else if (st.st_mode & 0o111) != 0 {
                            display_name.push('*');
                        }
                    }
                }

                if lstat(c_full.as_ptr(), &mut st) == 0 {
                    if (st.st_mode & libc::S_IFMT) == libc::S_IFLNK {
                        print!("\x1b[36m{}\x1b[0m  ", display_name);
                    } else if (st.st_mode & libc::S_IFMT) == libc::S_IFDIR {
                        print!("\x1b[34m{}\x1b[0m  ", display_name);
                    } else if (st.st_mode & (libc::S_IXUSR | libc::S_IXGRP | libc::S_IXOTH)) != 0 {
                        print!("\x1b[32m{}\x1b[0m  ", display_name);
                    } else {
                        print!("{}  ", display_name);
                    }
                } else {
                    print!("{}  ", display_name);
                }
            }
            println!();
        }
    }
}

fn print_long_format(path: &str, name: &str) {
    unsafe {
        let mut st: stat = std::mem::zeroed();
        let c_path = CString::new(path).unwrap();

        if libc::lstat(c_path.as_ptr(), &mut st) != 0 {
            return;
        }

        let file_type = match st.st_mode & libc::S_IFMT {
            libc::S_IFREG => '-',
            libc::S_IFDIR => 'd',
            libc::S_IFLNK => 'l',
            libc::S_IFCHR => 'c',
            libc::S_IFBLK => 'b',
            libc::S_IFIFO => 'p',
            libc::S_IFSOCK => 's',
            _ => '?',
        };

        let permissions = format_permissions(st.st_mode);
        
        // Size or device numbers
        let size_or_dev = if file_type == 'c' || file_type == 'b' {
            let maj = major(st.st_rdev);
            let min = minor(st.st_rdev);
            format!("{}, {}", maj, min)
        } else {
            st.st_size.to_string()
        };

        let mtime = st.st_mtime as i64;
        let datetime = format_time(mtime);
        let username = uid_to_username(st.st_uid);
        let groupname = gid_to_groupname(st.st_gid);
        let nlink = st.st_nlink;
        let extended_attrs = if has_extended_attrs(path) { "+" } else { "" };

        println!(
            "{}{}{} {} {} {} {:>8} {} {}",
            file_type, permissions, extended_attrs, nlink, username, groupname, size_or_dev, datetime, name
        );
    }
}

fn has_extended_attrs(path: &str) -> bool {
    use libc::listxattr;
    use std::ffi::CString;

    let c_path = CString::new(path).unwrap();
    unsafe {
        let size = listxattr(c_path.as_ptr(), std::ptr::null_mut(), 0);
        size > 0
    }
}

pub fn mkdir(args: &[String]) {
    if args.is_empty() {
        println!("mkdir: missing arguments");
        return;
    }

    for dir in args {
        if Path::new(dir).exists() {
            println!("mkdir: cannot creat directory '{}': Already exists", dir);
        } else if let Err(_) = fs::create_dir(dir) {
            println!("mkdir: cannot creat directory '{}': Permission denied", dir);
        }
    }
}

pub fn rm(args: &[String]) {
    if args.len() == 0 {
        println!("rm: Missing arguments");
        return;
    }

    let mut r_flag = false;
    let mut files = Vec::new();

    for arg in args {
        if arg == "-r" || arg == "-R" {
            r_flag = true;
        } else {
            files.push(arg);
        }
    }

    for file in files {
        let path = Path::new(file);

        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", file);
            continue;
        }

        if path.is_dir() {
            if r_flag {
                if let Err(_) = fs::remove_dir_all(path) {
                    println!("rm: cannot remove '{}': Directory not empty", file);
                }
            } else {
                println!("rm: cannot remove '{}': Is a directory", file);
            }
        } else {
            if let Err(_) = fs::remove_file(path) {
                println!("rm: cannot remove '{}': Permission Denied", file);
            }
        }
    }
}

pub fn touch(args: &[String]) {
    if args.is_empty() {
        println!("touch: missing file");
        return;
    }

    for filename in args {
        let path = Path::new(filename);

        if path.exists() {
            continue;
        } else {
            match fs::File::create(path) {
                Ok(_) => {}
                Err(_) => {
                    println!("touch: cannot touch '{}' Permission Denied", filename);
                }
            }
        }
    }
}

pub fn clear(_args: &[String]) {
    print!("\x1b[2J\x1b[H\x1b[3J"); // 2J: clear shown screen / H: cursor to top-left / 3J: clear hidden
    io::stdout().flush().unwrap();
}

pub fn cp(args: &[String]) {
    if args.len() != 2 {
        println!("cp: usage: cp <source> <destination>");
        return;
    }

    let src = Path::new(&args[0]);
    let dest = Path::new(&args[1]);

    if src.is_dir() {
        println!("cp: '{}' is a directory", src.display());
        return;
    }

    let dest = if dest.is_dir() {
        let mut dest_path = PathBuf::from(dest);
        dest_path.push(src.file_name().unwrap());
        dest_path
    } else {
        PathBuf::from(dest)
    };

    match fs::copy(src, &dest) {
        Ok(_) => {}
        Err(e) => println!("cp: error copying file: {}", e),
    }
}

pub fn mv(args: &[String]) {
    if args.len() != 2 {
        println!("mv: usage: mv <src> <dest>");
        return;
    }

    let src = Path::new(&args[0]);
    let dest = Path::new(&args[1]);

    let dest = if dest.is_dir() {
        let mut dest_path = PathBuf::from(dest);
        dest_path.push(src.file_name().unwrap());
        dest_path
    } else {
        PathBuf::from(dest)
    };

    match std::fs::rename(src, &dest) {
        Ok(_) => {}
        Err(e) => println!("mv: error moving file: {}", e),
    }
}

pub fn cat(args: &[String]) {
    if args.is_empty() {
        // No arguments → behave like shell `cat`, read from stdin
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = String::new();

        while let Ok(n) = handle.read_line(&mut buffer) {
            if n == 0 {
                break; // EOF
            }
            print!("{}", buffer);
            buffer.clear();
        }
        return;
    }

    for filename in args {
        let file = File::open(filename);
        match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    match line {
                        Ok(content) => println!("{}", content),
                        Err(e) => {
                            eprintln!("cat: error reading from '{}': {}", filename, e);
                            break;
                        }
                    }
                }
            }
            Err(_) => {
                eprintln!("cat: {}: No such file or directory", filename);
            }
        }
    }
}

pub fn help(_args: &[String]) {
    println!("Built-in commands:");
    println!("  echo [text...]     - Display text");
    println!("  cd [directory]     - Change directory");
    println!("  ls [-a] [-l] [-F]  - List directory contents");
    println!("  pwd                - Print working directory");
    println!("  cat [file...]      - Display file contents");
    println!("  cp <src> <dist>    - Copy files");
    println!("  rm [-r] [file...]  - Remove files/directories");
    println!("  mv <src> <dist>    - Move/rename files");
    println!("  mkdir [dir...]     - Create directories");
    println!("  touch [file...]    - Create empty files or update timestamps");
    println!("  clear              - Clear the terminal screen");
    println!("  help               - Show this help message");
    println!("  exit               - Exit the shell");
    println!();
    println!("Features:");
    println!("  - Multi Commands with ';'");
    println!("  - Colorized output");
    println!("  - Current directory in prompt");
    println!("  - Ctrl+D to exit");
}

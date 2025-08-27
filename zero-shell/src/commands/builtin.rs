use std::ffi::CStr;
use std::time::SystemTime;
use std::os::unix::fs::PermissionsExt;
use std::{ env };
use std::fs::{ self, DirEntry };
use std::os::unix::fs::MetadataExt;
use crate::utils::human_readable;
use libc::{ getpwuid, getgrgid };
use chrono::{ DateTime, Local };

pub fn echo(args: &[String]) {
    if args.len() == 0 {
        println!();
        return;
    }

    let output = args.join(" ");

    let output = output.replace("\\n", "\n").replace("\\t", "\t");
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

    if args.len() == 0 {
        path = env::var("HOME").unwrap_or("~".to_string());
    } else {
        path = args[0].clone();
    }

    if let Err(_) = env::set_current_dir(&path) {
        println!("cd: -- {} -- No sucha file or dir", path);
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

    for path in paths {
        list_dir(&path, a_flag, l_flag, f_flag);
    }
}

fn list_dir(path: &str, a_flag: bool, l_flag: bool, f_flag: bool) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir, // dir here is a ReadDir which is an iterator over dir [files and dirs] called DirEntry which we used in the content Vec
        Err(_) => {
            println!("Error Listing the directory.");
            return;
        }
    };

    let mut content: Vec<DirEntry> = Vec::new();

    for file in dir {
        if let Ok(file_or_dir) = file {
            content.push(file_or_dir);
        }
    }
    // sort the content
    content.sort_by(|a, b| a.file_name().cmp(&b.file_name())); // Keep in mind that file_name returns an OsString not a normal String which not can be not UTF-8 and it's based on the system like in Unix could be arbitrary bytes since OS paths are bytes sequence, but in Windows it's stored as WTF-16 (Windows's Unicode encoding)

    if l_flag {
        // a block in Unix ls sense is 1024 bytes 1 KB
        let mut total_blocks = 0;
        for c in &content {
            if let Ok(metadata) = c.metadata() {
                total_blocks += metadata.len() / 1024; // metadata.len() is the file size in bytes
            }
        }
        println!("total {}", total_blocks);

        for c in &content {
            let file_name = c.file_name();
            let name = file_name.to_string_lossy();

            if !a_flag && name.starts_with('.') {
                continue;
            }

            print_long_format(c);
        }
    } else {
        for c in &content {
            let file_name = c.file_name();
            let name = file_name.to_string_lossy();

            if !a_flag && name.starts_with('.') {
                continue;
            }

            let mut display_name = name.to_string();

            if f_flag {
                if let Ok(metadata) = c.metadata() {
                    if metadata.is_dir() {
                        display_name.push('/');
                    } else if (metadata.permissions().mode() & 0o111) != 0 {
                        display_name.push('*');
                    }
                }
            }
            // need coloring for dir and executable files
            print!("{}  ", display_name);
        }
        println!();
    }
}

fn print_long_format(c: &DirEntry) {
    let metadata = match c.metadata() {
        Ok(meta) => meta,
        Err(_) => {
            return;
        }
    };

    let file_type = if metadata.is_dir() { 'd' } else { '-' };

    let permissions = format_permissions(metadata.permissions().mode());
    let size = metadata.len();
    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime = format_time(modified);
    let username = uid_to_username(metadata.uid());
    let groupname = gid_to_groupname(metadata.gid());
    let nlink = metadata.nlink();

    println!(
        "{}{} {} {} {} {:^10} {} {}", // root root need actually to be real user and group names using libc to get them later
        file_type,
        permissions,
        nlink,
        username,
        groupname,
        human_readable(size), // formated size from raw bytes to readable size [B, K, M, G]
        datetime,
        c.file_name().to_string_lossy()
    );
}

fn uid_to_username(uid: u32) -> String {
    unsafe {
        let passwd = getpwuid(uid);
        if passwd.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*passwd).pw_name);
        name.to_string_lossy().into_owned()
    }
}

fn gid_to_groupname(uid: u32) -> String {
    unsafe {
        let passwd = getgrgid(uid);
        if passwd.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*passwd).gr_name);
        name.to_string_lossy().into_owned()
    }
}

fn format_permissions(mode: u32) -> String {
    let mut perms = String::new();
    // let's get back to the first quest in Go pool XD:
    // mod => 0o755 for example the 0o stands for octale and the rest is permitions and 3 bits for each one [user, group, others] total of 9 bits
    // binary numbers : [0: 000, 1: 001, 2: 010, 3: 011, 4: 100, 5: 101, 6: 110, 7: 111]
    // means mod in binary is => [111  101  101]
    // soo we have :
    // read    = 0o400 = r
    // write   = 0o200 = w
    // execute = 0o100 = x
    // meams that :
    // we will have a rwxr-xr-x as a formated permissions

    // user
    perms.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o100) != 0 { 'x' } else { '-' });

    // group
    perms.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o010) != 0 { 'x' } else { '-' });

    // others
    perms.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
    perms.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
    perms.push(if (mode & 0o001) != 0 { 'x' } else { '-' });

    perms
}

// Done using chrono.
fn format_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into(); // convert to local time
    // soo the datetime looks like: 2025-08-27 16:19:23.606228703 +01:00 => using format!() we make it look like Feb 27 09:21
    datetime.format("%b %e %H:%M").to_string()
}

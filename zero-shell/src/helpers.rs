use libc::{ getpwuid, getgrgid };
use chrono::{ DateTime, Local };
use std::ffi::CStr;
use std::time::SystemTime;

// Done using chrono.
pub fn format_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into(); // convert to local time
    // soo the datetime looks like: 2025-08-27 16:19:23.606228703 +01:00 => using format!() we make it look like Feb 27 09:21
    datetime.format("%b %e %H:%M").to_string()
}

pub fn format_permissions(mode: u32) -> String {
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

pub fn uid_to_username(uid: u32) -> String {
    unsafe {
        let passwd = getpwuid(uid);
        if passwd.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*passwd).pw_name);
        name.to_string_lossy().into_owned()
    }
}

pub fn gid_to_groupname(uid: u32) -> String {
    unsafe {
        let passwd = getgrgid(uid);
        if passwd.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*passwd).gr_name);
        name.to_string_lossy().into_owned()
    }
}

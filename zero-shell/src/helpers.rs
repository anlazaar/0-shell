use chrono::{ DateTime, Utc };
use chrono_tz::Africa::Casablanca;
use std::ffi::CStr;
use std::ffi::CString;

pub fn blocks512_for_path(path: &str) -> Option<u64> {
    let cpath = CString::new(path).ok()?;
    let mut st: libc::stat = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::stat(cpath.as_ptr(), &mut st) };
    if rc == 0 {
        Some(st.st_blocks as u64)
    } else {
        None
    }
}

pub fn clean_input(input: &str) -> String {
    input
        .replace("\x1B[A", "")
        .replace("\x1B[B", "")
        .replace("\x1B[C", "")
        .replace("\x1B[D", "")
        .to_string()
}

// Done using chrono.
pub fn format_time(mtime: i64) -> String {
    // Build a UTC datetime from epoch seconds
    let dt_utc = DateTime::from_timestamp(mtime, 0).unwrap_or_else(||
        DateTime::<Utc>::from_timestamp(0, 0).unwrap()
    );

    // Convert to local time
    let local_dt = dt_utc.with_timezone(&Casablanca);

    // Format like "Feb 27 09:21"
    local_dt.format("%b %e %H:%M").to_string()
}

pub fn format_permissions(mode: u32) -> String {
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
    let mut perms = String::new();

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
        let pw = libc::getpwuid(uid);
        if pw.is_null() {
            return uid.to_string();
        }
        let name = CStr::from_ptr((*pw).pw_name);
        name.to_string_lossy().into_owned()
    }
}

pub fn gid_to_groupname(gid: u32) -> String {
    unsafe {
        let gr = libc::getgrgid(gid);
        if gr.is_null() {
            return gid.to_string();
        }
        let name = CStr::from_ptr((*gr).gr_name);
        name.to_string_lossy().into_owned()
    }
}

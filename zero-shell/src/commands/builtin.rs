use std::{ env };
use std::fs::{ self, DirEntry };

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

pub fn ls(_args: &[String]) {
    let mut paths = Vec::new(); // Will hold the dir to list [NOT IMPLIMENTED YET]

    if paths.len() == 0 {
        paths.push(".".to_string());
    }

    for path in paths {
        list_dir(&path);
    }
}

fn list_dir(path: &str) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            println!("Error Listing the directory.");
            return;
        }
    };

    let mut content: Vec<DirEntry> = Vec::new();

    for c in dir {
        if let Ok(file_or_dir) = c {
            content.push(file_or_dir);
        }
    }

    // sort the content
    content.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    println!("{:?}", content);
}

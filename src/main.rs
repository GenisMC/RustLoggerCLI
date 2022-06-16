use std::env;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::prelude::*;

struct Log {
    date: String,
    hours: String,
    category: String,
    path: String,
}

fn check_if_default_path(path: String) -> String {
    if path.contains("default") {
        return "S:/Code/Logs/logs.txt".to_string();
    }
    return path;
}

fn main() {
    let now = Utc::now();
    let date = now.format("%d-%m-%Y").to_string();
    let path;
    // get parameter from command line and check if it is valid
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: logs <hours> <category> <path>");
        return;
    }
    else if args[1] == "help" {
        println!("Usage: logs <hours> <category> <path>");
        return;
    }
    else if args[1] == "list" {
        // Read file lines and print them
        path = check_if_default_path(args[2].to_string());
        let mut file = OpenOptions::new()
            .read(true)
            .open(&path)
            .expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read file");
        println!("{}", contents);
        return;
    }
    let hours = args[1].to_string();
    let category = args[2].to_string();
    let path = check_if_default_path(args[3].to_string());

    // create log object
    let args = Log {
        date: date,
        hours: hours,
        category: category,
        path: path,
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(args.path)
        .unwrap();

    if let Err(e) = write!(file, "{} {} {} \n", args.date, args.hours, args.category) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

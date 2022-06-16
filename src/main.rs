use std::env;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::prelude::*;

// Basic record structure
struct Log {
    date: String,
    hours: String,
    category: String,
    path: String,
}

const VERSION:&str = "0.2.2";

// Display usage of the CLI
fn usage() {
    println!("Usage: log [OPTION]... [FILE]");
    println!("Log your hours");
    println!("");
    println!("-h, -help             display this help and exit");
    println!("-l, -list     [FILE]  list all logs in file");
    println!("-r, -remove   [FILE]  remove a log");
    println!("-v, -version  [FILE]  display version of the CLI");
    println!("-a, -add      [HOURS] [CATEGORY] [FILE]  add a new log");
}

// Checks if inserted path is default or not
fn check_if_default_path(path: String) -> String {
    if path.contains("default") {
        return "S:/Code/Logs/logs.txt".to_string();
    }
    return path;
}

// Removes last line from file
fn remove_last_line_of_file(path: String) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    let last_line = lines.next_back().unwrap();
    let mut new_contents = String::new();
    for line in lines {
        if line != last_line {
            new_contents.push_str(line);
            new_contents.push_str("\n");
        }
    }
    file.set_len(0).unwrap();
    file.write_all(new_contents.as_bytes()).unwrap();
}

fn main() {
    let date = Utc::now().format("%d-%m-%Y").to_string();
    // get parameter from command line and check if it is valid
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        if args[1] == "-h" || args[1] == "-help" {
            usage();
            return;
        } else if args[1] == "-l" || args[1] == "-list" {
            if args.len() == 3 {
                let path = check_if_default_path(args[2].to_string());
                let mut file = OpenOptions::new()
                    .read(true)
                    .open(&path)
                    .expect("Unable to open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Unable to read file");
                let lines = contents.lines();
                for line in lines {
                    println!("{}", line);
                }
            } else {
                println!("Please specify a file");
            }
        }
        else if args[1] == "-a" || args[1] == "-add" {
            if args.len() == 5 {
                let hours = args[2].to_string();
                let category = args[3].to_string();
                let path = check_if_default_path(args[4].to_string());

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
                    .expect("Unable to open file");

                if let Err(e) = write!(file, "{} {} {} \n", args.date, args.hours, args.category) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
            else {
                println!("Please specify hours, category and file");
            }
        }
        else if args[1] == "-r" || args[1] == "-remove" {
            if args.len() == 3 {
                let path = check_if_default_path(args[2].to_string());
                remove_last_line_of_file(path);
            }
            else {
                println!("Please specify a file");
            }
        }
        else if args[1] == "-v" || args[1] == "-version" {
            println!("logger {}", VERSION);
        }
        else {
            usage();
        }
    }
    else{
        usage();
    }
}

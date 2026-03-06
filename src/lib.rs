use std::io::stdin;
use std::{collections::HashMap, fs::File, io::Write, process};

pub enum Command {
    Insert(String, String),
    Remove(String),
    Select(String),
}

// args must looks like this : ["/target/...", "INSTRUCTION", "Target", "value" if the user put one]
pub fn parse_args(args: Vec<String>) -> Result<Command, &'static str> {
    let len_args = args.len();
    if len_args < 3 {
        return Err("Not enough arguments, usage : rkv [INSTRUCTION] [Target] value");
    }

    let target = args[2].clone();
    //args[1] is the instruction
    match args[1].as_str() {
        "INSERT" => match args.len() {
            4 => Ok(Command::Insert(target, args[3].clone())),
            _ => Err("Not a valid number of arguments, Usage: INSERT key value"),
        },

        "REMOVE" => match args.len() {
            3 => Ok(Command::Remove(target)),
            _ => Err("Not a valid number of arguments, Usage: REMOVE key"),
        },

        "SELECT" => match args.len() {
            3 => Ok(Command::Select(target)),
            _ => Err("Not a valid number of arguments, Usage: SELECT key/operator"),
        },

        _ => {
            return Err("Not a valid instruction hint:(maybe the instruction is not in caps)");
        }
    }
}

pub fn database_to_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let content = match std::fs::read_to_string("database.txt") {
        Ok(txt) => txt,
        Err(_) => return map,
    };

    for line in content.lines() {
        let mut temp = line.split_whitespace();

        if let Some(key) = temp.next()
            && let Some(value) = temp.next()
        {
            map.insert(key.to_string(), value.to_string());
        }
    }

    return map;
}

pub fn leave_database(map: HashMap<String, String>) {
    /*In this function we will retrive all the informations into the database.txt file, we will erase the old one and write on top with
     * the hasmap we have.
     */

    let mut db = File::create("database.txt").expect("Error creating the database.txt file");

    for (key, value) in map {
        writeln!(db, "{key} {value}").expect("Error writing in the database.txt file");
    }

    process::exit(0);
}

pub fn insert(mut map: HashMap<String, String>, key: &str, value: &str) -> HashMap<String, String> {
    if map.contains_key(key) {
        println!(
            "Your database already contains this key and can only be in it once. Are you sure you want to overide it ? (y/n)"
        );

        let mut answer = String::new();
        stdin()
            .read_line(&mut answer)
            .expect("Error while reading the user input.");

        let answer: String = answer.trim().to_lowercase();

        if !answer.eq("y") {
            process::exit(1);
        }
    }

    map.insert(key.to_string(), value.to_string());

    return map;
}

pub fn remove(mut map: HashMap<String, String>, key: &str) -> HashMap<String, String> {
    map.remove(key);

    return map;
}

pub fn select(map: HashMap<String, String>, target: &str) -> HashMap<String, String> {
    if target.eq("*") {
        for (key, value) in &map {
            println!("{key} {value}");
        }
    } else {
        for (key, value) in &map {
            if key.eq(target) {
                println!("{key} {value}");
            }
        }
    }

    return map;
}

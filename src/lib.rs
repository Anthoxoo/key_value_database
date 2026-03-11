use std::io::stdin;
use std::{collections::HashMap, fs::File, io::Write, process};

/*This enum will contains the arguments used for the differents functions with the same names as the queries in the enum */
pub enum Command {
    Insert(String, String),
    Remove(String),
    Select(String),
    Drop(),
}

pub fn parse_args(args: Vec<String>) -> Result<Command, &'static str> {
    /* This function parse the arguments given into the Command enum, every checking of len is +1 compared to what
     * it is supposed to be because the first argument will always be the path of the executable.
     * args must looks like this : ["/target/...", "INSTRUCTION", "Target" if the user put one , "value" if the user put one] */

    let len_args = args.len();

    if len_args < 2 {
        return Err("Not enough arguments, usage : rkv [INSTRUCTION] target value");
    }

    match args[1].as_str() {
        "INSERT" => match len_args {
            4 => {
                let target = args[2].clone();
                Ok(Command::Insert(target, args[3].clone()))
            }
            _ => Err("Not a valid number of arguments, Usage: INSERT key value"),
        },

        "REMOVE" => match len_args {
            3 => {
                let target = args[2].clone();
                Ok(Command::Remove(target))
            }
            _ => Err("Not a valid number of arguments, Usage: REMOVE key"),
        },

        "SELECT" => match len_args {
            3 => {
                let target = args[2].clone();
                Ok(Command::Select(target))
            }
            _ => Err("Not a valid number of arguments, Usage: SELECT key/operator"),
        },

        "DROP" => match len_args {
            2 => Ok(Command::Drop()),
            _ => Err("Not a valid number of arguments, Usage: DROP"),
        },
        _ => Err("Not a valid instruction hint:(maybe the instruction is not in caps)"),
    }
}

pub fn database_to_map() -> HashMap<String, String> {
    /*This function read the database.txt file and transpose it into a Hashmap called map. */

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

    map
}

pub fn leave_database(map: HashMap<String, String>) {
    /*In this function we will retrive all the informations into the database.txt file, we will erase the old one and write on top with
     * the hashmap we have.
     */

    let mut db = File::create("database.txt").expect("Error creating the database.txt file");

    for (key, value) in map {
        writeln!(db, "{key} {value}").expect("Error writing in the database.txt file");
    }

    process::exit(0);
}

pub fn insert(map: &mut HashMap<String, String>, key: &str, value: &str) {
    /* This function insert the given value in the hashmap (=our current database) and checks if the key is already in it, if so we give a warning. */

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
}

pub fn remove(map: &mut HashMap<String, String>, key: &str) {
    /* This function removes the key given from the hashmap. */

    map.remove(key);
}

pub fn select(map: &HashMap<String, String>, target: &str) {
    /* This function prints the target value if it is in the hashmap, if the star argument (*) is given we print everythings.
     * To use the star argument you must put a backslash (\*) behind so the shell does not intrepret it as his metacaracter.
     */

    if target.eq("*") {
        for (key, value) in map {
            println!("{key} {value}");
        }
    } else {
        for (key, value) in map {
            if key.eq(target) {
                println!("{key} {value}");
            }
        }
    }
}

pub fn drop_database(map: &mut HashMap<String, String>) {
    println!("Are you sure you want to delete the database content ? (y/n)");

    let mut answer = String::new();

    stdin()
        .read_line(&mut answer)
        .expect("Error while reading the user input.");

    let answer: String = answer.trim().to_lowercase();

    if !answer.eq("y") {
        process::exit(1);
    }

    map.clear()
}

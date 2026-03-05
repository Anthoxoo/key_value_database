use std::collections::HashMap;

pub enum Command {
    Insert(String, String),
    Remove(String),
    Select(),
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
            _ => Err("Not enough arguments, you need a target AND a value for INSERT."),
        },
        "REMOVE" => Ok(Command::Remove(target)),

        "SELECT" => Ok(Command::Select()),

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

    map
}

pub fn leave_database(map: HashMap<String, String>) {
    /*In this function we will retrive all the informations into the database.txt file, we will erase the old one and write on top with
     * the hasmap we have.
     */
}

pub fn insert(
    mut map: &HashMap<String, String>,
    target: &str,
    value: &str,
) -> HashMap<String, String> {
    todo!("Send a warning message because we can only have the same key once in the map");
    unimplemented!()
}

pub fn remove(mut map: &HashMap<String, String>, target: &str) -> HashMap<String, String> {
    unimplemented!()
}

pub fn select(map: &HashMap<String, String>) -> HashMap<String, String> {
    unimplemented!()
}

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
            return Err(
                "Not a valid instruction, rkv --help, (maybe the instruction is not in caps)",
            );
        }
    }
}

pub fn insert(target: &str, value: &str) {
    unimplemented!()
}

pub fn remove(target: &str) {
    unimplemented!()
}

pub fn select() {
    unimplemented!()
}

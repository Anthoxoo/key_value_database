pub enum Instruction {
    Insert,
    Remove,
    Select,
}

pub struct Command {
    pub instruction: Instruction,
    pub target: String,
    pub value: Option<String>,
}

// args must looks like this : ["/target/...", "INSTRUCTION", "value" if the user put one]
pub fn parse_args(args: Vec<String>) -> Result<Command, &'static str> {
    if args.len() < 3 {
        return Err("Not enough arguments, usage : rkv [INSTRUCTION] [Target] value");
    }

    let config: Instruction = match args[1].as_str() {
        "INSERT" => Instruction::Insert,
        "REMOVE" => Instruction::Remove,
        "SELECT" => Instruction::Select,
        _ => {
            return Err(
                "Not a valid instruction, rkv --help, (maybe the instruction is not in caps)",
            );
        }
    };

    Ok(Command {
        instruction: config,
        target: args[2].clone(),
        value: Some(args[3].clone()),
    })
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

use key_value_database::{Command, insert, parse_args, remove, select};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match parse_args(args) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            std::process::exit(1);
        }
    };

    match command {
        Command::Insert(target, value) => insert(&target, &value),
        Command::Remove(target) => remove(&target),
        Command::Select() => select(),
    }
}

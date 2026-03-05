use key_value_database::{
    Command, database_to_map, insert, leave_database, parse_args, remove, select,
};
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

    let mut map = database_to_map();

    map = match command {
        Command::Insert(target, value) => insert(&map, &target, &value),
        Command::Remove(target) => remove(&map, &target),
        Command::Select() => select(&map),
    };
    leave_database(map);
}

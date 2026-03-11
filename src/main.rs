use key_value_database::{
    Command, database_to_map, drop_database, insert, leave_database, parse_args, remove, select,
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

    let mut map = database_to_map(); // opening database

    match command {
        Command::Insert(key, value) => insert(&mut map, &key, &value),
        Command::Remove(key) => remove(&mut map, &key),
        Command::Select(target) => select(&map, &target),
        Command::Drop() => drop_database(&mut map),
    };

    leave_database(map); // closing database
}

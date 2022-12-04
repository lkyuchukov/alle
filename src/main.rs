use clap::{arg, Command};
use rocksdb::{IteratorMode, DB};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TODO {
    status: Status,
    notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    InProgress,
    Done,
}

fn main() {
    let path = "/Users/lyubokyuchukov";
    let db = DB::open_default(path).unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let res = db.get(key).unwrap();
            match res.is_some() {
                true => println!("TODO with name {} already exists", key),
                false => {
                    fun_name(&db, key, sub_matches);
                }
            }
        }
        Some(("list", _)) => {
            let iter = db.iterator(IteratorMode::Start);
            for item in iter {
                let (key, value) = item.unwrap();
                println!(
                    "{}: {}",
                    String::from_utf8_lossy(&key),
                    String::from_utf8_lossy(&value)
                );
            }
        }

        _ => unreachable!(),
    }
}

fn fun_name(db: &DB, key: &String, sub_matches: &clap::ArgMatches) {
    let todo = TODO {
        status: Status::InProgress,
        notes: String::from("no notes for now"),
    };
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();
    println!(
        "Adding {}",
        sub_matches.get_one::<String>("NAME").expect("required")
    );
}

fn cli() -> Command {
    Command::new("todo")
        .version("0.1")
        .about("Terminal TODO manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("list").about("List all TODOs"))
}

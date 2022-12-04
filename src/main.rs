use clap::{arg, Command};
use rocksdb::{IteratorMode, DB};
use todo_rust::{add_todo, complete_todo, delete_todo};

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
                    add_todo(&db, key);
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
        Some(("complete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let res = db.get(key).unwrap();
            match res.is_some() {
                true => {
                    let val = String::from_utf8(res.unwrap()).unwrap();
                    complete_todo(&db, &key, &val);
                }
                false => {
                    println!("No TODO with name {}", key)
                }
            }
        }
        Some(("delete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let res = db.get(key).unwrap();
            match res.is_some() {
                true => {
                    delete_todo(&db, &key);
                }
                false => {
                    println!("No TODO with name {}", key)
                }
            }
        }
        _ => unreachable!(),
    }
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
        .subcommand(
            Command::new("complete")
                .about("Complete a TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true),
        )
}

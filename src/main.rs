use clap::{arg, Command};
use rocksdb::{DB};
use todo_rust::{add_todo, complete_todo, delete_todo, drop_db, get_all_todos};

fn main() {
    let path = "/Users/lyubokyuchukov/todo-rust";
    let db = DB::open_default(path).unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = add_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("list", _)) => {
            let todos = get_all_todos(&db);
            for todo in todos {
                println!("{:?}", todo);
            }
        }
        Some(("complete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = complete_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("delete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");

            if let Err(e) = delete_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("drop-db", _)) => {
            if let Err(e) = drop_db(path) {
                println!("{}", e);
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
        .subcommand(Command::new("drop-db").about("Drops the database of TODOs"))
}

use clap::{arg, Command};
use rocksdb::DB;
use tman::{
    add_todo, add_todo_note, add_todo_tag, complete_todo, delete_todo, drop_db, get_all_todos,
    remove_todo_tag, uncomplete_todo, edit_todo_note, remove_todo_note,
};

fn main() {
    let binding = dirs::home_dir().unwrap();
    let path = binding.to_str().unwrap().to_string() + "/.tman";
    let db = DB::open_default(&path).unwrap();

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
        Some(("uncomplete", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = uncomplete_todo(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("add-note", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let note = sub_matches.get_one::<String>("NOTE").expect("required");
            if let Err(e) = add_todo_note(&db, &key, &note) {
                println!("{}", e);
            }
        }
        Some(("edit-note", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let new_note = sub_matches.get_one::<String>("NOTE").expect("required");
            if let Err(e) = edit_todo_note(&db, &key, &new_note) {
                println!("{}", e);
            }
        }
        Some(("remove-note", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            if let Err(e) = remove_todo_note(&db, &key) {
                println!("{}", e);
            }
        }
        Some(("add-tag", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let tag = sub_matches.get_one::<String>("TAG").expect("required");
            if let Err(e) = add_todo_tag(&db, &key, &tag) {
                println!("{}", e);
            }
        }
        Some(("remove-tag", sub_matches)) => {
            let key = sub_matches.get_one::<String>("NAME").expect("required");
            let tag = sub_matches.get_one::<String>("TAG").expect("required");
            if let Err(e) = remove_todo_tag(&db, &key, &tag) {
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
            if let Err(e) = drop_db(&path) {
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
            Command::new("uncomplete")
                .about("Uncomplete a TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add-note")
                .about("Add a note for a given TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true)
                .arg(arg!(<NOTE> "The note to add"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("edit-note")
                .about("Edit the note for a given TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("remove-note")
                .about("Remove the note for a given TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("add-tag")
                .about("Add a tag to a given TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true)
                .arg(arg!(<TAG> "The tag to add"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove-tag")
                .about("Remove a tag from a given TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true)
                .arg(arg!(<TAG> "The tag to remove"))
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

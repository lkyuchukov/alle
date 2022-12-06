use clap::{Command, arg};

pub fn cli() -> Command {
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
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove-note")
                .about("Remove the note for a given TODO")
                .arg(arg!(<NAME> "The name of the todo"))
                .arg_required_else_help(true),
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

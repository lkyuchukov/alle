# alle
A terminal manager for your TODOs written in Rust.

# Usage
```
Usage: alle <COMMAND>
Commands:
  add              Add a TODO
  list             List all TODOs
  complete         Complete a TODO
  uncomplete       Uncomplete a TODO
  add-note         Add a note for a given TODO
  edit-note        Edit the note for a given TODO
  remove-note      Remove the note for a given TODO
  add-tag          Add a tag to a given TODO
  remove-tag       Remove a tag from a given TODO
  add-due-date     Add a due date to a given TODO
  change-due-date  Change the due date for a given TODO
  remove-due-date  Remove the due date from a given TODO
  delete           Delete a TODO
  drop-db          Drops the database of TODOs
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

```

```
Usage: alle add [OPTIONS] <NAME>

Arguments:
  <NAME>  The name of the todo

Options:
  -n <NOTE>
  -d <DUE_DATE>
  -h, --help         Print help information

```
```
Usage: alle list [OPTIONS]

Options:
  -s <STATUS>
  -t <TAG>
  -h, --help       Print help information
```

# Installation

With cargo:
```
cargo install alle
```

With homebrew:
```
brew tap lkyuchukov/alle
brew install alle
```
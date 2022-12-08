use std::{
    fs,
    str::{from_utf8, FromStr},
};

use chrono::NaiveDate;
use rocksdb::{IteratorMode, Options, DB};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
    pub status: Status,
    pub due_date: String,
    pub note: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Status {
    ToDo,
    Done,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::ToDo => String::from("To Do"),
            Status::Done => String::from("Done"),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ToDo" => Ok(Status::ToDo),
            "Done" => Ok(Status::Done),
            _ => Err(format!("{} is not a valid status", s)),
        }
    }
}

pub fn add_todo(db: &DB, key: &String) -> Result<(), &'static str> {
    let res = db.get(key).unwrap();
    if res.is_some() {
        return Err("Todo with this name already exists");
    }

    let todo = Todo {
        name: key.to_string(),
        status: Status::ToDo,
        due_date: String::from(""),
        note: String::from(""),
        tags: Vec::new(),
    };
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn get_all_todos(db: &DB, status: Option<&String>, tag: Option<&String>) -> Vec<Todo> {
    let iter = db.iterator(IteratorMode::Start);
    let mut todos: Vec<Todo> = Vec::new();

    for item in iter {
        let (_, todo) = item.unwrap();
        let todo = from_utf8(&todo).unwrap();

        todos.push(serde_json::from_str(todo).unwrap());
    }

    if status.is_some() {
        let status = Status::from_str(status.unwrap()).unwrap();
        if status == Status::Done {
            todos.retain(|t| t.status == Status::Done)
        } else if status == Status::ToDo {
            todos.retain(|t| t.status == Status::ToDo)
        }
    }

    if tag.is_some() {
        let tag = tag.unwrap();
        todos.retain(|t| t.tags.contains(&tag.to_string()));
    }

    todos
}

pub fn complete_todo(db: &DB, key: &String) -> Result<(), &'static str> {
    let res = db.get(key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }
    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    todo.status = Status::Done;
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn uncomplete_todo(db: &DB, key: &String) -> Result<(), &'static str> {
    let res = db.get(key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }
    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    todo.status = Status::ToDo;
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn add_todo_note(db: &DB, key: &String, note: &String) -> Result<(), &'static str> {
    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }
    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    if !todo.note.is_empty() {
        return Err("This todo already has a note");
    }
    todo.note = note.to_string();
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn edit_todo_note(db: &DB, key: &String, new_note: &String) -> Result<(), &'static str> {
    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }
    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    todo.note = new_note.to_string();
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn remove_todo_note(db: &DB, key: &String) -> Result<(), &'static str> {
    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }
    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    todo.note = String::from("");
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn add_todo_tag(db: &DB, key: &String, tag: &String) -> Result<(), &'static str> {
    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }
    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();

    if todo.tags.contains(tag) {
        return Err("This tag is has already been added to this todo");
    }

    todo.tags.push(tag.to_string());
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn remove_todo_tag(db: &DB, key: &String, tag: &String) -> Result<(), &'static str> {
    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }
    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    if !todo.tags.contains(tag) {
        return Err("This tag does not exist for this todo");
    }

    todo.tags.retain(|t| t != tag);
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn add_due_date(db: &DB, key: &String, date: &String) -> Result<(), &'static str> {
    let date = NaiveDate::parse_from_str(date, "%d-%m-%Y");
    if date.is_err() {
        return Err("Invalid date format");
    }

    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }

    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    todo.due_date = date.unwrap().format("%d-%m-%Y").to_string();
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn change_due_date(db: &DB, key: &String, new_date: &String) -> Result<(), &'static str> {
    let date = NaiveDate::parse_from_str(new_date, "%d-%m-%Y");
    if date.is_err() {
        return Err("Invalid date format");
    }

    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }

    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    todo.due_date = date.unwrap().format("%d-%m-%Y").to_string();
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn remove_due_date(db: &DB, key: &String) -> Result<(), &'static str> {
    let res = db.get(&key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }

    let val = String::from_utf8(res.unwrap()).unwrap();

    let mut todo: Todo = serde_json::from_str(&val).unwrap();
    todo.due_date = String::from("");
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn delete_todo(db: &DB, key: &String) -> Result<(), &'static str> {
    let res = db.get(key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }

    db.delete(&key).unwrap();

    Ok(())
}

pub fn drop_db(path: &str) -> std::io::Result<()> {
    let _ = DB::destroy(&Options::default(), path);

    fs::remove_dir_all(path)?;

    Ok(())
}

use std::{fs, str::from_utf8};

use rocksdb::{IteratorMode, Options, DB};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub name: String,
    pub status: Status,
    pub note: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    InProgress,
    Done,
}

pub fn add_todo(db: &DB, key: &String) -> Result<(), &'static str> {
    let res = db.get(key).unwrap();
    if res.is_some() {
        return Err("Todo with this name already exists");
    }

    let todo = Todo {
        name: key.to_string(),
        status: Status::InProgress,
        note: String::from(""),
        tags: Vec::new(),
    };
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();

    Ok(())
}

pub fn get_all_todos(db: &DB) -> Vec<Todo> {
    let iter = db.iterator(IteratorMode::Start);
    let mut todos = Vec::new();
    for item in iter {
        let (_, todo) = item.unwrap();
        let todo = from_utf8(&todo).unwrap();

        todos.push(serde_json::from_str(todo).unwrap());
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
    todo.status = Status::InProgress;
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

use std::{str::from_utf8, };

use rocksdb::{IteratorMode, DB};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub status: Status,
    pub notes: String,
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
        status: Status::InProgress,
        notes: String::from("no notes for now"),
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

pub fn complete_todo(db: &DB, key: &String) -> Result<(), &'static str>{
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

pub fn delete_todo(db: &DB, key: &String) -> Result<(), &'static str>{
    let res = db.get(key).unwrap();
    if res.is_none() {
        return Err("Todo with this name does not exist");
    }

    db.delete(&key).unwrap();

    Ok(())
}

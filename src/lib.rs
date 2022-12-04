use rocksdb::DB;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    status: Status,
    notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    InProgress,
    Done,
}

pub fn add_todo(db: &DB, key: &String) {
    let todo = Todo {
        status: Status::InProgress,
        notes: String::from("no notes for now"),
    };
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();
}

pub fn complete_todo(db: &DB, key: &String, s: &String) {
    let mut todo: Todo = serde_json::from_str(s).unwrap();
    todo.status = Status::Done;
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(key, serialized).unwrap();
}

pub fn delete_todo(db: &DB, key: &String) {
    db.delete(&key).unwrap();
}

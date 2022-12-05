use rocksdb::{Options, DB};
use tman::{add_todo, complete_todo, delete_todo, get_all_todos, Status, Todo, uncomplete_todo, add_todo_note};
use serial_test::serial;

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let result = add_todo(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();
        matches!(todo.status, Status::InProgress);
        assert_eq!(todo.name, key.to_string());
        assert_eq!(todo.notes, String::from("no notes for now"));
    }

    let _ = DB::destroy(&Options::default(), path);

    // generate some good names for a terminal todo app
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo_already_exists() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        insert_todo(&db, &key, Status::InProgress, &String::from("no notes this time"));

        let result = add_todo(&db, &key);
        assert_eq!(false, result.is_ok());
        assert_eq!(result.err().unwrap(), "Todo with this name already exists");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_get_all_todos() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key1 = String::from("foo");
        let notes1 = String::from("random notes");
        insert_todo(&db, &key1, Status::InProgress, &notes1);

        let key2 = String::from("bar");
        let notes2 = String::from("random notes again");
        insert_todo(&db, &key2, Status::Done, &notes2);

        let todos = get_all_todos(&db);
        assert_eq!(2, todos.len());

        let todo1 = todos.get(1).unwrap();
        matches!(todo1.status, Status::Done);
        assert_eq!(todo1.name, key1.to_string());
        assert_eq!(todo1.notes, notes1);

        let todo2 = todos.get(0).unwrap();
        assert_eq!(todo2.name, key2.to_string());
        matches!(todo2.status, Status::InProgress);
        assert_eq!(todo2.notes, notes2)
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_complete_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("whatever");
        insert_todo(&db, &key, Status::InProgress, &notes);

        let result = complete_todo(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::InProgress);
        assert_eq!(todo.notes, String::from("whatever"));
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_complete_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");

        let result = complete_todo(&db, &key);
        assert_eq!(false, result.is_ok());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_uncomplete_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("whatever");
        insert_todo(&db, &key, Status::Done, &notes);

        let result = uncomplete_todo(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::InProgress);
        assert_eq!(todo.notes, String::from("whatever"));
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_uncomplete_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");

        let result = uncomplete_todo(&db, &key);
        assert_eq!(false, result.is_ok());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_todo_note() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        insert_todo(&db, &key, Status::Done, &notes);

        let new_note = String::from("new note");
        let result = add_todo_note(&db, &key, &new_note);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::InProgress);
        assert_eq!(todo.notes, String::from("new note"));
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_todo_note_with_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note = String::from("random note");

        let result = add_todo_note(&db, &key, &note);
        assert_eq!(false, result.is_ok());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_delete_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("whatever");
        insert_todo(&db, &key, Status::InProgress, &notes);

        let result = delete_todo(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = db.get(key).unwrap();
        assert_eq!(true, db_value.is_none());
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_delete_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");

        let result = delete_todo(&db, &key);
        assert_eq!(false, result.is_ok());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

fn insert_todo(db: &DB, key: &String, status: Status, notes: &String) {
    let todo1 = Todo {
        name: key.to_string(),
        status,
        notes: notes.to_string(),
    };
    let serialized = serde_json::to_string(&todo1).unwrap();
    db.put(&key, serialized).unwrap();
}

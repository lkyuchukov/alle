use rocksdb::{Options, DB};
use serial_test::serial;
use tman::todo;
use todo::{
    add_todo, add_todo_note, add_todo_tag, complete_todo, delete_todo, edit_todo_note,
    get_all_todos, remove_todo_note, remove_todo_tag, uncomplete_todo, Status, Todo,
};

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
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.name, key.to_string());
        assert_eq!(todo.note, String::from(""));
        assert_eq!(0, todo.tags.len())
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo_already_exists() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let tags = Vec::new();
        insert_todo(
            &db,
            &key,
            Status::ToDo,
            &String::from("no notes this time"),
            &tags,
        );

        let result = add_todo(&db, &key);
        assert_eq!(true, result.is_err());
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
        let note1 = String::from("random notes");
        let mut tags1: Vec<String> = Vec::new();
        tags1.push(String::from("another tag"));
        insert_todo(&db, &key1, Status::ToDo, &note1, &tags1);

        let key2 = String::from("bar");
        let note2 = String::from("random notes again");
        let mut tags2: Vec<String> = Vec::new();
        tags2.push(String::from("random tag"));
        insert_todo(&db, &key2, Status::Done, &note2, &tags2);

        let todos = get_all_todos(&db);
        assert_eq!(2, todos.len());

        let todo1 = todos.get(1).unwrap();
        matches!(todo1.status, Status::Done);
        assert_eq!(todo1.name, key1.to_string());
        assert_eq!(todo1.note, note1);
        assert_eq!(todo1.tags, tags1);

        let todo2 = todos.get(0).unwrap();
        assert_eq!(todo2.name, key2.to_string());
        matches!(todo2.status, Status::ToDo);
        assert_eq!(todo2.tags, tags2)
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
        let note = String::from("whatever");
        let tags = Vec::new();
        insert_todo(&db, &key, Status::ToDo, &note, &tags);

        let result = complete_todo(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from("whatever"));
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
        assert_eq!(true, result.is_err());
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
        let note = String::from("whatever");
        let tags = Vec::new();
        insert_todo(&db, &key, Status::Done, &note, &tags);

        let result = uncomplete_todo(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from("whatever"));
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
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo_note() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note = String::from("");
        let tags = Vec::new();
        insert_todo(&db, &key, Status::Done, &note, &tags);

        let new_note = String::from("new note");
        let result = add_todo_note(&db, &key, &new_note);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from("new note"));
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo_note_with_existing_note() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note = String::from("random note");
        let tags = Vec::new();
        insert_todo(&db, &key, Status::Done, &note, &tags);

        let result = add_todo_note(&db, &key, &note);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "This todo already has a note");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo_note_with_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note = String::from("random note");

        let result = add_todo_note(&db, &key, &note);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_edit_todo_note() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note = String::from("original note");
        let tags = Vec::new();
        insert_todo(&db, &key, Status::Done, &note, &tags);

        let new_note = String::from("new note");
        let result = edit_todo_note(&db, &key, &new_note);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from("new note"));
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_edit_todo_note_with_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let new_note = String::from("random note");

        let result = edit_todo_note(&db, &key, &new_note);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_remove_todo_note() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note = String::from("original note");
        let tags = Vec::new();
        insert_todo(&db, &key, Status::Done, &note, &tags);

        let result = remove_todo_note(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from(""));
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_remove_todo_note_with_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");

        let result = remove_todo_note(&db, &key);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_tag_to_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let tags = Vec::new();
        let tag = String::from("random tag");
        insert_todo(&db, &key, Status::Done, &notes, &tags);

        let result = add_todo_tag(&db, &key, &tag);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from(""));
        assert_eq!(1, todo.tags.len());
        assert_eq!(&tag, todo.tags.get(0).unwrap())
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_tag_to_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let tag = String::from("random tag");

        let result = add_todo_tag(&db, &key, &tag);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_tag_that_already_exists_to_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let mut tags = Vec::new();
        let tag = String::from("randomt tag");
        tags.push(tag.to_string());
        insert_todo(&db, &key, Status::Done, &notes, &tags);

        let result = add_todo_tag(&db, &key, &tag);
        assert_eq!(true, result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "This tag is has already been added to this todo"
        );
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_remove_todo_tag() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let mut tags = Vec::new();
        let tag = String::from("random tag");
        tags.push(tag.to_string());
        insert_todo(&db, &key, Status::Done, &notes, &tags);

        let result = remove_todo_tag(&db, &key, &tag);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from(""));
        assert_eq!(0, todo.tags.len());
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_remove_todo_tag_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let tag = String::from("random tag");

        let result = remove_todo_tag(&db, &key, &tag);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_remove_missing_todo_tag() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let mut tags = Vec::new();
        let tag = String::from("random tag");
        tags.push(tag.to_string());
        insert_todo(&db, &key, Status::Done, &notes, &tags);

        let missing_tag = String::from("missing tag");

        let result = remove_todo_tag(&db, &key, &missing_tag);
        assert_eq!(true, result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "This tag does not exist for this todo"
        );
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
        let tags = Vec::new();
        insert_todo(&db, &key, Status::ToDo, &notes, &tags);

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
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

fn insert_todo(db: &DB, key: &String, status: Status, notes: &String, tags: &Vec<String>) {
    let todo1 = Todo {
        name: key.to_string(),
        status,
        note: notes.to_string(),
        tags: tags.to_vec(),
    };
    let serialized = serde_json::to_string(&todo1).unwrap();
    db.put(&key, serialized).unwrap();
}

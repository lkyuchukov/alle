use rocksdb::{Options, DB};
use serial_test::serial;
use tman::todo::{self, add_due_date, change_due_date, remove_due_date};
use todo::{
    add_todo, add_todo_note, add_todo_tag, complete_todo, delete_todo, edit_todo_note,
    get_all_todos, remove_todo_note, remove_todo_tag, uncomplete_todo, Status, Todo,
};

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo_with_no_args() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note_arg: Option<&String> = None;
        let due_date_arg: Option<&String> = None;
        let result = add_todo(&db, &key, note_arg, due_date_arg);
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
fn test_add_todo_with_note_and_due_date_args() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note = String::from("whatever");
        let note_arg: Option<&String> = Some(&note);

        let due_date = String::from("17-07-2022");
        let due_date_arg: Option<&String> = Some(&due_date);
        let result = add_todo(&db, &key, note_arg, due_date_arg);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.name, key.to_string());
        assert_eq!(todo.note, note);
        assert_eq!(todo.due_date, due_date);
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
        let due_date = String::from("");
        insert_todo(
            &db,
            &key,
            Status::ToDo,
            &due_date,
            &String::from("no notes this time"),
            &tags,
        );

        let note_arg: Option<&String> = None;
        let due_date_arg: Option<&String> = None;
        let result = add_todo(&db, &key, note_arg, due_date_arg);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name already exists");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_todo_invalid_due_date() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let note_arg: Option<&String> = None;
        let due_date = String::from("17-07-222022");
        let due_date_arg: Option<&String> = Some(&due_date);
        let result = add_todo(&db, &key, note_arg, due_date_arg);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid date format");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_list_all_todos_with_no_flags() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key1 = String::from("foo");
        let note1 = String::from("random notes");
        let due_date = String::from("");
        let mut tags1: Vec<String> = Vec::new();
        tags1.push(String::from("another tag"));
        insert_todo(&db, &key1, Status::ToDo, &due_date, &note1, &tags1);

        let key2 = String::from("bar");
        let note2 = String::from("random notes again");
        let mut tags2: Vec<String> = Vec::new();
        tags2.push(String::from("random tag"));
        insert_todo(&db, &key2, Status::Done, &due_date, &note2, &tags2);

        // initialize status filter to be Option<&String>
        let status_filter: Option<&String> = None;
        let tag_filter: Option<&String> = None;
        let todos = get_all_todos(&db, status_filter, tag_filter);
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
fn test_list_all_todos_with_flags() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key1 = String::from("foo");
        let note1 = String::from("random notes");
        let due_date = String::from("");
        let mut tags1: Vec<String> = Vec::new();
        tags1.push(String::from("awesome"));
        insert_todo(&db, &key1, Status::ToDo, &due_date, &note1, &tags1);

        let key2 = String::from("bar");
        let note2 = String::from("random notes again");
        let mut tags2: Vec<String> = Vec::new();
        tags2.push(String::from("random tag"));
        insert_todo(&db, &key2, Status::Done, &due_date, &note2, &tags2);

        let todo_status = String::from("ToDo");
        let status_filter: Option<&String> = Some(&todo_status);

        let awesome_tag = String::from("awesome");
        let tag_filter: Option<&String> = Some(&awesome_tag);
        let todos = get_all_todos(&db, status_filter, tag_filter);
        assert_eq!(1, todos.len());

        let todo1 = todos.get(0).unwrap();
        matches!(todo1.status, Status::Done);
        assert_eq!(todo1.name, key1.to_string());
        assert_eq!(todo1.note, note1);
        assert_eq!(todo1.tags, tags1);
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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::ToDo, &due_date, &note, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &note, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &note, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &note, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &note, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &note, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

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
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

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
fn test_add_due_date_for_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let tags = Vec::new();
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

        let new_due_date = String::from("17-07-2022");
        let result = add_due_date(&db, &key, &new_due_date);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from(""));
        assert_eq!(todo.due_date, new_due_date);
        assert_eq!(0, todo.tags.len());
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_invalid_due_date_for_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let tags = Vec::new();
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

        let new_due_date = String::from("17-07-202222");
        let result = add_due_date(&db, &key, &new_due_date);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid date format");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_add_due_date_for_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let due_date = String::from("17-07-2022");
        let result = add_due_date(&db, &key, &due_date);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_change_due_date_for_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let tags = Vec::new();
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

        let new_due_date = String::from("17-07-2022");
        let result = change_due_date(&db, &key, &new_due_date);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from(""));
        assert_eq!(todo.due_date, new_due_date);
        assert_eq!(0, todo.tags.len());
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_change_invalid_due_date_for_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let tags = Vec::new();
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

        let new_due_date = String::from("17-07-202222");
        let result = change_due_date(&db, &key, &new_due_date);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid date format");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_change_due_date_for_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let due_date = String::from("17-07-2022");
        let result = change_due_date(&db, &key, &due_date);
        assert_eq!(true, result.is_err());
        assert_eq!(result.err().unwrap(), "Todo with this name does not exist");
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_remove_due_date_for_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");
        let notes = String::from("");
        let tags = Vec::new();
        let due_date = String::from("");
        insert_todo(&db, &key, Status::Done, &due_date, &notes, &tags);

        let result = remove_due_date(&db, &key);
        assert_eq!(true, result.is_ok());

        let db_value = String::from_utf8(db.get(&key).unwrap().unwrap()).unwrap();
        let todo: Todo = serde_json::from_str(&db_value).unwrap();

        assert_eq!(todo.name, key.to_string());
        matches!(todo.status, Status::ToDo);
        assert_eq!(todo.note, String::from(""));
        assert_eq!(todo.due_date, String::from(""));
        assert_eq!(0, todo.tags.len());
    }

    let _ = DB::destroy(&Options::default(), path);
}

#[test]
#[serial(timeout_ms = 1000)]
fn test_remove_due_date_for_missing_todo() {
    let path = "/tmp";
    {
        let db = DB::open_default(path).unwrap();

        let key = String::from("foo");

        let result = remove_due_date(&db, &key);
        assert_eq!(true, result.is_err());
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
        let tags = Vec::new();
        let due_date = String::from("");
        insert_todo(&db, &key, Status::ToDo, &due_date, &notes, &tags);

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

fn insert_todo(
    db: &DB,
    key: &String,
    status: Status,
    due_date: &String,
    notes: &String,
    tags: &Vec<String>,
) {
    let todo = Todo {
        name: key.to_string(),
        status,
        due_date: due_date.to_string(),
        note: notes.to_string(),
        tags: tags.to_vec(),
    };
    let serialized = serde_json::to_string(&todo).unwrap();
    db.put(&key, serialized).unwrap();
}

#[test]
// #[ignore]
#[serial(timeout_ms = 1000)]
fn destroy_db() {
    let path = "/tmp";
    let _ = DB::destroy(&Options::default(), path);
}

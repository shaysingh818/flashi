use crate::common::TestDb;
use flashi_lib::answer::answer_type::*;

#[test]
fn test_seeded_answer_types() {

    let db = TestDb::new();
    let conn = db.conn();

    let names: Vec<String> = AnswerType::retrieve(&conn).unwrap()
        .into_iter()
        .map(|a| a.name)
        .collect();
    assert_eq!(names, vec!["Open Ended", "Multiple Choice", "Ordered Sequence"]);

}

#[test]
fn test_create_answer_type() {

    let db = TestDb::new();
    let conn = db.conn();
    AnswerType::new("True or False").insert(&conn).unwrap();

    let answer_types = AnswerType::retrieve(&conn).unwrap();
    assert_eq!(answer_types.len(), 4);
    assert_eq!(answer_types.iter().any(|a| a.name == "True or False"), true);

}

#[test]
fn test_view_answer_type() {

    let db = TestDb::new();
    let conn = db.conn();

    let answer_type = AnswerType::view(&conn, "2").unwrap();
    assert_eq!(answer_type.answer_type_id, 2);
    assert_eq!(answer_type.name, "Multiple Choice");

    assert_eq!(AnswerType::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_answer_type() {

    let db = TestDb::new();
    let conn = db.conn();
    AnswerType::new("True or False").insert(&conn).unwrap();

    let mut answer_type = AnswerType::search(&conn, "True or False").unwrap().pop().unwrap();
    let id = answer_type.answer_type_id.to_string();
    answer_type.name = "Yes or No".to_string();
    answer_type.update(&conn, &id).unwrap();

    assert_eq!(AnswerType::view(&conn, &id).unwrap().name, "Yes or No");

}

#[test]
fn test_delete_answer_type() {

    let db = TestDb::new();
    let conn = db.conn();
    AnswerType::new("True or False").insert(&conn).unwrap();

    let mut answer_type = AnswerType::search(&conn, "True or False").unwrap().pop().unwrap();
    let id = answer_type.answer_type_id.to_string();
    answer_type.delete(&conn, &id).unwrap();

    assert_eq!(AnswerType::view(&conn, &id).is_err(), true);
    assert_eq!(AnswerType::retrieve(&conn).unwrap().len(), 3);

}

#[test]
fn test_search_answer_type() {

    let db = TestDb::new();
    let conn = db.conn();

    let mut results = AnswerType::search(&conn, "Ended").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Open Ended");

    results = AnswerType::search(&conn, "e").unwrap();
    assert_eq!(results.len(), 3);

    results = AnswerType::search(&conn, "Missing").unwrap();
    assert_eq!(results.len(), 0);

}

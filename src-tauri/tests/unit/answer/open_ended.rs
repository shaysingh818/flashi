use crate::common::TestDb;
use flashi_lib::answer::open_ended::*;

#[test]
fn test_create_open_ended_answer() {

    let db = TestDb::new();
    let conn = db.conn();

    let answer = OpenEndedAnswer::new("Ownership rules", "Explanation", vec!["owner", "borrow"])
        .insert(&conn)
        .unwrap();
    assert_eq!(answer.answer_id > 0, true);
    assert_eq!(answer.answer, "Ownership rules");
    assert_eq!(answer.explanation, "Explanation");
    assert_eq!(answer.keywords, vec!["owner", "borrow"]);

    let answers = OpenEndedAnswer::retrieve(&conn).unwrap();
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].keywords, vec!["owner", "borrow"]);

}

#[test]
fn test_open_ended_special_keywords_survive_updates() {

    let db = TestDb::new();
    let conn = db.conn();

    let expected = ["it's", "a, b", "back\\slash", "[x]"];
    let answer = OpenEndedAnswer::new("Answer", "Explanation", expected.to_vec())
        .insert(&conn)
        .unwrap();
    assert_eq!(answer.keywords, expected);

    let id = answer.answer_id.to_string();
    let mut viewed = OpenEndedAnswer::view(&conn, &id).unwrap();
    assert_eq!(viewed.keywords, expected);

    viewed.update(&conn, &id).unwrap();
    viewed.update(&conn, &id).unwrap();
    assert_eq!(OpenEndedAnswer::view(&conn, &id).unwrap().keywords, expected);

}

#[test]
fn test_view_open_ended_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let answer = OpenEndedAnswer::new("Answer view", "Explanation", vec!["key"]).insert(&conn).unwrap();

    let viewed = OpenEndedAnswer::view(&conn, &answer.answer_id.to_string()).unwrap();
    assert_eq!(viewed.answer, "Answer view");
    assert_eq!(viewed.keywords, vec!["key"]);

    assert_eq!(OpenEndedAnswer::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_open_ended_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut answer = OpenEndedAnswer::new("Answer", "Explanation", vec!["key"]).insert(&conn).unwrap();
    let id = answer.answer_id.to_string();

    answer.answer = "New answer".to_string();
    answer.explanation = "New explanation".to_string();
    answer.keywords = vec!["new1".to_string(), "new2".to_string()];
    answer.update(&conn, &id).unwrap();

    let updated = OpenEndedAnswer::view(&conn, &id).unwrap();
    assert_eq!(updated.answer, "New answer");
    assert_eq!(updated.explanation, "New explanation");
    assert_eq!(updated.keywords, vec!["new1", "new2"]);

}

#[test]
fn test_delete_open_ended_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut answer = OpenEndedAnswer::new("Answer", "Explanation", vec!["key"]).insert(&conn).unwrap();
    OpenEndedAnswer::new("Answer keep", "Explanation", vec!["key"]).insert(&conn).unwrap();

    let id = answer.answer_id.to_string();
    answer.delete(&conn, &id).unwrap();

    assert_eq!(OpenEndedAnswer::view(&conn, &id).is_err(), true);
    assert_eq!(OpenEndedAnswer::retrieve(&conn).unwrap().len(), 1);

}

#[test]
fn test_search_open_ended_answer() {

    let db = TestDb::new();
    let conn = db.conn();

    for answer in ["Rust ownership", "Rust borrowing", "Tauri commands"] {
        OpenEndedAnswer::new(answer, "Explanation", vec![]).insert(&conn).unwrap();
    }

    assert_eq!(OpenEndedAnswer::search(&conn, "Rust").unwrap().len(), 2);
    assert_eq!(OpenEndedAnswer::search(&conn, "Tauri").unwrap()[0].answer, "Tauri commands");
    assert_eq!(OpenEndedAnswer::search(&conn, "Missing").unwrap().len(), 0);

}

#[test]
fn test_validate_open_ended_answer() {

    let mut answer = OpenEndedAnswer::new("Answer", "Explanation", vec!["owner", "borrow"]);

    let (correct, explanation) = answer.validate("each value has one owner, others borrow it");
    assert_eq!(correct, true);
    assert_eq!(explanation, "");

    let (correct, explanation) = answer.validate("each value has one owner");
    assert_eq!(correct, false);
    assert_eq!(explanation.contains("borrow"), true);
    assert_eq!(explanation.contains("owner"), false);

}

#[test]
fn test_validate_open_ended_answer_without_keywords() {

    let mut answer = OpenEndedAnswer::new("Answer", "Explanation", vec![]);
    let (correct, _) = answer.validate("anything");
    assert_eq!(correct, true);

}

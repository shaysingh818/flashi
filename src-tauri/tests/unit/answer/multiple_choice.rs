use duckdb::Connection;
use crate::common::TestDb;
use flashi_lib::answer::multiple_choice::*;

fn insert_choices(conn: &Connection, answer_id: usize) -> Vec<MultipleChoiceAnswer> {
    vec![
        MultipleChoiceAnswer::new(answer_id, "Option A", true).insert(conn).unwrap(),
        MultipleChoiceAnswer::new(answer_id, "Option B", false).insert(conn).unwrap(),
        MultipleChoiceAnswer::new(answer_id, "Option C", true).insert(conn).unwrap(),
    ]
}

#[test]
fn test_create_multiple_choice() {

    let db = TestDb::new();
    let conn = db.conn();

    let multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();
    assert_eq!(multiple_choice.answer_id > 0, true);
    assert_eq!(multiple_choice.explanation, "Explanation");

    let viewed = MultipleChoice::view(&conn, &multiple_choice.answer_id.to_string()).unwrap();
    assert_eq!(viewed.explanation, "Explanation");

}

#[test]
fn test_view_missing_multiple_choice() {

    let db = TestDb::new();
    let conn = db.conn();
    assert_eq!(MultipleChoice::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_multiple_choice() {

    let db = TestDb::new();
    let conn = db.conn();

    let mut multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();
    let id = multiple_choice.answer_id.to_string();
    multiple_choice.explanation = "New explanation".to_string();
    multiple_choice.update(&conn, &id).unwrap();

    assert_eq!(MultipleChoice::view(&conn, &id).unwrap().explanation, "New explanation");

}

#[test]
fn test_delete_multiple_choice() {

    let db = TestDb::new();
    let conn = db.conn();

    let mut multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();
    let id = multiple_choice.answer_id.to_string();
    multiple_choice.delete(&conn, &id).unwrap();

    assert_eq!(MultipleChoice::view(&conn, &id).is_err(), true);

}

#[test]
fn test_create_multiple_choice_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();

    let answer = MultipleChoiceAnswer::new(multiple_choice.answer_id, "Option A", true).insert(&conn).unwrap();
    assert_eq!(answer.id > 0, true);
    assert_eq!(answer.answer_id, multiple_choice.answer_id);
    assert_eq!(answer.answer, "Option A");
    assert_eq!(answer.is_correct, true);

}

#[test]
fn test_retrieve_multiple_choice_answers() {

    let db = TestDb::new();
    let conn = db.conn();
    let choice1 = MultipleChoice::new("Explanation 1").insert(&conn).unwrap();
    let choice2 = MultipleChoice::new("Explanation 2").insert(&conn).unwrap();

    insert_choices(&conn, choice1.answer_id);
    MultipleChoiceAnswer::new(choice2.answer_id, "Other", false).insert(&conn).unwrap();

    let answers = MultipleChoiceAnswer::retrieve(choice1.answer_id, &conn).unwrap();
    assert_eq!(answers.len(), 3);
    assert_eq!(answers.iter().all(|a| a.answer_id == choice1.answer_id), true);

    let answers = MultipleChoiceAnswer::retrieve(choice2.answer_id, &conn).unwrap();
    assert_eq!(answers.len(), 1);

}

#[test]
fn test_view_multiple_choice_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();
    let answer = MultipleChoiceAnswer::new(multiple_choice.answer_id, "Option A", false).insert(&conn).unwrap();

    let viewed = MultipleChoiceAnswer::view(&conn, &answer.id.to_string()).unwrap();
    assert_eq!(viewed.answer, "Option A");
    assert_eq!(viewed.is_correct, false);

    assert_eq!(MultipleChoiceAnswer::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_multiple_choice_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();
    let mut answer = MultipleChoiceAnswer::new(multiple_choice.answer_id, "Option A", false).insert(&conn).unwrap();
    let id = answer.id.to_string();

    answer.answer = "Option Z".to_string();
    answer.is_correct = true;
    answer.update(&conn, &id).unwrap();

    let updated = MultipleChoiceAnswer::view(&conn, &id).unwrap();
    assert_eq!(updated.answer, "Option Z");
    assert_eq!(updated.is_correct, true);
    assert_eq!(updated.answer_id, multiple_choice.answer_id);

}

#[test]
fn test_delete_multiple_choice_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();
    let mut answers = insert_choices(&conn, multiple_choice.answer_id);

    let id = answers[0].id.to_string();
    answers[0].delete(&conn, &id).unwrap();

    assert_eq!(MultipleChoiceAnswer::view(&conn, &id).is_err(), true);
    assert_eq!(MultipleChoiceAnswer::retrieve(multiple_choice.answer_id, &conn).unwrap().len(), 2);

}

#[test]
fn test_validate_multiple_choice() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut multiple_choice = MultipleChoice::new("Explanation").insert(&conn).unwrap();
    let answers = insert_choices(&conn, multiple_choice.answer_id);

    let a = answers[0].id.to_string();
    let b = answers[1].id.to_string();
    let c = answers[2].id.to_string();

    // order of the selection does not matter
    let (correct, message) = multiple_choice.validate(&conn, vec![c.clone(), a.clone()]).unwrap();
    assert_eq!(correct, true);
    assert_eq!(message, "Correct!");

    // missing one correct answer
    let (correct, _) = multiple_choice.validate(&conn, vec![a.clone()]).unwrap();
    assert_eq!(correct, false);

    // all correct answers plus an incorrect one
    let (correct, _) = multiple_choice.validate(&conn, vec![a.clone(), b.clone(), c.clone()]).unwrap();
    assert_eq!(correct, false);

    let (correct, _) = multiple_choice.validate(&conn, vec![]).unwrap();
    assert_eq!(correct, false);

}

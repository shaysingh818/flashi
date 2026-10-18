use crate::common::TestDb;
use flashi_lib::answer::ordered_sequence::*;

#[test]
fn test_create_ordered_sequence_answer() {

    let db = TestDb::new();
    let conn = db.conn();

    let answer = OrderedSequenceAnswer::new(vec!["first", "second", "third"], "Explanation")
        .insert(&conn)
        .unwrap();
    assert_eq!(answer.answer_id > 0, true);
    assert_eq!(answer.sequence, vec!["first", "second", "third"]);
    assert_eq!(answer.explanation, "Explanation");

    let answers = OrderedSequenceAnswer::retrieve(&conn).unwrap();
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].sequence, vec!["first", "second", "third"]);

}

#[test]
fn test_sequence_items_with_commas_and_quotes_round_trip() {

    let db = TestDb::new();
    let conn = db.conn();

    let items = vec!["First, then second", "It's done", "a/b\\c", "plain"];
    let inserted = OrderedSequenceAnswer::new(items.clone(), "explanation").insert(&conn).unwrap();
    assert_eq!(inserted.sequence, items);

    let id = inserted.answer_id.to_string();
    let mut viewed = OrderedSequenceAnswer::view(&conn, &id).unwrap();
    assert_eq!(viewed.sequence, items);

    // Saving an edit and reading it back must not split or escape items
    viewed.update(&conn, &id).unwrap();
    assert_eq!(OrderedSequenceAnswer::view(&conn, &id).unwrap().sequence, items);

}

#[test]
fn test_view_ordered_sequence_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let answer = OrderedSequenceAnswer::new(vec!["a", "b"], "Explanation view").insert(&conn).unwrap();

    let viewed = OrderedSequenceAnswer::view(&conn, &answer.answer_id.to_string()).unwrap();
    assert_eq!(viewed.sequence, vec!["a", "b"]);
    assert_eq!(viewed.explanation, "Explanation view");

    assert_eq!(OrderedSequenceAnswer::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_ordered_sequence_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut answer = OrderedSequenceAnswer::new(vec!["a", "b"], "Explanation").insert(&conn).unwrap();
    let id = answer.answer_id.to_string();

    answer.sequence = vec!["c".to_string(), "b".to_string(), "a".to_string()];
    answer.explanation = "New explanation".to_string();
    answer.update(&conn, &id).unwrap();

    let updated = OrderedSequenceAnswer::view(&conn, &id).unwrap();
    assert_eq!(updated.sequence, vec!["c", "b", "a"]);
    assert_eq!(updated.explanation, "New explanation");

}

#[test]
fn test_delete_ordered_sequence_answer() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut answer = OrderedSequenceAnswer::new(vec!["a"], "Explanation").insert(&conn).unwrap();
    OrderedSequenceAnswer::new(vec!["b"], "Explanation keep").insert(&conn).unwrap();

    let id = answer.answer_id.to_string();
    answer.delete(&conn, &id).unwrap();

    assert_eq!(OrderedSequenceAnswer::view(&conn, &id).is_err(), true);
    assert_eq!(OrderedSequenceAnswer::retrieve(&conn).unwrap().len(), 1);

}

#[test]
fn test_search_ordered_sequence_answer() {

    let db = TestDb::new();
    let conn = db.conn();

    for explanation in ["Steps to boil water", "Steps to make tea", "Planet order"] {
        OrderedSequenceAnswer::new(vec!["a"], explanation).insert(&conn).unwrap();
    }

    assert_eq!(OrderedSequenceAnswer::search(&conn, "Steps").unwrap().len(), 2);
    assert_eq!(OrderedSequenceAnswer::search(&conn, "Planet").unwrap()[0].explanation, "Planet order");
    assert_eq!(OrderedSequenceAnswer::search(&conn, "Missing").unwrap().len(), 0);

}

#[test]
fn test_validate_ordered_sequence_answer() {

    let mut answer = OrderedSequenceAnswer::new(vec!["first", "second", "third"], "Explanation");
    let to_vec = |items: &[&str]| items.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let (correct, message) = answer.validate(to_vec(&["first", "second", "third"]));
    assert_eq!(correct, true);
    assert_eq!(message, "Correct!");

    let (correct, message) = answer.validate(to_vec(&["second", "first", "third"]));
    assert_eq!(correct, false);
    assert_eq!(message, r#"["first", "second", "third"]"#);

    let (correct, _) = answer.validate(to_vec(&["first", "second"]));
    assert_eq!(correct, false);

}

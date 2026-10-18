use duckdb::Connection;
use crate::common::TestDb;
use flashi_lib::subject::*;
use flashi_lib::deck::*;
use flashi_lib::question::*;
use flashi_lib::image_artifact::*;

const OPEN_ENDED: usize = 1;

fn insert_deck(conn: &Connection, name: &str) -> Deck {

    let mut subject = Subject::new("Test subject", "Test subject description");
    subject.insert(conn).unwrap();
    let subject = Subject::retrieve(conn).unwrap().pop().unwrap();

    let mut deck = Deck::new(subject.id, name, vec![], "Deck description");
    deck.insert(conn).unwrap();

    Deck::retrieve(conn).unwrap()
        .into_iter()
        .find(|d| d.name == name)
        .unwrap()

}

fn insert_question(conn: &Connection, deck_id: usize, raw: &str) -> Question {

    let id = Question::new(deck_id, OPEN_ENDED, 1, raw).insert(conn).unwrap();
    Question::view(conn, &id.to_string()).unwrap()

}

fn insert_image(conn: &Connection, path: &str) -> i64 {
    ImageArtifact::new(path).insert(conn).unwrap()
}

#[test]
fn test_create_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");

    let mut question = Question::new(deck.id, OPEN_ENDED, 7, "What is Rust?");
    let id = question.insert(&conn).unwrap();
    assert_eq!(id > 0, true);

    let questions = Question::retrieve(&conn, &deck.id.to_string()).unwrap();
    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0].id, id as usize);
    assert_eq!(questions[0].deck_id, deck.id);
    assert_eq!(questions[0].answer_type_id, OPEN_ENDED);
    assert_eq!(questions[0].answer_id, 7);
    assert_eq!(questions[0].raw, "What is Rust?");
    assert_eq!(questions[0].reviewed, 0);
    assert_eq!(questions[0].correct, 0);
    assert_eq!(questions[0].incorrect, 0);

}

#[test]
fn test_create_question_returns_unique_ids() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");

    let id1 = Question::new(deck.id, OPEN_ENDED, 1, "Question 1").insert(&conn).unwrap();
    let id2 = Question::new(deck.id, OPEN_ENDED, 1, "Question 2").insert(&conn).unwrap();
    assert_ne!(id1, id2);

}

#[test]
fn test_retrieve_questions_by_deck() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck1 = insert_deck(&conn, "Deck 1");
    let deck2 = insert_deck(&conn, "Deck 2");

    insert_question(&conn, deck1.id, "Question 1");
    insert_question(&conn, deck1.id, "Question 2");
    insert_question(&conn, deck2.id, "Question 3");

    let questions = Question::retrieve(&conn, &deck1.id.to_string()).unwrap();
    assert_eq!(questions.len(), 2);
    assert_eq!(questions.iter().all(|q| q.deck_id == deck1.id), true);

    let questions = Question::retrieve(&conn, &deck2.id.to_string()).unwrap();
    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0].raw, "Question 3");

}

#[test]
fn test_view_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");

    let id = Question::new(deck.id, OPEN_ENDED, 3, "Question view").insert(&conn).unwrap();

    let viewed = Question::view(&conn, &id.to_string()).unwrap();
    assert_eq!(viewed.id, id as usize);
    assert_eq!(viewed.deck_id, deck.id);
    assert_eq!(viewed.answer_id, 3);
    assert_eq!(viewed.raw, "Question view");

    assert_eq!(Question::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");
    let mut question = insert_question(&conn, deck.id, "Question update");
    let id = question.id.to_string();

    question.raw = "New question text".to_string();
    question.reviewed = 5;
    question.correct = 3;
    question.incorrect = 2;
    question.update(&conn, &id).unwrap();

    let updated = Question::view(&conn, &id).unwrap();
    assert_eq!(updated.raw, "New question text");
    assert_eq!(updated.reviewed, 5);
    assert_eq!(updated.correct, 3);
    assert_eq!(updated.incorrect, 2);
    assert_eq!(updated.deck_id, deck.id);
    assert_eq!(updated.answer_type_id, OPEN_ENDED);

}

#[test]
fn test_delete_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");
    let mut question = insert_question(&conn, deck.id, "Question delete");
    insert_question(&conn, deck.id, "Question keep");

    let id = question.id.to_string();
    question.delete(&conn, &id).unwrap();

    let questions = Question::retrieve(&conn, &deck.id.to_string()).unwrap();
    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0].raw, "Question keep");
    assert_eq!(Question::view(&conn, &id).is_err(), true);

}

#[test]
fn test_delete_question_with_images() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");
    let mut question = insert_question(&conn, deck.id, "Question with images");
    let id = question.id.to_string();

    let image1 = insert_image(&conn, "image1.png");
    let image2 = insert_image(&conn, "image2.png");
    question.add_image_artifact_to_question(&conn, &image1.to_string()).unwrap();
    question.add_image_artifact_to_question(&conn, &image2.to_string()).unwrap();

    question.delete(&conn, &id).unwrap();

    assert_eq!(Question::view(&conn, &id).is_err(), true);
    assert_eq!(ImageArtifact::retrieve_question_image_artifacts(&conn, &id).unwrap().len(), 0);

    // the image artifacts themselves are kept, only the links are removed
    assert_eq!(ImageArtifact::retrieve(&conn).unwrap().len(), 2);

}

#[test]
fn test_search_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");

    for raw in ["What is Rust?", "What is Tauri?", "Explain ownership"] {
        insert_question(&conn, deck.id, raw);
    }

    let mut results = Question::search(&conn, "What is").unwrap();
    assert_eq!(results.len(), 2);

    results = Question::search(&conn, "ownership").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].raw, "Explain ownership");

    results = Question::search(&conn, "Missing").unwrap();
    assert_eq!(results.len(), 0);

}

#[test]
fn test_add_image_artifact_to_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");
    let mut question = insert_question(&conn, deck.id, "Question 1");
    let image_id = insert_image(&conn, "image1.png");

    let result = question.add_image_artifact_to_question(&conn, &image_id.to_string()).unwrap();
    assert_eq!(result, "Success");

    let images = ImageArtifact::retrieve_question_image_artifacts(&conn, &question.id.to_string()).unwrap();
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].image_path, "image1.png");

}

#[test]
fn test_add_duplicate_image_artifact_to_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");
    let mut question = insert_question(&conn, deck.id, "Question 1");
    let image_id = insert_image(&conn, "image1.png").to_string();

    assert_eq!(question.add_image_artifact_to_question(&conn, &image_id).unwrap(), "Success");
    assert_eq!(question.add_image_artifact_to_question(&conn, &image_id).unwrap(), "Duplicate");

    let images = ImageArtifact::retrieve_question_image_artifacts(&conn, &question.id.to_string()).unwrap();
    assert_eq!(images.len(), 1);

}

#[test]
fn test_remove_image_artifact_from_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let deck = insert_deck(&conn, "Deck 1");
    let mut question = insert_question(&conn, deck.id, "Question 1");
    let question_id = question.id.to_string();

    let image1 = insert_image(&conn, "image1.png").to_string();
    let image2 = insert_image(&conn, "image2.png").to_string();
    question.add_image_artifact_to_question(&conn, &image1).unwrap();
    question.add_image_artifact_to_question(&conn, &image2).unwrap();

    let result = question.remove_image_artifact_from_question(&conn, &image1).unwrap();
    assert_eq!(result, "Success");

    let images = ImageArtifact::retrieve_question_image_artifacts(&conn, &question_id).unwrap();
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].image_path, "image2.png");

}

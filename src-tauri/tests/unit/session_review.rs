use duckdb::Connection;
use crate::common::*;
use flashi_lib::question::Question;
use flashi_lib::session::Session;
use flashi_lib::session_review::*;
use flashi_lib::session_review_question::SessionReviewQuestion;

// session with one deck of `question_count` questions
fn seed_session_with_questions(conn: &Connection, question_count: usize) -> Session {

    let subject_id = seed_subject(conn, "Subject");
    let deck_id = seed_deck(conn, subject_id, "Deck");
    for i in 0..question_count {
        seed_question(conn, deck_id, OPEN_ENDED, &format!("Question {i}"));
    }

    let mut session = seed_session(conn, "Session");
    session.add_deck_to_session(conn, &deck_id.to_string()).unwrap();
    session

}

#[test]
fn test_create_session_review() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");

    let review = SessionReview::new(session.id).insert(&conn).unwrap();
    assert_eq!(review.id > 0, true);
    assert_eq!(review.session_id, session.id);
    assert_eq!(review.complete, false);
    assert_eq!(review.cycle, 0);
    assert_eq!(review.current_question, 0);

}

#[test]
fn test_retrieve_session_reviews() {

    let db = TestDb::new();
    let conn = db.conn();
    let session1 = seed_session(&conn, "Session 1");
    let session2 = seed_session(&conn, "Session 2");

    SessionReview::new(session1.id).insert(&conn).unwrap();
    SessionReview::new(session1.id).insert(&conn).unwrap();
    SessionReview::new(session2.id).insert(&conn).unwrap();

    let reviews = SessionReview::retrieve(&conn, &session1.id.to_string()).unwrap();
    assert_eq!(reviews.len(), 2);
    assert_eq!(reviews.iter().all(|r| r.session_id == session1.id), true);

}

#[test]
fn test_view_session_review() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    let review = SessionReview::new(session.id).insert(&conn).unwrap();

    let viewed = SessionReview::view(&conn, &review.id.to_string()).unwrap();
    assert_eq!(viewed.id, review.id);
    assert_eq!(viewed.session_id, session.id);

    assert_eq!(SessionReview::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_session_review() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    let mut review = SessionReview::new(session.id).insert(&conn).unwrap();
    let id = review.id.to_string();

    review.complete = true;
    review.cycle = 2;
    review.current_question = 5;
    review.update(&conn, &id).unwrap();

    let updated = SessionReview::view(&conn, &id).unwrap();
    assert_eq!(updated.complete, true);
    assert_eq!(updated.cycle, 2);
    assert_eq!(updated.current_question, 5);

}

#[test]
fn test_delete_session_review() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    let mut review = SessionReview::new(session.id).insert(&conn).unwrap();
    SessionReview::new(session.id).insert(&conn).unwrap();

    let id = review.id.to_string();
    review.delete(&conn, &id).unwrap();

    assert_eq!(SessionReview::view(&conn, &id).is_err(), true);
    assert_eq!(SessionReview::retrieve(&conn, &session.id.to_string()).unwrap().len(), 1);

}

#[test]
fn test_delete_session_reviews_from_session() {

    let db = TestDb::new();
    let conn = db.conn();
    let session1 = seed_session(&conn, "Session 1");
    let session2 = seed_session(&conn, "Session 2");
    let mut review = SessionReview::new(session1.id).insert(&conn).unwrap();
    SessionReview::new(session1.id).insert(&conn).unwrap();
    SessionReview::new(session2.id).insert(&conn).unwrap();

    review.delete_from_session(&conn, &session1.id.to_string()).unwrap();

    assert_eq!(SessionReview::retrieve(&conn, &session1.id.to_string()).unwrap().len(), 0);
    assert_eq!(SessionReview::retrieve(&conn, &session2.id.to_string()).unwrap().len(), 1);

}

#[test]
fn test_session_review_active() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    let session_id = session.id.to_string();

    assert_eq!(SessionReview::session_review_active(&conn, &session_id).unwrap(), -1);

    let mut review = SessionReview::new(session.id).insert(&conn).unwrap();
    assert_eq!(SessionReview::session_review_active(&conn, &session_id).unwrap(), review.id as i64);

    review.complete = true;
    review.update(&conn, &review.id.to_string()).unwrap();
    assert_eq!(SessionReview::session_review_active(&conn, &session_id).unwrap(), -1);

}

#[test]
fn test_session_review_complete() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session_with_questions(&conn, 2);
    let mut review = SessionReview::new(session.id).insert(&conn).unwrap();
    let review_id = review.id.to_string();

    // nothing generated yet, so nothing is left to answer
    assert_eq!(SessionReview::session_review_complete(&conn, &review_id).unwrap(), true);

    review.generate_questions(&conn).unwrap();
    assert_eq!(SessionReview::session_review_complete(&conn, &review_id).unwrap(), false);

    for mut question in SessionReviewQuestion::retrieve_by_session_review_id(&conn, &review_id).unwrap() {
        question.active = false;
        question.update(&conn, &question.id.to_string()).unwrap();
    }
    assert_eq!(SessionReview::session_review_complete(&conn, &review_id).unwrap(), true);

}

#[test]
fn test_generate_questions() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session_with_questions(&conn, 3);

    // a deck that is not part of the session should not be included
    let subject_id = seed_subject(&conn, "Other subject");
    let other_deck = seed_deck(&conn, subject_id, "Other deck");
    seed_question(&conn, other_deck, OPEN_ENDED, "Other question");

    let mut review = SessionReview::new(session.id).insert(&conn).unwrap();
    review.generate_questions(&conn).unwrap();

    let questions = SessionReviewQuestion::retrieve_by_session_review_id(&conn, &review.id.to_string()).unwrap();
    assert_eq!(questions.len(), 3);
    assert_eq!(questions.iter().all(|q| q.session_review_id == review.id && q.active), true);

    let priorities: Vec<usize> = questions.iter().map(|q| q.priority_index).collect();
    assert_eq!(priorities, vec![0, 1, 2]);

}

#[test]
fn test_generate_questions_orders_by_correct() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session_with_questions(&conn, 3);

    let deck_id = Session::retrieve_session_decks(&conn, session.id).unwrap()[0].id;
    for (mut question, correct) in Question::retrieve(&conn, &deck_id.to_string()).unwrap().into_iter().zip([1, 5, 3]) {
        question.correct = correct;
        question.update(&conn, &question.id.to_string()).unwrap();
    }

    let mut review = SessionReview::new(session.id).insert(&conn).unwrap();
    review.generate_questions(&conn).unwrap();

    let raws: Vec<String> = SessionReviewQuestion::retrieve_by_session_review_id(&conn, &review.id.to_string())
        .unwrap()
        .into_iter()
        .map(|q| Question::view(&conn, &q.question_id.to_string()).unwrap().raw)
        .collect();
    assert_eq!(raws, vec!["Question 1", "Question 2", "Question 0"]);

}

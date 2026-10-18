use duckdb::Connection;
use crate::common::*;
use flashi_lib::session::Session;
use flashi_lib::session_review::SessionReview;
use flashi_lib::session_review_question::*;

// a session review plus the ids of `question_count` questions in the session's deck
fn seed_review(conn: &Connection, question_count: usize) -> (Session, SessionReview, Vec<usize>) {

    let subject_id = seed_subject(conn, "Subject");
    let deck_id = seed_deck(conn, subject_id, "Deck");
    let question_ids = (0..question_count)
        .map(|i| seed_question(conn, deck_id, OPEN_ENDED, &format!("Question {i}")))
        .collect();

    let mut session = seed_session(conn, "Session");
    session.add_deck_to_session(conn, &deck_id.to_string()).unwrap();
    let review = SessionReview::new(session.id).insert(conn).unwrap();
    (session, review, question_ids)

}

#[test]
fn test_create_session_review_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let (_, review, question_ids) = seed_review(&conn, 1);

    SessionReviewQuestion::new(review.id, question_ids[0], 4).insert(&conn).unwrap();

    let questions = SessionReviewQuestion::retrieve_by_session_review_id(&conn, &review.id.to_string()).unwrap();
    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0].session_review_id, review.id);
    assert_eq!(questions[0].question_id, question_ids[0]);
    assert_eq!(questions[0].priority_index, 4);
    assert_eq!(questions[0].incorrect_count, 0);
    assert_eq!(questions[0].review_count, 0);
    assert_eq!(questions[0].active, true);

}

#[test]
fn test_retrieve_session_review_questions() {

    let db = TestDb::new();
    let conn = db.conn();
    let (_, review, question_ids) = seed_review(&conn, 2);

    for (idx, question_id) in question_ids.iter().enumerate() {
        SessionReviewQuestion::new(review.id, *question_id, idx).insert(&conn).unwrap();
    }

    assert_eq!(SessionReviewQuestion::retrieve(&conn).unwrap().len(), 2);

}

#[test]
fn test_retrieve_by_session_review_id() {

    let db = TestDb::new();
    let conn = db.conn();
    let (session, review, question_ids) = seed_review(&conn, 3);
    let other_review = SessionReview::new(session.id).insert(&conn).unwrap();

    // inserted out of order to check the priority ordering
    for (question_id, priority) in question_ids.iter().zip([2, 0, 1]) {
        SessionReviewQuestion::new(review.id, *question_id, priority).insert(&conn).unwrap();
    }
    SessionReviewQuestion::new(other_review.id, question_ids[0], 0).insert(&conn).unwrap();

    let questions = SessionReviewQuestion::retrieve_by_session_review_id(&conn, &review.id.to_string()).unwrap();
    let priorities: Vec<usize> = questions.iter().map(|q| q.priority_index).collect();
    assert_eq!(priorities, vec![0, 1, 2]);
    assert_eq!(questions.iter().all(|q| q.session_review_id == review.id), true);

}

#[test]
fn test_retrieve_by_session_review_id_skips_inactive() {

    let db = TestDb::new();
    let conn = db.conn();
    let (_, review, question_ids) = seed_review(&conn, 2);

    SessionReviewQuestion::new(review.id, question_ids[0], 0).insert(&conn).unwrap();
    let mut inactive = SessionReviewQuestion::new(review.id, question_ids[1], 1);
    inactive.active = false;
    inactive.insert(&conn).unwrap();

    let questions = SessionReviewQuestion::retrieve_by_session_review_id(&conn, &review.id.to_string()).unwrap();
    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0].question_id, question_ids[0]);

}

#[test]
fn test_retrieve_active_questions() {

    let db = TestDb::new();
    let conn = db.conn();
    let (session, review, question_ids) = seed_review(&conn, 3);

    let mut first = SessionReviewQuestion::new(review.id, question_ids[0], 0);
    first.incorrect_count = 2;
    first.insert(&conn).unwrap();
    SessionReviewQuestion::new(review.id, question_ids[1], 1).insert(&conn).unwrap();
    let mut inactive = SessionReviewQuestion::new(review.id, question_ids[2], 2);
    inactive.active = false;
    inactive.insert(&conn).unwrap();

    let questions = SessionReviewQuestion::retrieve_active_questions(&conn, &session.id.to_string()).unwrap();
    let ids: Vec<usize> = questions.iter().map(|q| q.question_id).collect();
    // ordered by incorrect count, so the least missed comes first
    assert_eq!(ids, vec![question_ids[1], question_ids[0]]);

}

#[test]
fn test_view_session_review_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let (_, review, question_ids) = seed_review(&conn, 1);
    SessionReviewQuestion::new(review.id, question_ids[0], 0).insert(&conn).unwrap();

    let id = SessionReviewQuestion::retrieve(&conn).unwrap()[0].id.to_string();
    let viewed = SessionReviewQuestion::view(&conn, &id).unwrap();
    assert_eq!(viewed.question_id, question_ids[0]);
    assert_eq!(viewed.session_review_id, review.id);

    assert_eq!(SessionReviewQuestion::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_session_review_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let (_, review, question_ids) = seed_review(&conn, 1);
    SessionReviewQuestion::new(review.id, question_ids[0], 0).insert(&conn).unwrap();

    let mut question = SessionReviewQuestion::retrieve(&conn).unwrap().remove(0);
    let id = question.id.to_string();
    question.incorrect_count = 2;
    question.review_count = 3;
    question.priority_index = 7;
    question.active = false;
    question.update(&conn, &id).unwrap();

    let updated = SessionReviewQuestion::view(&conn, &id).unwrap();
    assert_eq!(updated.incorrect_count, 2);
    assert_eq!(updated.review_count, 3);
    assert_eq!(updated.priority_index, 7);
    assert_eq!(updated.active, false);

}

#[test]
fn test_delete_session_review_question() {

    let db = TestDb::new();
    let conn = db.conn();
    let (_, review, question_ids) = seed_review(&conn, 2);
    SessionReviewQuestion::new(review.id, question_ids[0], 0).insert(&conn).unwrap();
    SessionReviewQuestion::new(review.id, question_ids[1], 1).insert(&conn).unwrap();

    let mut question = SessionReviewQuestion::retrieve(&conn).unwrap().remove(0);
    let id = question.id.to_string();
    question.delete(&conn, &id).unwrap();

    assert_eq!(SessionReviewQuestion::view(&conn, &id).is_err(), true);
    assert_eq!(SessionReviewQuestion::retrieve(&conn).unwrap().len(), 1);

}

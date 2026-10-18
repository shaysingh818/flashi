use duckdb::Connection;
use crate::common::*;
use flashi_lib::analytics::*;
use flashi_lib::question::Question;
use flashi_lib::session_review::SessionReview;

fn insert_metrics(conn: &Connection, review_id: usize, correct: usize, incorrect: usize, accuracy: f64, review_time: f64) {
    conn.execute(
        "INSERT INTO SESSION_REVIEW_METRICS
            (SESSION_REVIEW_ID, QUESTIONS_CORRECT, QUESTIONS_INCORRECT, ACCURACY, REVIEW_TIME)
         VALUES (?, ?, ?, ?, ?)",
        duckdb::params![review_id, correct, incorrect, accuracy, review_time],
    ).unwrap();
}

#[test]
fn test_general_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    assert_eq!(general(&conn).unwrap(), (0, 0, 0));

    let subject_id = seed_subject(&conn, "Subject 1");
    seed_subject(&conn, "Subject 2");
    seed_deck(&conn, subject_id, "Deck 1");
    seed_deck(&conn, subject_id, "Deck 2");
    seed_deck(&conn, subject_id, "Deck 3");
    seed_session(&conn, "Session 1");

    assert_eq!(general(&conn).unwrap(), (2, 3, 1));

}

#[test]
fn test_historical_metrics_empty() {

    let db = TestDb::new();
    let conn = db.conn();
    assert_eq!(historical(&conn).unwrap().len(), 0);

}

#[test]
fn test_historical_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    let review = SessionReview::new(session.id).insert(&conn).unwrap();

    insert_metrics(&conn, review.id, 3, 1, 75.0, 60.0);
    insert_metrics(&conn, review.id, 1, 1, 50.0, 30.0);

    let history = historical(&conn).unwrap();
    assert_eq!(history.len(), 1);
    let (_, correct, incorrect, accuracy) = &history[0];
    assert_eq!(*correct, 4);
    assert_eq!(*incorrect, 2);
    assert_eq!(*accuracy, 62.5);

}

#[test]
fn test_historical_metrics_skips_old_entries() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    let review = SessionReview::new(session.id).insert(&conn).unwrap();

    insert_metrics(&conn, review.id, 3, 1, 75.0, 60.0);
    conn.execute("UPDATE SESSION_REVIEW_METRICS SET CREATED = now()::TIMESTAMP - INTERVAL 60 DAY", []).unwrap();

    assert_eq!(historical(&conn).unwrap().len(), 0);

}

#[test]
fn test_review_time_metrics_empty() {

    let db = TestDb::new();
    let conn = db.conn();
    assert_eq!(review_time(&conn).unwrap(), (0.0, 0.0, 0.0));

}

#[test]
fn test_review_time_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    let review = SessionReview::new(session.id).insert(&conn).unwrap();

    insert_metrics(&conn, review.id, 1, 0, 100.0, 60.0);
    insert_metrics(&conn, review.id, 1, 0, 100.0, 30.0);

    // everything lands on the same day, week and month
    assert_eq!(review_time(&conn).unwrap(), (90.0, 90.0, 90.0));

}

#[test]
fn test_question_review_time_metrics_empty() {

    let db = TestDb::new();
    let conn = db.conn();
    assert_eq!(question_review_time(&conn).unwrap(), (0, 0.0, 0.0));

}

#[test]
fn test_question_review_time_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject_id = seed_subject(&conn, "Subject");
    let deck_id = seed_deck(&conn, subject_id, "Deck");

    for reviewed in [2, 3] {
        let id = seed_question(&conn, deck_id, OPEN_ENDED, "Question").to_string();
        let mut question = Question::view(&conn, &id).unwrap();
        question.reviewed = reviewed;
        question.update(&conn, &id).unwrap();
    }

    assert_eq!(question_review_time(&conn).unwrap(), (5, 5.0, 5.0));

}

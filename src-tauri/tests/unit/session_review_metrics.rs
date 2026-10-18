use duckdb::Connection;
use crate::common::*;
use flashi_lib::session::Session;
use flashi_lib::session_review::SessionReview;
use flashi_lib::session_review_question::SessionReviewQuestion;
use flashi_lib::session_review_metrics::*;

// a review over `incorrect_counts.len()` questions, each answered with the given
// incorrect count and reviewed incorrect + 1 times
fn seed_answered_review(conn: &Connection, session: &mut Session, incorrect_counts: &[usize]) -> SessionReview {

    let subject_id = seed_subject(conn, &format!("Subject {}", session.id));
    let deck_id = seed_deck(conn, subject_id, &format!("Deck {}", session.id));
    session.add_deck_to_session(conn, &deck_id.to_string()).unwrap();

    let review = SessionReview::new(session.id).insert(conn).unwrap();
    for (idx, incorrect) in incorrect_counts.iter().enumerate() {
        let question_id = seed_question(conn, deck_id, OPEN_ENDED, &format!("Question {idx}"));
        let mut question = SessionReviewQuestion::new(review.id, question_id, idx);
        question.incorrect_count = *incorrect;
        question.review_count = incorrect + 1;
        question.active = false;
        question.insert(conn).unwrap();
    }
    review

}

#[test]
fn test_new_session_review_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session");
    let review = seed_answered_review(&conn, &mut session, &[0, 0, 0, 2]);

    let metrics = SessionReviewMetrics::new(review.id, &conn).unwrap();
    assert_eq!(metrics.session_review_id, review.id);
    assert_eq!(metrics.questions_correct, 3);
    assert_eq!(metrics.questions_incorrect, 1);
    assert_eq!(metrics.accuracy, 75.0);
    assert_eq!(metrics.total_review_count, 6);
    assert_eq!(metrics.number_of_cycles, 0);
    assert_eq!(metrics.review_time >= 0.0, true);

}

#[test]
fn test_new_session_review_metrics_uses_review_times() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session");
    let review = seed_answered_review(&conn, &mut session, &[0]);

    conn.execute(
        "UPDATE SESSION_REVIEW SET CYCLE = 3, START_TIME = '2026-01-01 10:00:00', END_TIME = '2026-01-01 10:02:30' WHERE ID = ?",
        [review.id],
    ).unwrap();

    let metrics = SessionReviewMetrics::new(review.id, &conn).unwrap();
    assert_eq!(metrics.review_time, 150.0);
    assert_eq!(metrics.number_of_cycles, 3);

}

#[test]
fn test_insert_session_review_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session");
    let review = seed_answered_review(&conn, &mut session, &[0, 1]);

    SessionReviewMetrics::new(review.id, &conn).unwrap().insert(&conn).unwrap();

    let metrics = SessionReviewMetrics::retrieve(&conn).unwrap();
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].id > 0, true);
    assert_eq!(metrics[0].session_review_id, review.id);
    assert_eq!(metrics[0].questions_correct, 1);
    assert_eq!(metrics[0].questions_incorrect, 1);
    assert_eq!(metrics[0].accuracy, 50.0);

}

#[test]
fn test_view_session_review_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session");
    let review = seed_answered_review(&conn, &mut session, &[0, 1]);
    SessionReviewMetrics::new(review.id, &conn).unwrap().insert(&conn).unwrap();

    let id = SessionReviewMetrics::retrieve(&conn).unwrap()[0].id.to_string();
    let viewed = SessionReviewMetrics::view(&conn, &id).unwrap();
    assert_eq!(viewed.session_review_id, review.id);
    assert_eq!(viewed.accuracy, 50.0);

    assert_eq!(SessionReviewMetrics::view(&conn, "999").is_err(), true);

}

#[test]
fn test_delete_session_review_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session");
    let review = seed_answered_review(&conn, &mut session, &[0]);
    SessionReviewMetrics::new(review.id, &conn).unwrap().insert(&conn).unwrap();
    SessionReviewMetrics::new(review.id, &conn).unwrap().insert(&conn).unwrap();

    let mut metrics = SessionReviewMetrics::retrieve(&conn).unwrap().remove(0);
    let id = metrics.id.to_string();
    metrics.delete(&conn, &id).unwrap();

    assert_eq!(SessionReviewMetrics::view(&conn, &id).is_err(), true);
    assert_eq!(SessionReviewMetrics::retrieve(&conn).unwrap().len(), 1);

}

#[test]
fn test_session_metrics_history() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session");
    let mut other_session = seed_session(&conn, "Other session");

    let mut complete = seed_answered_review(&conn, &mut session, &[0]);
    complete.complete = true;
    complete.update(&conn, &complete.id.to_string()).unwrap();
    let incomplete = SessionReview::new(session.id).insert(&conn).unwrap();
    let question_id = SessionReviewQuestion::retrieve(&conn).unwrap()[0].question_id;
    SessionReviewQuestion::new(incomplete.id, question_id, 0).insert(&conn).unwrap();
    let mut other = seed_answered_review(&conn, &mut other_session, &[0]);
    other.complete = true;
    other.update(&conn, &other.id.to_string()).unwrap();

    for review_id in [complete.id, complete.id, incomplete.id, other.id] {
        SessionReviewMetrics::new(review_id, &conn).unwrap().insert(&conn).unwrap();
    }

    // only metrics for completed reviews of this session
    let history = SessionReviewMetrics::session_metrics_history(&conn, &session.id.to_string()).unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history.iter().all(|m| m.session_review_id == complete.id), true);

}

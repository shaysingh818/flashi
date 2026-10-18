use duckdb::Connection;
use crate::common::*;
use flashi_lib::question::Question;
use flashi_lib::session::*;
use flashi_lib::session_review::SessionReview;
use flashi_lib::session_review_question::SessionReviewQuestion;
use flashi_lib::session_review_metrics::SessionReviewMetrics;

// session with one deck of `question_count` questions and a generated review
fn seed_reviewed_session(conn: &Connection, question_count: usize) -> (Session, SessionReview) {

    let subject_id = seed_subject(conn, "Subject");
    let deck_id = seed_deck(conn, subject_id, "Deck");
    for i in 0..question_count {
        seed_question(conn, deck_id, OPEN_ENDED, &format!("Question {i}"));
    }

    let mut session = seed_session(conn, "Session");
    session.add_deck_to_session(conn, &deck_id.to_string()).unwrap();

    let mut review = SessionReview::new(session.id).insert(conn).unwrap();
    review.generate_questions(conn).unwrap();
    (session, review)

}

fn count(conn: &Connection, table: &str) -> i64 {
    conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get(0)).unwrap()
}

#[test]
fn test_create_session() {

    let db = TestDb::new();
    let conn = db.conn();
    Session::new("Session 1", "Session description").insert(&conn).unwrap();

    let sessions = Session::retrieve(&conn).unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].name, "Session 1");
    assert_eq!(sessions[0].description, "Session description");

}

#[test]
fn test_view_session() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session view");

    let viewed = Session::view(&conn, &session.id.to_string()).unwrap();
    assert_eq!(viewed.id, session.id);
    assert_eq!(viewed.name, "Session view");

    assert_eq!(Session::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_session() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session update");
    let id = session.id.to_string();

    session.name = "New Session Name".to_string();
    session.description = "New Session Description".to_string();
    session.update(&conn, &id).unwrap();

    let updated = Session::view(&conn, &id).unwrap();
    assert_eq!(updated.name, "New Session Name");
    assert_eq!(updated.description, "New Session Description");

}

#[test]
fn test_search_session() {

    let db = TestDb::new();
    let conn = db.conn();
    for name in ["Morning review", "Evening review", "Exam cram"] {
        seed_session(&conn, name);
    }

    assert_eq!(Session::search(&conn, "review").unwrap().len(), 2);
    assert_eq!(Session::search(&conn, "Exam").unwrap()[0].name, "Exam cram");
    assert_eq!(Session::search(&conn, "Missing").unwrap().len(), 0);

}

#[test]
fn test_delete_session() {

    let db = TestDb::new();
    let conn = db.conn();
    let mut session = seed_session(&conn, "Session delete");
    seed_session(&conn, "Session keep");

    let id = session.id.to_string();
    session.delete(&conn, &id).unwrap();

    assert_eq!(Session::view(&conn, &id).is_err(), true);
    assert_eq!(Session::retrieve(&conn).unwrap().len(), 1);

}

#[test]
fn test_delete_session_removes_decks() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject_id = seed_subject(&conn, "Subject");
    let deck_id = seed_deck(&conn, subject_id, "Deck");
    let mut session = seed_session(&conn, "Session");
    session.add_deck_to_session(&conn, &deck_id.to_string()).unwrap();

    let id = session.id.to_string();
    session.delete(&conn, &id).unwrap();

    assert_eq!(Session::view(&conn, &id).is_err(), true);
    assert_eq!(count(&conn, "SESSION_DECK"), 0);
    // the deck itself is kept
    assert_eq!(count(&conn, "DECK"), 1);

}

#[test]
fn test_delete_session_removes_reviews() {

    let db = TestDb::new();
    let conn = db.conn();
    let (mut session, review) = seed_reviewed_session(&conn, 2);
    SessionReviewMetrics::new(review.id, &conn).unwrap().insert(&conn).unwrap();

    let id = session.id.to_string();
    session.delete(&conn, &id).unwrap();

    assert_eq!(Session::view(&conn, &id).is_err(), true);
    assert_eq!(count(&conn, "SESSION_REVIEW_QUESTION"), 0);
    assert_eq!(count(&conn, "SESSION_REVIEW_METRICS"), 0);
    assert_eq!(count(&conn, "SESSION_REVIEW"), 0);
    assert_eq!(count(&conn, "SESSION_DECK"), 0);
    // questions are kept
    assert_eq!(count(&conn, "QUESTION"), 2);

}

#[test]
fn test_add_deck_to_session() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject_id = seed_subject(&conn, "Subject");
    let deck1 = seed_deck(&conn, subject_id, "Deck 1");
    let deck2 = seed_deck(&conn, subject_id, "Deck 2");
    seed_deck(&conn, subject_id, "Deck 3");
    let mut session = seed_session(&conn, "Session");

    assert_eq!(session.add_deck_to_session(&conn, &deck1.to_string()).unwrap(), "Success");
    assert_eq!(session.add_deck_to_session(&conn, &deck2.to_string()).unwrap(), "Success");

    let mut names: Vec<String> = Session::retrieve_session_decks(&conn, session.id).unwrap()
        .into_iter()
        .map(|d| d.name)
        .collect();
    names.sort();
    assert_eq!(names, vec!["Deck 1", "Deck 2"]);

}

#[test]
fn test_add_duplicate_deck_to_session() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject_id = seed_subject(&conn, "Subject");
    let deck_id = seed_deck(&conn, subject_id, "Deck").to_string();
    let mut session = seed_session(&conn, "Session");

    assert_eq!(session.add_deck_to_session(&conn, &deck_id).unwrap(), "Success");
    assert_eq!(session.add_deck_to_session(&conn, &deck_id).unwrap(), "Duplicate");
    assert_eq!(Session::retrieve_session_decks(&conn, session.id).unwrap().len(), 1);

}

#[test]
fn test_remove_deck_from_session() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject_id = seed_subject(&conn, "Subject");
    let deck1 = seed_deck(&conn, subject_id, "Deck 1").to_string();
    let deck2 = seed_deck(&conn, subject_id, "Deck 2").to_string();
    let mut session = seed_session(&conn, "Session");
    session.add_deck_to_session(&conn, &deck1).unwrap();
    session.add_deck_to_session(&conn, &deck2).unwrap();

    assert_eq!(session.remove_deck_from_session(&conn, &deck1).unwrap(), "Success");

    let decks = Session::retrieve_session_decks(&conn, session.id).unwrap();
    assert_eq!(decks.len(), 1);
    assert_eq!(decks[0].name, "Deck 2");

}

#[test]
fn test_session_progress_status() {

    let db = TestDb::new();
    let conn = db.conn();
    let (session, review) = seed_reviewed_session(&conn, 3);
    let id = session.id.to_string();

    assert_eq!(Session::session_progress_status(&conn, &id).unwrap(), (0, 3));

    let mut question = SessionReviewQuestion::retrieve_by_session_review_id(&conn, &review.id.to_string())
        .unwrap()
        .remove(0);
    question.active = false;
    question.update(&conn, &question.id.to_string()).unwrap();

    assert_eq!(Session::session_progress_status(&conn, &id).unwrap(), (1, 3));

}

#[test]
fn test_session_progress_status_without_reviews() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    assert_eq!(Session::session_progress_status(&conn, &session.id.to_string()).unwrap(), (0, 0));

}

#[test]
fn test_session_metrics() {

    let db = TestDb::new();
    let conn = db.conn();
    let (session, review) = seed_reviewed_session(&conn, 2);

    let mut metrics = SessionReviewMetrics::new(review.id, &conn).unwrap();
    metrics.questions_correct = 3;
    metrics.questions_incorrect = 1;
    metrics.accuracy = 75.0;
    metrics.total_review_count = 4;
    metrics.review_time = 120.0;
    metrics.insert(&conn).unwrap();

    let (accuracy, reviewed, review_time, correct, incorrect) =
        Session::metrics(&conn, &session.id.to_string()).unwrap();
    assert_eq!(accuracy, 75.0);
    assert_eq!(reviewed, 4);
    assert_eq!(review_time, 120);
    assert_eq!(correct, 3);
    assert_eq!(incorrect, 1);

}

#[test]
fn test_session_metrics_without_reviews() {

    let db = TestDb::new();
    let conn = db.conn();
    let session = seed_session(&conn, "Session");
    assert_eq!(Session::metrics(&conn, &session.id.to_string()).unwrap(), (0.0, 0, 0, 0, 0));

}

#[test]
fn test_session_question_breakdown() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject_id = seed_subject(&conn, "Subject");
    let deck_id = seed_deck(&conn, subject_id, "Deck");
    seed_question(&conn, deck_id, OPEN_ENDED, "Question 1");
    seed_question(&conn, deck_id, OPEN_ENDED, "Question 2");
    seed_question(&conn, deck_id, MULTIPLE_CHOICE, "Question 3");

    let mut session = seed_session(&conn, "Session");
    session.add_deck_to_session(&conn, &deck_id.to_string()).unwrap();
    SessionReview::new(session.id).insert(&conn).unwrap().generate_questions(&conn).unwrap();

    let breakdown = Session::question_breakdown(&conn, &session.id.to_string()).unwrap();
    assert_eq!(breakdown, vec![
        ("Open Ended".to_string(), 2),
        ("Multiple Choice".to_string(), 1),
    ]);

}

#[test]
fn test_session_daily_accuracy_history() {

    let db = TestDb::new();
    let conn = db.conn();
    let (session, review) = seed_reviewed_session(&conn, 1);

    for accuracy in [50.0, 100.0] {
        let mut metrics = SessionReviewMetrics::new(review.id, &conn).unwrap();
        metrics.accuracy = accuracy;
        metrics.insert(&conn).unwrap();
    }

    let history = Session::daily_accuracy_history(&conn, &session.id.to_string()).unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].1, 75.0);

}

#[test]
fn test_session_correct_incorrect_questions() {

    let db = TestDb::new();
    let conn = db.conn();
    let (session, _) = seed_reviewed_session(&conn, 2);

    let deck_id = Session::retrieve_session_decks(&conn, session.id).unwrap()[0].id;
    let questions = Question::retrieve(&conn, &deck_id.to_string()).unwrap();
    for (mut question, (correct, incorrect)) in questions.into_iter().zip([(3, 1), (1, 3)]) {
        question.reviewed = 4;
        question.correct = correct;
        question.incorrect = incorrect;
        question.update(&conn, &question.id.to_string()).unwrap();
    }

    let (correct, incorrect) = Session::correct_incorrect_questions(&conn, &session.id.to_string()).unwrap();

    assert_eq!(correct.len(), 2);
    assert_eq!(correct[0].raw, "Question 0");
    assert_eq!(correct[0].correct, 3);
    assert_eq!(correct[0].correct_ratio, 0.75);

    assert_eq!(incorrect.len(), 2);
    assert_eq!(incorrect[0].raw, "Question 1");
    assert_eq!(incorrect[0].incorrect, 3);
    assert_eq!(incorrect[0].correct_ratio, 0.75);

}

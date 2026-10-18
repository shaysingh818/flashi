use chrono;
use duckdb::{params, Connection, OptionalExt, Result};
use serde::{Deserialize, Serialize};

use crate::queries::session_review_question::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReviewQuestion {
    pub id: usize,
    pub session_review_id: usize,
    pub question_id: usize,
    pub priority_index: usize,
    pub incorrect_count: usize,
    pub review_count: usize,
    pub active: bool,
    pub created: String,
    pub updated: String,
}

impl SessionReviewQuestion {

    pub fn new(
        session_review_id: usize,
        question_id: usize,
        priority_index: usize,
    ) -> SessionReviewQuestion {
        let now = chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        SessionReviewQuestion {
            id: 0,
            session_review_id: session_review_id,
            question_id: question_id,
            priority_index: priority_index,
            incorrect_count: 0,
            review_count: 0,
            active: true,
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            INSERT_SESSION_REVIEW_QUESTION,
            params![
                &self.session_review_id,
                &self.question_id,
                &self.priority_index,
                &self.incorrect_count,
                &self.review_count,
                &self.active,
                &self.created,
                &self.updated
            ],
        );

        match result {
            Ok(_) => {
                println!("[session_review_question::insert] Successfully inserted session review question");
                Ok(())
            }
            Err(err) => {
                println!("[session_review_question::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<SessionReviewQuestion>> {
        let mut stmt = conn.prepare(RETRIEVE_SESSION_REVIEW_QUESTIONS)?;

        let sessions: Result<Vec<SessionReviewQuestion>> = stmt
            .query_map([], |row| {
                Ok(SessionReviewQuestion {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?,
                    question_id: row.get(2)?,
                    priority_index: row.get(3)?,
                    incorrect_count: row.get(4)?,
                    review_count: row.get(5)?,
                    active: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            })?
            .collect();
        sessions
    }

    pub fn retrieve_by_session_review_id(
        conn: &Connection,
        session_review_id: &str,
    ) -> Result<Vec<SessionReviewQuestion>> {

        let mut stmt = conn.prepare(RETRIEVE_QUESTIONS_BY_SESSION_REVIEW)?;

        let sessions: Result<Vec<SessionReviewQuestion>> = stmt
            .query_map([&session_review_id], |row| {
                Ok(SessionReviewQuestion {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?,
                    question_id: row.get(2)?,
                    priority_index: row.get(3)?,
                    incorrect_count: row.get(4)?,
                    review_count: row.get(5)?,
                    active: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            })?
            .collect();
        sessions
    }

    pub fn retrieve_active_questions(
        conn: &Connection,
        session_id: &str,
    ) -> Result<Vec<SessionReviewQuestion>> {

        let mut stmt = conn.prepare(RETRIEVE_ACTIVE_SESSION_REVIEW_QUESTIONS)?;

        let sessions: Result<Vec<SessionReviewQuestion>> = stmt
            .query_map([session_id], |row| {
                Ok(SessionReviewQuestion {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?,
                    question_id: row.get(2)?,
                    priority_index: row.get(3)?,
                    incorrect_count: row.get(4)?,
                    review_count: row.get(5)?,
                    active: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            })?
            .collect();

        sessions
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        self.updated = chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let result = conn.execute(
            UPDATE_SESSION_REVIEW_QUESTION, 
            params![
                &self.session_review_id,
                &self.question_id,
                &self.priority_index,
                &self.incorrect_count,
                &self.review_count,
                &self.active,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("[session_review_question::update] Successfully updated session review question");
                Ok(())
            }
            Err(err) => {
                println!("[session_review_question::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM SESSION_REVIEW_QUESTION WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("[session_review_question::delete] Successfully deleted session review question");
                Ok(())
            }
            Err(err) => {
                println!(
                    "[session_review_question::delete] Error deleting session review question {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<SessionReviewQuestion> {

        conn.query_row(RETRIEVE_SESSION_REVIEW_QUESTION, &[&id], |row| {
            Ok(SessionReviewQuestion {
                id: row.get(0)?,
                session_review_id: row.get(1)?,
                question_id: row.get(2)?,
                priority_index: row.get(3)?,
                incorrect_count: row.get(4)?,
                review_count: row.get(5)?,
                active: row.get(6)?,
                created: row.get(7)?,
                updated: row.get(8)?,
            })
        })
    }
}


#[tauri::command]
pub fn view_session_review_questions(
    db_path: &str,
    session_review_id: &str,
) -> Vec<SessionReviewQuestion> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[session_review_question::view_session_review_questions] error {:?}", e);
            return Vec::new();
        }
    };

    match SessionReviewQuestion::retrieve_by_session_review_id(&conn, session_review_id) {
        Ok(session_questions) => session_questions,
        Err(e) => {
            println!("[session_review_question::view_session_review_questions] error {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_session_review_question(
    db_path: &str,
    id: &str,
) -> Result<SessionReviewQuestion, String> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[session_review_question::view_session_review_question] error {:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match SessionReviewQuestion::view(&conn, id) {
        Ok(session_question) => {
            return Ok(session_question);
        }
        Err(e) => {
            let err_msg = format!(
                "Error retrieving session question with id: {:?} Cause: {:?}",
                id, e
            );
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn view_active_session_review_questions(
    db_path: &str,
    session_id: &str,
) -> Vec<SessionReviewQuestion> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[session_review_question::view_active_session_review_questions] error {:?}", e);
            return Vec::new();
        }
    };

    match SessionReviewQuestion::retrieve_active_questions(&conn, session_id) {
        Ok(session_questions) => session_questions,
        Err(e) => {
            println!("[session_review_question::view_active_session_review_questions] error {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn edit_session_review_question(
    db_path: &str,
    id: &str,
    priority_index: usize,
    review_count: usize,
    incorrect_count: usize,
    active: bool,
) -> String {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[session_review_question::edit_session_review_question] error {:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return err_msg.to_string();
        }
    };

    match SessionReviewQuestion::view(&conn, id) {
        Ok(mut session_question) => {
            session_question.priority_index = priority_index;
            session_question.review_count = review_count;
            session_question.incorrect_count = incorrect_count;
            session_question.active = active;

            match session_question.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update session review question: {}", e),
            }
        }
        Err(e) => {
            let err_msg = format!(
                "Error retrieving session question with id: {:?} Cause: {:?}",
                id, e
            );
            return err_msg.to_string();
        }
    }
}

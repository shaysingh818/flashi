use chrono;
use duckdb::{params, Connection, OptionalExt, Result};
use serde::{Deserialize, Serialize};

use crate::queries::session_review::*;
use crate::session_review_question::*;
use crate::session_review_metrics::*; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReview {
    pub id: usize,
    pub session_id: usize,
    pub complete: bool,
    pub cycle: usize,
    pub current_question: usize,
    pub start_time: String,
    pub end_time: String,
    pub created: String,
    pub updated: String,
}

impl SessionReview {
    pub fn new(session_id: usize) -> SessionReview {
        let now = chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        SessionReview {
            id: 0,
            session_id: session_id,
            complete: false,
            cycle: 0,
            current_question: 0,
            start_time: now.clone(),
            end_time: now.clone(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<SessionReview> {
        conn.query_row(
            INSERT_SESSION_REVIEW,
            params![
                &self.session_id,
                &self.complete,
                &self.cycle,
                &self.current_question,
                &self.start_time,
                &self.end_time,
                &self.created,
                &self.updated
            ],
            |row| {
                Ok(SessionReview {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    complete: row.get(2)?,
                    cycle: row.get(3)?,
                    current_question: row.get(4)?,
                    start_time: row.get(5)?,
                    end_time: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            },
        )
    }

    pub fn session_review_active(conn: &Connection, session_id: &str) -> Result<i64> {

        let mut stmt = conn.prepare(SESSION_REVIEW_ACTIVE)?;
        let session_id: Option<i64> = stmt
            .query_row(params![session_id], |row| row.get(0))
            .optional()?;

        Ok(session_id.unwrap_or(-1))
    }

    pub fn session_review_complete(conn: &Connection, session_review_id: &str) -> Result<bool> {

        let mut stmt = conn.prepare(SESSION_REVIEW_COMPLETE)?;
        let active_question_count: i64 =
            stmt.query_row(params![session_review_id], |row| row.get(0))?;

        Ok(active_question_count == 0)
    }

    pub fn retrieve(conn: &Connection, session_id: &str) -> Result<Vec<SessionReview>> {
        let mut stmt = conn.prepare(RETRIEVE_SESSION_REVIEW)?;

        let sessions: Result<Vec<SessionReview>> = stmt
            .query_map([session_id], |row| {
                Ok(SessionReview {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    complete: row.get(2)?,
                    cycle: row.get(3)?,
                    current_question: row.get(4)?,
                    start_time: row.get(5)?,
                    end_time: row.get(6)?,
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
        self.end_time = chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let result = conn.execute(
            UPDATE_SESSION_REVIEW, 
            params![
                &self.complete,
                &self.cycle,
                &self.current_question,
                &self.start_time.to_string(),
                &self.end_time,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated session review");
                Ok(())
            }
            Err(err) => {
                println!("[session_review::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_SESSION_REVIEW, [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted session review");
                Ok(())
            }
            Err(err) => {
                println!("[session_review::delete] Error deleting session {:?}", err);
                Err(err)
            }
        }
    }
    
    pub fn delete_from_session(&mut self, conn: &Connection, session_id: &str) -> Result<()> {
        let result = conn.execute(REMOVE_SESSION_REVIEW_FROM_SESSION, [session_id]);
        match result {
            Ok(_) => {
                println!("Successfully removed session review's from existing session");
                Ok(())
            }
            Err(err) => {
                println!(
                    "[session_review::delete_from_session] Error deleting session review metrics {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<SessionReview> {
        conn.query_row(
            VIEW_SESSION_REVIEW,
            &[&id],
            |row| {
                Ok(SessionReview {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    complete: row.get(2)?,
                    cycle: row.get(3)?,
                    current_question: row.get(4)?,
                    start_time: row.get(5)?,
                    end_time: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            },
        )
    }

    pub fn generate_questions(&mut self, conn: &Connection) -> Result<()> {

        let mut stmt = conn.prepare(GENERATE_SESSION_REVIEW_QUESTIONS)?;

        let question_ids: Vec<usize> = stmt
            .query_map(&[&self.session_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        for (idx, question_id) in question_ids.into_iter().enumerate() {
            let mut session_question = SessionReviewQuestion::new(self.id, question_id, idx);
            session_question.insert(conn);
        }

        Ok(())
    }
}


#[tauri::command]
pub fn create_session_review(session_id: &str, db_path: &str) -> Result<SessionReview, String> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };

    let session_id_usize: usize = session_id.parse::<usize>().unwrap();

    let mut session_review: SessionReview = SessionReview::new(session_id_usize);

    match session_review.insert(&conn) {
        Ok(mut session_review_instance) => {
            session_review_instance.generate_questions(&conn).unwrap();
            Ok(session_review_instance)
        }
        Err(e) => Err(format!("Failed to insert session review: {}", e)),
    }
}

#[tauri::command]
pub fn edit_session_review(
    session_review_id: &str,
    complete: bool,
    cycle: usize,
    current_question: usize,
    start_time: &str,
    end_time: &str,
    db_path: &str,
) -> String {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match SessionReview::view(&conn, session_review_id) {
        Ok(mut session_review) => {
            session_review.complete = complete;
            session_review.cycle = cycle;
            session_review.current_question = current_question;
            session_review.start_time = start_time.to_string();
            session_review.end_time = end_time.to_string();

            match session_review.update(&conn, session_review_id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update session review: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating session review: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_session_reviews(db_path: &str, session_id: &str) -> Vec<SessionReview> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match SessionReview::retrieve(&conn, session_id) {
        Ok(session_reviews) => session_reviews,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_session_review(
    db_path: &str,
    session_review_id: &str,
) -> Result<SessionReview, String> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match SessionReview::view(&conn, session_review_id) {
        Ok(session_review) => {
            return Ok(session_review);
        }
        Err(e) => {
            let err_msg = format!(
                "Error retrieving session review with id: {:?} Cause: {:?}",
                session_review_id, e
            );
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn check_session_review_active(db_path: &str, session_id: &str) -> Result<i64, String> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match SessionReview::session_review_active(&conn, session_id) {
        Ok(result) => {
            return Ok(result);
        }
        Err(e) => {
            let err_msg = format!(
                "Error checking session review active: {:?} Cause: {:?}",
                session_id, e
            );
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn check_session_review_complete(
    db_path: &str,
    session_review_id: &str,
) -> Result<bool, String> {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match SessionReview::session_review_complete(&conn, session_review_id) {
        Ok(result) => {
            return Ok(result);
        }
        Err(e) => {
            let err_msg = format!(
                "Error checking session review complete: {:?} Cause: {:?}",
                session_review_id, e
            );
            return Err(err_msg.to_string());
        }
    }
}

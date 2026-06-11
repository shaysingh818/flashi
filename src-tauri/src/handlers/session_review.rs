use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::session_review::*; 


#[tauri::command]
pub fn create_session_review(session_id: &str, db_path: &str) -> Result<SessionReview, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };

    let session_id_usize: usize = session_id.parse::<usize>().unwrap();

    let mut session_review: SessionReview = SessionReview::new(
        session_id_usize
    );

    let existing_session_id = session_review.check_complete_sessions(&conn).unwrap();
    if existing_session_id == -1 {
        match session_review.insert(&conn) {
            Ok(mut session_review_instance) => {
                session_review_instance.generate_questions(&conn).unwrap();
                Ok(session_review_instance)
            }
            Err(e) => Err(format!("Failed to insert session review: {}", e)),
        }
    } else {
        match SessionReview::view(&conn, &existing_session_id.to_string()) {
            Ok(session_review) => Ok(session_review),
            Err(e) => Err(format!("Failed to retrieve existing session review: {}", e)),
        }
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
    db_path: &str) -> String {

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
pub fn view_session_review_questions(db_path: &str, session_review_id: &str) -> Vec<SessionReviewQuestion> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match SessionReviewQuestion::retrieve_by_session_review_id(&conn, session_review_id) {
        Ok(session_questions) => session_questions,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}



#[tauri::command]
pub fn view_session_review_question(db_path: &str, id: &str) -> Result<SessionReviewQuestion, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match SessionReviewQuestion::view(&conn, id) {
        Ok(session_question) => {
            return Ok(session_question);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving session question with id: {:?} Cause: {:?}", id, e);
            return Err(err_msg.to_string());
        }
    }
}

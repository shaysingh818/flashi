use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 


use crate::flashi_api::deck::Deck; 
use crate::flashi_api::session::*; 



#[tauri::command]
pub fn create_session(name: &str, description: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    let mut session: Session = Session::new(name, description);

    match session.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert session: {}", e),
    }
}

#[tauri::command]
pub fn edit_session(session_id: &str, name: &str, description: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
          
            session.name = name.to_string();            
            session.description = description.to_string();
            
            match session.update(&conn, session_id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update session: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating session: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_sessions(db_path: &str) -> Vec<Session> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Session::retrieve(&conn) {
        Ok(sessions) => sessions,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_session(db_path: &str, session_id: &str) -> Result<Session, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match Session::view(&conn, session_id) {
        Ok(session) => {
            return Ok(session);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving session with id: {:?} Cause: {:?}", session_id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_sessions(db_path: &str, search_term: &str) -> Vec<Session> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Session::search(&conn, search_term) {
        Ok(sessions) => sessions,
        Err(e) => {
            println!("Error searching sessions: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_session(db_path: &str, session_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
            let _ = session.delete(&conn, session_id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving subject with id: {:?} Cause: {:?}", session_id, e);
        }
    }
}


#[tauri::command]
pub fn view_session_decks(db_path: &str, session_id: &str) -> Vec<Deck> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    let session_id_usize: usize = session_id.parse::<usize>().unwrap();

    match Session::retrieve_session_decks(&conn, session_id_usize) {
        Ok(decks) => decks,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}


#[tauri::command]
pub fn add_deck_to_session(db_path: &str, session_id: &str, deck_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
            let result = session.add_deck_to_session(&conn, deck_id).unwrap();
            return result;
        }
        Err(e) => {
            return format!("Error adding deck to session with id: {:?} Cause: {:?}", session_id, e);
        }
    }
}


#[tauri::command]
pub fn remove_deck_from_session(db_path: &str, session_id: &str, deck_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
            let result = session.remove_deck_from_session(&conn, deck_id).unwrap();
            return result;
        }
        Err(e) => {
            return format!("Error adding deck to session with id: {:?} Cause: {:?}", session_id, e);
        }
    }
}


#[tauri::command]
pub fn session_progress_status(db_path: &str, session_id: &str) -> (usize, usize) {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return (0, 0);
        }
    };

    match Session::session_progress_status(&conn, session_id) {
        Ok((completed, total)) => {
            return (completed, total);
        }
        Err(e) => {
            println!("Error retrieving session progress status: {:?} Cause: {:?}", session_id, e);
            return (0, 0); 
        }
    }
}

#[tauri::command]
pub fn session_metrics(db_path: &str, session_id: &str) -> Result<(f64, usize, usize, usize, usize), String> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    };

    match Session::metrics(&conn, session_id) {
        Ok((avg_accuracy, total_reviewed, total_review_time, total_correct, total_incorrect)) => {
            Ok((avg_accuracy, total_reviewed, total_review_time, total_correct, total_incorrect))
        }
        Err(e) => {
            println!("Error retrieving session progress status: {:?} Cause: {:?}", session_id, e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn session_question_breakdown(db_path: &str, session_id: &str) -> Result<Vec<(String, i64)>, String> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string()); 
        }
    };

    match Session::question_breakdown(&conn, session_id) {
        Ok(rows) => Ok(rows), 
        Err(e) => {
            println!("Error retrieving session progress status: {:?} Cause: {:?}", session_id, e);
            return Err(e.to_string()) 
        }
    }
}
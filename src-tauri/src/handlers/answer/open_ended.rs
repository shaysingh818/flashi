use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::answer::open_ended::*; 


#[tauri::command]
pub fn create_open_ended_answer(answer: &str, explanation: &str, keywords: Vec<&str>, db_path: &str) -> Result<OpenEndedAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };
 
    let mut open_ended: OpenEndedAnswer = OpenEndedAnswer::new(answer, explanation, keywords);

    match open_ended.insert(&conn) {
        Ok(answer) => {
            let _ = conn.close();
            return Ok(answer);
        }
        Err(e) => Err(format!("Failed to insert open ended: {}", e)),
    }
}

#[tauri::command]
pub fn edit_open_ended_answer(id: &str, answer: &str, explanation: &str, keywords: Vec<String>, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match OpenEndedAnswer::view(&conn, id) {
        Ok(mut open_ended) => {

            open_ended.answer = answer.to_string();
            open_ended.explanation = explanation.to_string();            
            open_ended.keywords = keywords;            
            
            match open_ended.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update open ended: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating open ended: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_open_ended_answers(db_path: &str) -> Vec<OpenEndedAnswer> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match OpenEndedAnswer::retrieve(&conn) {
        Ok(open_ended_answers) => open_ended_answers,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_open_ended_answer(db_path: &str, id: &str) -> Result<OpenEndedAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match OpenEndedAnswer::view(&conn, id) {
        Ok(open_ended_answer) => {
            return Ok(open_ended_answer);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving ordered sequence with id: {:?} Cause: {:?}", id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_open_ended_answers(db_path: &str, search_term: &str) -> Vec<OpenEndedAnswer> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match OpenEndedAnswer::search(&conn, search_term) {
        Ok(open_ended_answer) => open_ended_answer,
        Err(e) => {
            println!("Error searching open ended answer: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_open_ended_answer(db_path: &str, id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match OpenEndedAnswer::view(&conn, id) {
        Ok(mut open_ended_answer) => {
            let _ = open_ended_answer.delete(&conn, id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving open ended answer with id: {:?} Cause: {:?}", id, e);
        }
    }
}
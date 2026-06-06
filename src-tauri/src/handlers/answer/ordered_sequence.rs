use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::answer::ordered_sequence::*; 


#[tauri::command]
pub fn create_ordered_sequence_answer(sequence: Vec<&str>, explanation: &str, db_path: &str) -> Result<OrderedSequenceAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };
 
    let mut ordered_sequence: OrderedSequenceAnswer = OrderedSequenceAnswer::new(sequence, explanation);

    match ordered_sequence.insert(&conn) {
        Ok(answer) => {
            let _ = conn.close();
            return Ok(answer)
        }
        Err(e) => Err(format!("Failed to insert ordered sequence: {}", e)),
    }
}

#[tauri::command]
pub fn edit_ordered_sequence_answer(id: &str, sequence: Vec<String>, explanation: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match OrderedSequenceAnswer::view(&conn, id) {
        Ok(mut ordered_sequence) => {
          
            ordered_sequence.explanation = explanation.to_string();            
            ordered_sequence.sequence = sequence;            
            
            match ordered_sequence.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update ordered sequence: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating ordered sequence: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_ordered_sequence_answers(db_path: &str) -> Vec<OrderedSequenceAnswer> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match OrderedSequenceAnswer::retrieve(&conn) {
        Ok(ordered_sequences) => ordered_sequences,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_ordered_sequence_answer(db_path: &str, id: &str) -> Result<OrderedSequenceAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match OrderedSequenceAnswer::view(&conn, id) {
        Ok(ordered_sequence) => {
            return Ok(ordered_sequence);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving ordered sequence with id: {:?} Cause: {:?}", id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_ordered_sequence_answers(db_path: &str, search_term: &str) -> Vec<OrderedSequenceAnswer> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match OrderedSequenceAnswer::search(&conn, search_term) {
        Ok(ordered_sequences) => ordered_sequences,
        Err(e) => {
            println!("Error searching ordered sequences: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_ordered_sequence_answer(db_path: &str, id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match OrderedSequenceAnswer::view(&conn, id) {
        Ok(mut ordered_sequence) => {
            let _ = ordered_sequence.delete(&conn, id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving ordered sequence with id: {:?} Cause: {:?}", id, e);
        }
    }
}

#[tauri::command]
pub fn validate_ordered_sequence_answer(db_path: &str, id: &str, answer: Vec<String>) -> (bool, String) {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return (false, e.to_string()); 
        }
    };

    match OrderedSequenceAnswer::view(&conn, id) {
        Ok(mut ordered_sequence) => {
            return ordered_sequence.validate(answer);
        }
        Err(e) => {
            let msg = format!("Error viewing ordered sequence answer for validation: {:?}", e);
            (false, msg.to_string())
        }
    }
}

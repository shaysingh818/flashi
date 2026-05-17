use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::answer::answer_type::*; 


#[tauri::command]
pub fn create_answer_type(name: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };
 
    let mut answer_type: AnswerType = AnswerType::new(name);

    match answer_type.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert answer type: {}", e),
    }
}

#[tauri::command]
pub fn edit_answer_type(id: &str, name: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match AnswerType::view(&conn, id) {
        Ok(mut answer_type) => {
          
            answer_type.name = name.to_string();            
            
            match answer_type.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update answer type: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating answer type: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_answer_types(db_path: &str) -> Vec<AnswerType> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match AnswerType::retrieve(&conn) {
        Ok(answer_types) => answer_types,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_answer_type(db_path: &str, answer_type_id: &str) -> Result<AnswerType, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match AnswerType::view(&conn, answer_type_id) {
        Ok(answer_type) => {
            return Ok(answer_type);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving answer type with id: {:?} Cause: {:?}", answer_type_id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_answer_types(db_path: &str, search_term: &str) -> Vec<AnswerType> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match AnswerType::search(&conn, search_term) {
        Ok(answer_types) => answer_types,
        Err(e) => {
            println!("Error searching answer types: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_answer_type(db_path: &str, answer_type_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match AnswerType::view(&conn, answer_type_id) {
        Ok(mut answer_type) => {
            let _ = answer_type.delete(&conn, answer_type_id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving answer type with id: {:?} Cause: {:?}", answer_type_id, e);
        }
    }
}
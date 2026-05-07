use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::subject::*; 


#[tauri::command]
pub fn create_subject(name: &str, description: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    let mut subject: Subject = Subject::new(name, description);

    match subject.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert subject: {}", e),
    }
}

#[tauri::command]
pub fn edit_subject(subject_id: &str, name: &str, description: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match Subject::view(&conn, subject_id) {
        Ok(mut subject) => {
          
            subject.name = name.to_string();            
            subject.description = description.to_string();
            
            match subject.update(&conn, subject_id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update subject: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating subject: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_subjects(db_path: &str) -> Vec<Subject> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Subject::retrieve(&conn) {
        Ok(subjects) => subjects,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_subject(db_path: &str, subject_id: &str) -> Result<Subject, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match Subject::view(&conn, subject_id) {
        Ok(subject) => {
            return Ok(subject);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving subject with id: {:?} Cause: {:?}", audio_file_id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_subjects(db_path: &str, search_term: &str) -> Vec<Subject> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Subject::search(&conn, search_term) {
        Ok(subjects) => subjects,
        Err(e) => {
            println!("Error searching subjects: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_subject(db_path: &str, subject_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Subject::view(&conn, subject_id) {
        Ok(mut subject) => {
            let _ = subject.delete(&conn, subject_id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving subject with id: {:?} Cause: {:?}", audio_file_id, e);
        }
    }
}
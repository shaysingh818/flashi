use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::answer::multiple_choice::*; 


#[tauri::command]
pub fn create_multiple_choice(db_path: &str) -> Result<MultipleChoice, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };

    let mut multiple_choice: MultipleChoice = MultipleChoice::new();

    match multiple_choice.insert(&conn) {
        Ok(answer) => {
            let _ = conn.close();
            Ok(answer)
        }
        Err(e) => Err(format!("Failed to insert multiple choice: {}", e)),
    }
}


#[tauri::command]
pub fn create_multiple_choice_answer(answer_id: &str, answer: &str, explanation: &str, is_correct: bool, db_path: &str) -> Result<MultipleChoiceAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    };

    let answer_id_usize: usize = answer_id.parse::<usize>().unwrap();
    let mut multiple_choice: MultipleChoiceAnswer = MultipleChoiceAnswer::new(answer_id_usize, answer, explanation, is_correct);

    match multiple_choice.insert(&conn) {
        Ok(answer) => {
            let _ = conn.close();
            Ok(answer)
        }
        Err(e) => Err(format!("Failed to insert multiple choice: {}", e)),
    }
}

#[tauri::command]
pub fn edit_multiple_choice_answer(id: &str, answer_id: &str, answer: &str, explanation: &str, is_correct: bool, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    let answer_id_usize: usize = answer_id.parse::<usize>().unwrap();

    match MultipleChoiceAnswer::view(&conn, id) {
        Ok(mut multiple_choice) => {

            multiple_choice.answer_id = answer_id_usize;
            multiple_choice.answer = answer.to_string();
            multiple_choice.explanation = explanation.to_string();            
            multiple_choice.is_correct = is_correct;            
            
            match multiple_choice.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update multiple choice: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating multiple choice: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_multiple_choice_answers(answer_id: &str, db_path: &str) -> Vec<MultipleChoiceAnswer> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    let answer_id_usize: usize = answer_id.parse::<usize>().unwrap();

    match MultipleChoiceAnswer::retrieve(answer_id_usize, &conn) {
        Ok(multiple_choice_answers) => multiple_choice_answers,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}


#[tauri::command]
pub fn view_multiple_choice(db_path: &str, id: &str) -> Result<MultipleChoice, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match MultipleChoice::view(&conn, id) {
        Ok(multiple_choice) => {
            return Ok(multiple_choice);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving multiple choice answer with id: {:?} Cause: {:?}", id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn view_multiple_choice_answer(db_path: &str, id: &str) -> Result<MultipleChoiceAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match MultipleChoiceAnswer::view(&conn, id) {
        Ok(multiple_choice_answer) => {
            return Ok(multiple_choice_answer);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving multiple choice answer with id: {:?} Cause: {:?}", id, e);
            return Err(err_msg.to_string());
        }
    }
}


#[tauri::command]
pub fn delete_multiple_choice(db_path: &str, id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match MultipleChoice::view(&conn, id) {
        Ok(mut multiple_choice) => {
            let _ = multiple_choice.delete(&conn, id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving multiple choice answer with id: {:?} Cause: {:?}", id, e);
        }
    }
}


#[tauri::command]
pub fn delete_multiple_choice_answer(db_path: &str, id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match MultipleChoiceAnswer::view(&conn, id) {
        Ok(mut multiple_choice_answer) => {
            let _ = multiple_choice_answer.delete(&conn, id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving multiple choice answer with id: {:?} Cause: {:?}", id, e);
        }
    }
}

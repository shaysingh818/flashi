use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::question::*; 


#[tauri::command]
pub fn create_question(deck_id: &str, answer_type_id: &str, answer_id: &str, raw: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    let deck_id_usize: usize = deck_id.parse::<usize>().unwrap();
    let answer_type_id_usize: usize = answer_type_id.parse::<usize>().unwrap();
    let answer_id_usize: usize = answer_id.parse::<usize>().unwrap();
    
    let mut question: Question = Question::new(deck_id_usize, answer_type_id_usize, answer_id_usize, raw);

    match question.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert question: {}", e),
    }
}

#[tauri::command]
pub fn edit_question(id: &str, raw: &str, reviewed: usize, correct: usize, incorrect: usize, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match Question::view(&conn, id) {
        Ok(mut question) => {
          
            question.raw = raw.to_string();           
            question.reviewed = reviewed;
            question.correct = correct; 
            question.incorrect = incorrect;
            
            match question.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update deck: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating deck: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_questions(db_path: &str, deck_id: &str) -> Vec<Question> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Question::retrieve(&conn, deck_id) {
        Ok(questions) => questions,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_question(db_path: &str, question_id: &str) -> Result<Question, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match Question::view(&conn, question_id) {
        Ok(question) => {
            return Ok(question);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving question with id: {:?} Cause: {:?}", question_id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_questions(db_path: &str, search_term: &str) -> Vec<Question> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Question::search(&conn, search_term) {
        Ok(questions) => questions,
        Err(e) => {
            println!("Error searching deck: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_question(db_path: &str, question_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Question::view(&conn, question_id) {
        Ok(mut question) => {

            match question.delete(&conn, question_id) {
                Ok(_) => {
                    return format!("Success");
                },
                Err(err) => {
                    return format!("Error deleting question {}: {:?}", question_id, err); 
                }
            }    
        }
        Err(e) => {
            return format!("Error retrieving question with id: {:?} Cause: {:?}", question_id, e);
        }
    }
}


#[tauri::command]
pub fn create_image_artifact(
    image_file_name: &str, 
    user_image_path: &str,
    user_local_image_path: &str, 
    db_path: &str) -> String {
 
    let image_file_name_final = match upload_image(
          image_file_name,
          user_image_path,
          user_local_image_path,
      ) {
          Ok(name) => name,
          Err(e) => return format!("Failed to upload image: {}", e),
      };

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };
 
    let mut image_artifact: ImageArtifact = ImageArtifact::new(&image_file_name_final);
    match image_artifact.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert question: {}", e),
    }
}


#[tauri::command]
pub fn view_question_image_artifacts(db_path: &str, question_id: &str) -> Vec<ImageArtifact> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match ImageArtifact::retrieve_question_image_artifacts(&conn, question_id) {
        Ok(images) => images,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}


#[tauri::command]
pub fn edit_image_artifact(
    id: &str,
    image_file_name: &str, 
    user_image_path: &str,
    user_local_image_path: &str, 
    db_path: &str) -> String {
 
    let image_file_name_final = match upload_image(
          image_file_name,
          user_image_path,
          user_local_image_path,
      ) {
          Ok(name) => name,
          Err(e) => return format!("Failed to upload image: {}", e),
      };

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    match ImageArtifact::view(&conn, id) {
        Ok(mut image_artifact) => {
            image_artifact.image_path = image_file_name_final; 
            match image_artifact.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    match remove_image(
                          image_file_name,
                          user_image_path,
                      ) {
                          Ok(name) => println!("removed image file name: {:?} ", name),
                          Err(e) => return format!("Failed to remove image: {}", e),
                      };
                    
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update deck: {}", e),
            }
            
        }
        Err(e) => {            
            let msg = format!("Error updating image artifact: {:?}", e);
            return msg.to_string();
        }
    }
 
}


#[tauri::command]
pub fn delete_image_artifact(
    id: &str,
    image_file_name: &str,    
    user_image_path: &str,
    db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };
    
    match ImageArtifact::view(&conn, id) {
        Ok(mut image_artifact) => {

            match image_artifact.delete(&conn, id) {
                Ok(_) => {

                    match remove_image(
                          image_file_name,
                          user_image_path,
                      ) {
                          Ok(name) => println!("Removed image file name: {:?}", name),
                          Err(e) => return format!("Failed to remove image: {}", e),
                      };
                    
                    return format!("Success");
                },
                Err(err) => {
                    return format!("Error deleting question {}: {:?}", id, err); 
                }
            }    
        }
        Err(e) => {
            return format!("Error retrieving question with id: {:?} Cause: {:?}", id, e);
        }
    }
 
}
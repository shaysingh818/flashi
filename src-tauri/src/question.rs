use chrono;
use std::io;
use std::fs;
use std::fmt;
use std::ffi::OsStr;
use std::path::Path;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

use crate::queries::question::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub deck_id: usize,
    pub answer_type_id: usize,
    pub answer_id: usize,
    pub reviewed: usize,
    pub correct: usize, 
    pub incorrect: usize,
    pub raw: String,
    pub created: String,
    pub updated: String
}

impl Question {

    pub fn new(deck_id: usize, answer_type_id: usize, answer_id: usize, raw: &str) -> Question {

        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      
        Question {
            id: 0,
            deck_id: deck_id,
            answer_type_id: answer_type_id,
            answer_id: answer_id,
            reviewed: 0,
            correct: 0, 
            incorrect: 0,
            raw: raw.to_string(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<i64> {
         
        let result = conn.prepare(INSERT_QUESTION)
          .and_then(|mut stmt| {
              stmt.query_row(
                  params![ 
                      &self.deck_id,
                      &self.answer_type_id,
                      &self.answer_id,
                      &self.reviewed,
                      &self.correct,
                      &self.incorrect,                
                      &self.raw,
                      &self.created,
                      &self.updated,
                  ],
                  |row| row.get::<_, i64>(0),
              )
          });

        match result {
            Ok(id) => {
                println!("[question::insert] Successfully inserted question with id: {:?}", id);
                Ok(id)
            },
            Err(err) => {
                println!("[question::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection, deck_id: &str) -> Result<Vec<Question>> {

        let mut stmt = conn.prepare(VIEW_QUESTIONS)?;

        let questions: Result<Vec<Question>> = stmt
            .query_map([&deck_id], |row| {
                Ok(Question {
                    id: row.get("ID")?,
                    deck_id: row.get("DECK_ID")?,
                    answer_type_id: row.get("ANSWER_TYPE_ID")?,
                    answer_id: row.get("ANSWER_ID")?,
                    reviewed: row.get("REVIEWED")?,
                    correct: row.get("CORRECT")?, 
                    incorrect: row.get("INCORRECT")?,
                    raw: row.get("RAW")?,
                    created: row.get("CREATED")?,
                    updated: row.get("UPDATED")?,
                })
            })?
            .collect();
        questions
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {

      self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            UPDATE_QUESTION,
            params![
                &self.raw,
                &self.reviewed,
                &self.correct,
                &self.incorrect,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("[question::update] Successfully updated question");
                Ok(())
            },
            Err(err) => {
                println!("[question::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {

        // Images currently attached to this question
        let this_question_images: Vec<i64> = {
            let mut stmt = conn.prepare(RETRIEVE_QUESTION_IMAGE_IDS)?;
            let rows = stmt.query_map([id], |row| row.get(0))?;
            rows.collect::<std::result::Result<Vec<i64>, _>>()?
        };
     
        for image_id in &this_question_images {
            if let Err(err) = conn.execute(
                DELETE_QUESTION_IMAGES,
                duckdb::params![id, image_id],
            ) {
                println!("[question::delete] Error removing question_image link for image {}: {:?}", image_id, err);
                return Err(err);
            } 
        }
    
        match conn.execute(DELETE_QUESTION, [id]) {
            Ok(_) => {
                println!("[question::delete] Successfully deleted question");
                Ok(())
            }
            Err(err) => {
                println!("[question::delete] Error deleting question {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<Question> {
        conn.query_row(
            VIEW_QUESTION,
            &[&id],
            |row| {
                Ok(Question {
                    id: row.get(0)?,
                    deck_id: row.get(1)?,
                    answer_type_id: row.get(2)?,
                    answer_id: row.get(3)?,
                    reviewed: row.get(4)?,
                    correct: row.get(5)?,
                    incorrect: row.get(6)?,
                    raw: row.get(7)?,
                    created: row.get(8)?,
                    updated: row.get(9)?,
                })
            }
        )
    }

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Question>> {
        let mut stmt = conn.prepare(&SEARCH_QUESTION)?;

        let questions: Result<Vec<Question>> = stmt
            .query_map([search_term], |row| {
                Ok(Question {
                    id: row.get(0)?,
                    deck_id: row.get(1)?,
                    answer_type_id: row.get(2)?,
                    answer_id: row.get(3)?,
                    reviewed: row.get(4)?,
                    correct: row.get(5)?,
                    incorrect: row.get(6)?,
                    raw: row.get(7)?,
                    created: row.get(8)?,
                    updated: row.get(9)?,
                })
            })?
            .collect();
        questions
    }

    pub fn add_image_artifact_to_question(&mut self, conn: &Connection, image_id: &str) -> Result<String> {

        let result = conn.execute(
            ADD_QUESTION_IMAGE_ARTIFACT,
            params![
                &self.id,
                image_id,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("[question::add_image_artifact_to_question] Successfully added image artifact to question");
                Ok(("Success").to_string())
            },
            Err(duckdb::Error::DuckDBFailure(err, msg)) => {
                let msg_str = msg.as_deref().unwrap_or("");
                if msg_str.contains("violates unique constraint") {
                    println!("[question::add_image_artifact_to_question] Image artifact already associated with question, skipping");
                    Ok("Duplicate".to_string())
                } else {
                    println!("[question::add_image_artifact_to_question] duckdb error {:?}", msg_str);
                    Err(duckdb::Error::DuckDBFailure(err, msg))
                }
            },
            Err(err) => {
                println!("[question::add_image_artifact_to_question] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn remove_image_artifact_from_question(&mut self, conn: &Connection, image_id: &str) -> Result<String> {
        
        let result = conn.execute(
            REMOVE_QUESTION_IMAGE_ARTIFACT,
            params![
                &self.id,
                image_id,
            ],
        );

        match result {
            Ok(_) => {
                println!("[question::remove_image_artifact_from_question] Successfully removed image artifact from question");
                Ok(("Success").to_string())
            },
            Err(err) => {
                println!("[question::remove_image_artifact_from_question] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

}


#[tauri::command]
pub fn create_question(deck_id: &str, answer_type_id: &str, answer_id: &str, raw: &str, db_path: &str) -> (i64, String) {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return (-1, format!("Failed to open database: {}", e)),
    };

    let deck_id_usize: usize = deck_id.parse::<usize>().unwrap();
    let answer_type_id_usize: usize = answer_type_id.parse::<usize>().unwrap();
    let answer_id_usize: usize = answer_id.parse::<usize>().unwrap();

    let mut question: Question = Question::new(deck_id_usize, answer_type_id_usize, answer_id_usize, raw);

    match question.insert(&conn) {
        Ok(id) => {
            let _ = conn.close();
            return (id, "Success".to_string());
        }
        Err(e) => (-1, format!("Failed to insert question: {}", e)),
    }
}

#[tauri::command]
pub fn edit_question(id: &str, raw: &str, reviewed: usize, correct: usize, incorrect: usize, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[question::edit_question] error {:?}", e);
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
            println!("[question::view_questions] error {:?}", e);
            return Vec::new();
        }
    };

    match Question::retrieve(&conn, deck_id) {
        Ok(questions) => questions,
        Err(e) => {
            println!("[question::view_questions] error {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_question(db_path: &str, question_id: &str) -> Result<Question, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[question::view_question] error {:?}", e);
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
pub fn add_question_image_artifact(db_path: &str, question_id: &str, image_id: &str) -> String {
    
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[question::add_question_image_artifact] error {:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Question::view(&conn, question_id) {
        Ok(mut question) => {
            let result = question.add_image_artifact_to_question(&conn, image_id).unwrap();
            return result;
        }
        Err(e) => {
            return format!("Error adding image artifact to question with id: {:?} Cause: {:?}", image_id, e);
        }
    }
    
}

#[tauri::command]
pub fn remove_question_image_artifact(db_path: &str, question_id: &str, image_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[question::remove_question_image_artifact] error {:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Question::view(&conn, question_id) {
        Ok(mut question) => {
            let result = question.remove_image_artifact_from_question(&conn, image_id).unwrap();
            return result;
        }
        Err(e) => {
            return format!("Error adding deck to session with id: {:?} Cause: {:?}", image_id, e);
        }
    }
}

#[tauri::command]
pub fn search_questions(db_path: &str, search_term: &str) -> Vec<Question> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[question::search_questions] error {:?}", e);
            return Vec::new();
        }
    };

    match Question::search(&conn, search_term) {
        Ok(questions) => questions,
        Err(e) => {
            println!("[question::search_questions] Error searching questions: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_question(db_path: &str, question_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[question::delete_question] error {:?}", e);
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


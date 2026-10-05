use chrono;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

use crate::queries::answer::multiple_choice::*; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoiceAnswer {
    pub id: usize,
    pub answer_id: usize,
    pub is_correct: bool,
    pub answer: String,
    pub explanation: String,
    pub created: String,
    pub updated: String
}

impl MultipleChoiceAnswer {
    
    pub fn new(answer_id: usize, answer: &str, explanation: &str, is_correct: bool) -> MultipleChoiceAnswer {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      MultipleChoiceAnswer {
            id: 0,
            answer_id: answer_id,
            is_correct: is_correct,
            answer: answer.to_string(),
            explanation: explanation.to_string(),
            created: now.clone(),
            updated: now,
        }       
    }
    
    pub fn insert(&mut self, conn: &Connection) -> Result<MultipleChoiceAnswer> {
        conn.query_row(
            INSERT_MULTIPLE_CHOICE,
            params![
                &self.answer_id,
                &self.is_correct,
                &self.answer,
                &self.explanation,
                &self.created,
                &self.updated
            ],
            |row| {
                Ok(MultipleChoiceAnswer {
                    id: row.get(0)?, 
                    answer_id: row.get(1)?,
                    is_correct: row.get(2)?,
                    answer: row.get(3)?,
                    explanation: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            },
        )
    }

    pub fn retrieve(answer_id: usize, conn: &Connection) -> Result<Vec<MultipleChoiceAnswer>> {
        let mut stmt = conn.prepare(RETRIEVE_MULTIPLE_CHOICE_ANSWER)?;

        let multiple_choice_answers: Result<Vec<MultipleChoiceAnswer>> = stmt
            .query_map([&answer_id], |row| {
                Ok(MultipleChoiceAnswer {
                    id: row.get(0)?, 
                    answer_id: row.get(1)?,
                    is_correct: row.get(2)?,
                    answer: row.get(3)?,
                    explanation: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            })?
            .collect();

        multiple_choice_answers
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            UPDATE_MULTIPLE_CHOICE_ANSWER, 
            params![
                &self.is_correct,
                &self.answer,
                &self.explanation,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated multiple choice answer");
                return Ok(())
            },
            Err(err) => {
                println!("[multiple_choice_answer::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_MULTIPLE_CHOICE_ANSWER, [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted multiple choice answer"); 
                return Ok(());
            },
            Err(err) => {
                println!("[multiple_choice_answer::delete] Error deleting multiple choice answer {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<MultipleChoiceAnswer> {
        conn.query_row(VIEW_MULTIPLE_CHOICE_ANSWER, &[&id], |row| {
            Ok(MultipleChoiceAnswer {
                id: row.get(0)?,
                answer_id: row.get(1)?,
                is_correct: row.get(2)?,
                answer: row.get(3)?,
                explanation: row.get(4)?,
                created: row.get(5)?,
                updated: row.get(6)?,
            })
        })
    }
    

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoice {
    pub answer_id: usize,
    pub created: String,
    pub updated: String
}

impl MultipleChoice {
    
    pub fn new() -> MultipleChoice {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      MultipleChoice {
            answer_id: 0,
            created: now.clone(),
            updated: now,
        }       
    }
    
    pub fn insert(&mut self, conn: &Connection) -> Result<MultipleChoice> {
        conn.query_row(
            INSERT_MULTIPLE_CHOICE,
            params![
                &self.created,
                &self.updated
            ],
            |row| {
                Ok(MultipleChoice {
                    answer_id: row.get(0)?,
                    created: row.get(1)?,
                    updated: row.get(2)?,
                })
            },
        )
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_MULTIPLE_CHOICE, [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted multiple choice"); 
                return Ok(());
            },
            Err(err) => {
                println!("[multiple_choice::delete] Error deleting multiple choice {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<MultipleChoice> {
        conn.query_row(VIEW_MULTIPLE_CHOICE, &[&id], |row| {
            Ok(MultipleChoice {
                answer_id: row.get(0)?,
                created: row.get(1)?,
                updated: row.get(2)?,
            })
        })
    }

    pub fn validate(&mut self, conn: &Connection, selected_answers: Vec<String>) -> Result<(bool, String)> {
 
        let mut stmt  = conn.prepare(VALIDATE_MULTIPLE_CHOICE)?;
        let correct_answers: Vec<String> = stmt
            .query_map(&[&self.answer_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        let correct_set: HashSet<String> = correct_answers.into_iter().collect();
        let selected_set: HashSet<String> = selected_answers.into_iter().collect();

        if selected_set == correct_set {
            Ok((true, String::from("Correct!")))
        } else {
            Ok((false, String::from("Missing one or more selected answers"))) 
        }

    }
  
}


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


#[tauri::command]
pub fn validate_multiple_choice_answer(db_path: &str, id: &str, answer: Vec<String>) -> (bool, String) {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return (false, e.to_string()); 
        }
    };

    match MultipleChoice::view(&conn, id) {
        Ok(mut multiple_choice) => {
            return multiple_choice.validate(&conn, answer).unwrap();
        }
        Err(e) => {
            let msg = format!("Error viewing ordered sequence answer for validation: {:?}", e);
            (false, msg.to_string())
        }
    }
}

use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

use crate::queries::answer::open_ended::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEndedAnswer {
    pub answer_id: usize,
    pub answer: String,
    pub explanation: String,
    pub keywords: Vec<String>,
    pub created: String,
    pub updated: String
}

impl OpenEndedAnswer {
    
    pub fn new(answer: &str, explanation: &str, keywords: Vec<&str>) -> OpenEndedAnswer {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      OpenEndedAnswer {
            answer_id: 0,
            answer: answer.to_string(),
            explanation: explanation.to_string(),
            keywords: keywords.iter().map(|k| k.to_string()).collect(),
            created: now.clone(),
            updated: now,
        }       
    }

    // Queries return the keywords as JSON (to_json) so keywords containing
    // commas or quotes round-trip intact
    pub fn parse_keywords(raw: String) -> Vec<String> {
        serde_json::from_str::<Option<Vec<String>>>(&raw)
            .ok()
            .flatten()
            .unwrap_or_default()
    }  
   
    pub fn insert(&mut self, conn: &Connection) -> Result<OpenEndedAnswer> {

        let keywords_literal = format!(
            "[{}]",
            self.keywords
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''")))
                .collect::<Vec<_>>()
                .join(", ")
        );

        conn.query_row(
            &INSERT_OPEN_ENDED_ANSWER.replace("{keywords}", &keywords_literal),
            params![
                &self.answer,
                &self.explanation,
                &self.created,
                &self.updated
            ],
            |row| {              
              let keywords_raw: String = row.get(3)?;
                Ok(OpenEndedAnswer {
                    answer_id: row.get(0)?,
                    answer: row.get(1)?,
                    explanation: row.get(2)?,
                    keywords: Self::parse_keywords(keywords_raw),
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            },
        )
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<OpenEndedAnswer>> {

        let mut stmt = conn.prepare(RETRIEVE_OPEN_ENDED_ANSWERS)?;

        let open_ended_answers: Result<Vec<OpenEndedAnswer>> = stmt
            .query_map([], |row| {
                let keywords_raw: String = row.get(3)?;
                Ok(OpenEndedAnswer {
                    answer_id: row.get(0)?,
                    answer: row.get(1)?,
                    explanation: row.get(2)?,
                    keywords: Self::parse_keywords(keywords_raw),
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            })?
            .collect();

        open_ended_answers
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        /* change date modified */
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let keywords_literal = format!(
            "[{}]",
            self.keywords
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''"))) // escape single quotes
                .collect::<Vec<_>>()
                .join(", ")
        );

        let result = conn.execute(
            &UPDATE_OPEN_ENDED_ANSWER.replace("{keywords}", &keywords_literal),
            params![
                &self.answer,
                &self.explanation,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("[open_ended_answer::update] Successfully updated open ended answer");
                return Ok(())
            },
            Err(err) => {
                println!("[open_ended_answer::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }


    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_OPEN_ENDED_ANSWER, [id]);
        match result {
            Ok(_) => {
                println!("[open_ended_answer::delete] Successfully deleted open ended answer"); 
                return Ok(());
            },
            Err(err) => {
                println!("[open_ended_answer::delete] Error deleting open ended answer {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<OpenEndedAnswer> {
        conn.query_row(VIEW_OPEN_ENDED_ANSWER, &[&id], |row| {
            let keywords_raw: String = row.get(3)?;
            Ok(OpenEndedAnswer {
                answer_id: row.get(0)?,
                answer: row.get(1)?,
                explanation: row.get(2)?,
                keywords: Self::parse_keywords(keywords_raw),
                created: row.get(4)?,
                updated: row.get(5)?
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<OpenEndedAnswer>> {
        let mut stmt = conn.prepare(&SEARCH_OPEN_ENDED_ANSWER)?;
        let open_ended_answers: Result<Vec<OpenEndedAnswer>> = stmt
            .query_map([search_term], |row| {
                let keywords_raw: String = row.get(3)?;
                Ok(OpenEndedAnswer {
                    answer_id: row.get(0)?,
                    answer: row.get(1)?,
                    explanation: row.get(2)?,
                    keywords: Self::parse_keywords(keywords_raw),
                    created: row.get(4)?,
                    updated: row.get(5)?
                })
            })?
            .collect();
        open_ended_answers
    }

    pub fn validate(&mut self, answer_text: &str) -> (bool, String) {

        let mut result = true;
        let mut explanation = String::new(); 
        let mut total_keywords = self.keywords.len();
        let mut keywords_matched = Vec::new();
        let mut keywords_missing = Vec::new();

        for keyword in &self.keywords {
            match answer_text.contains(&*keyword) {
                false => {
                    result = false;
                    keywords_missing.push(keyword);
                },
                true => {
                    keywords_matched.push(keyword);
                }
            }
        }

        if !result {
            explanation = format!("Answer is missing these keywords: {:?}", keywords_missing);
        }

        (result, explanation.to_string())
    }

}


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
            println!("[open_ended::edit_open_ended_answer] error {:?}", e);
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
            println!("[open_ended::view_open_ended_answers] error {:?}", e);
            return Vec::new();
        }
    };

    match OpenEndedAnswer::retrieve(&conn) {
        Ok(open_ended_answers) => open_ended_answers,
        Err(e) => {
            println!("[open_ended::view_open_ended_answers] error {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_open_ended_answer(db_path: &str, id: &str) -> Result<OpenEndedAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[open_ended::view_open_ended_answer] error {:?}", e);
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
            println!("[open_ended::search_open_ended_answers] error {:?}", e);
            return Vec::new();
        }
    };

    match OpenEndedAnswer::search(&conn, search_term) {
        Ok(open_ended_answer) => open_ended_answer,
        Err(e) => {
            println!("[open_ended::search_open_ended_answers] Error searching open ended answer: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_open_ended_answer(db_path: &str, id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[open_ended::delete_open_ended_answer] error {:?}", e);
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

#[tauri::command]
pub fn validate_open_ended_answer(id: &str, answer: &str, db_path: &str) -> (bool, String) {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[open_ended::validate_open_ended_answer] error {:?}", e);
            return (false, e.to_string()); 
        }
    };

    match OpenEndedAnswer::view(&conn, id) {
        Ok(mut open_ended) => {
            return open_ended.validate(answer);
        }
        Err(e) => {
            let msg = format!("Error viewing open ended answer for validation: {:?}", e);
            (false, msg.to_string())
        }
    }
}



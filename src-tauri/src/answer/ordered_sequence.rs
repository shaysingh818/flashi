use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

use crate::queries::answer::ordered_sequence::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderedSequenceAnswer {
    pub answer_id: usize,
    pub sequence: Vec<String>,
    pub explanation: String,
    pub created: String,
    pub updated: String
}

impl OrderedSequenceAnswer {
    
    pub fn new(sequence: Vec<&str>, explanation: &str) -> OrderedSequenceAnswer {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      OrderedSequenceAnswer {
            answer_id: 0,
            sequence: sequence.iter().map(|k| k.to_string()).collect(),
            explanation: explanation.to_string(),
            created: now.clone(),
            updated: now,
        }       
    }

    // Queries return the sequence as JSON (to_json) so items containing
    // commas or quotes round-trip intact
    pub fn parse_sequence(raw: String) -> Vec<String> {
        serde_json::from_str::<Option<Vec<String>>>(&raw)
            .ok()
            .flatten()
            .unwrap_or_default()
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<OrderedSequenceAnswer> {

        let sequence_literal = format!(
            "[{}]",
            self.sequence
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''")))
                .collect::<Vec<_>>()
                .join(", ")
        );
 
        conn.query_row(
            &INSERT_ORDERED_SEQUENCE.replace("{sequence}", &sequence_literal),
            params![
                &self.explanation,
                &self.created,
                &self.updated
            ],
            |row| {              
                let sequence_raw: String = row.get(1)?;
                Ok(OrderedSequenceAnswer {
                    answer_id: row.get(0)?,
                    sequence: Self::parse_sequence(sequence_raw),
                    explanation: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            },
        )

    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<OrderedSequenceAnswer>> {

        let mut stmt = conn.prepare(RETRIEVE_ORDERED_SEQUENCE)?;

        let ordered_sequence_answers: Result<Vec<OrderedSequenceAnswer>> = stmt
            .query_map([], |row| {
                let sequence_raw: String = row.get(1)?;
                Ok(OrderedSequenceAnswer {
                    answer_id: row.get(0)?,
                    sequence: Self::parse_sequence(sequence_raw),
                    explanation: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();

        ordered_sequence_answers
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let sequence_literal = format!(
            "[{}]",
            self.sequence
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''"))) // escape single quotes
                .collect::<Vec<_>>()
                .join(", ")
        );

        let result = conn.execute(
            &UPDATE_ORDERED_SEQUENCE.replace("{sequence}", &sequence_literal),
            params![
                &self.explanation,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("[ordered_sequence_answer::update] Successfully updated ordered sequence answer");
                return Ok(())
            },
            Err(err) => {
                println!("[ordered_sequence_answer::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }


    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_ORDERED_SEQUENCE, [id]);
        match result {
            Ok(_) => {
                println!("[ordered_sequence_answer::delete] Successfully deleted ordered sequence answer"); 
                return Ok(());
            },
            Err(err) => {
                println!("[ordered_sequence_answer::delete] Error deleting ordered sequence answer {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<OrderedSequenceAnswer> {
        conn.query_row(VIEW_ORDERED_SEQUENCE, &[&id], |row| {
            let sequence_raw: String = row.get(1)?;
            Ok(OrderedSequenceAnswer {
                answer_id: row.get(0)?,
                sequence: Self::parse_sequence(sequence_raw),
                explanation: row.get(2)?,
                created: row.get(3)?,
                updated: row.get(4)?,
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<OrderedSequenceAnswer>> {
        
        let mut stmt = conn.prepare(&SEARCH_ORDERED_SEQUENCE)?;
        let ordered_sequence_answers: Result<Vec<OrderedSequenceAnswer>> = stmt
            .query_map([search_term], |row| {
                let sequence_raw: String = row.get(1)?;
                Ok(OrderedSequenceAnswer {
                    answer_id: row.get(0)?,
                    sequence: Self::parse_sequence(sequence_raw),
                    explanation: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();
        ordered_sequence_answers
    }

    pub fn validate(&mut self, answer_sequence: Vec<String>) -> (bool, String) {

        let mut result = true;
        let mut explanation = String::new();
        
        if answer_sequence == self.sequence {
            (true, String::from("Correct!"))
        } else {
            (false, format!("{:?}", self.sequence))
        }

    }

}


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
            println!("[ordered_sequence::edit_ordered_sequence_answer] error {:?}", e);
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
            println!("[ordered_sequence::view_ordered_sequence_answers] error {:?}", e);
            return Vec::new();
        }
    };

    match OrderedSequenceAnswer::retrieve(&conn) {
        Ok(ordered_sequences) => ordered_sequences,
        Err(e) => {
            println!("[ordered_sequence::view_ordered_sequence_answers] error {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_ordered_sequence_answer(db_path: &str, id: &str) -> Result<OrderedSequenceAnswer, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[ordered_sequence::view_ordered_sequence_answer] error {:?}", e);
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
            println!("[ordered_sequence::search_ordered_sequence_answers] error {:?}", e);
            return Vec::new();
        }
    };

    match OrderedSequenceAnswer::search(&conn, search_term) {
        Ok(ordered_sequences) => ordered_sequences,
        Err(e) => {
            println!("[ordered_sequence::search_ordered_sequence_answers] Error searching ordered sequences: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_ordered_sequence_answer(db_path: &str, id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[ordered_sequence::delete_ordered_sequence_answer] error {:?}", e);
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
            println!("[ordered_sequence::validate_ordered_sequence_answer] error {:?}", e);
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

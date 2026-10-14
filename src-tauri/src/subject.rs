use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

use crate::queries::subject::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub created: String,
    pub updated: String,
}

impl Subject {

    pub fn new(name: &str, description: &str) -> Subject {

        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      
        Subject {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            INSERT_SUBJECT,
            params![
                &self.name,
                &self.description,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("[subject::insert] Successfully inserted subject");
                Ok(())
            },
            Err(err) => {
                println!("[subject::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<Subject>> {
        
        let mut stmt = conn.prepare(VIEW_SUBJECTS)?;

        let subjects: Result<Vec<Subject>> = stmt
            .query_map([], |row| {
                Ok(Subject {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();
        subjects
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            UPDATE_SUBJECT,
            params![
                &self.name,
                &self.description,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("[subject::update] Successfully updated subject");
                Ok(())
            },
            Err(err) => {
                println!("[subject::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_SUBJECT, [id]);
        match result {
            Ok(_) => {
                println!("[subject::delete] Successfully deleted subject");
                Ok(())
            },
            Err(err) => {
                println!("[subject::delete] Error deleting subject {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<Subject> {
        conn.query_row(
            VIEW_SUBJECT,
            &[&id],
            |row| {
                Ok(Subject {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            }
        )
    }

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Subject>> {
        
        let mut stmt = conn.prepare(&SEARCH_SUBJECT)?;

        let subjects: Result<Vec<Subject>> = stmt
            .query_map([search_term], |row| {
                Ok(Subject {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();
        subjects
    }

    pub fn metrics(conn: &Connection, id: &str) -> Result<(usize, usize, f64)> {
        conn.query_row(
            SUBJECT_METRICS,
            &[&id, &id, &id],
            |row| {
                let question_count: Option<i64> = row.get(0)?;
                let total_reviewed: Option<i64> = row.get(1)?;
                let total_accuracy: Option<f64> = row.get(2)?; 
                Ok((
                    total_reviewed.unwrap_or(0) as usize,                    
                    question_count.unwrap_or(0) as usize,
                    total_accuracy.unwrap_or(0.0) as f64
                ))
            }
        )
    }
}

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
            println!("[subject::edit_subject] error {:?}", e);
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
            println!("[subject::view_subjects] error {:?}", e);
            return Vec::new();
        }
    };

    match Subject::retrieve(&conn) {
        Ok(subjects) => subjects,
        Err(e) => {
            println!("[subject::view_subjects] error {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_subject(db_path: &str, subject_id: &str) -> Result<Subject, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[subject::view_subject] error {:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match Subject::view(&conn, subject_id) {
        Ok(subject) => {
            return Ok(subject);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving subject with id: {:?} Cause: {:?}", subject_id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_subjects(db_path: &str, search_term: &str) -> Vec<Subject> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[subject::search_subjects] error {:?}", e);
            return Vec::new();
        }
    };

    match Subject::search(&conn, search_term) {
        Ok(subjects) => subjects,
        Err(e) => {
            println!("[subject::search_subjects] Error searching subjects: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_subject(db_path: &str, subject_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[subject::delete_subject] error {:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Subject::view(&conn, subject_id) {
        Ok(mut subject) => {
            let _ = subject.delete(&conn, subject_id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving subject with id: {:?} Cause: {:?}", subject_id, e);
        }
    }
}

#[tauri::command]
pub fn subject_metrics(db_path: &str, subject_id: &str) -> (usize, usize, f64) {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[subject::subject_metrics] error {:?}", e);
            return (0, 0, 0.0); 
        }
    };

    match Subject::metrics(&conn, subject_id) {
        Ok((question_count, total_reviewed, total_accuracy)) => {
            (question_count, total_reviewed, total_accuracy)
        }
        Err(e) => {
            println!("[subject::subject_metrics] Error retrieving subject with id: {:?} Cause: {:?}", subject_id, e);
            return (0, 0, 0.0)
        }
    }
}

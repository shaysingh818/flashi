use chrono;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result};
use crate::flashi_api::deck::Deck; 
use crate::flashi_api::queries::session::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCorrectQuestion {
    pub correct: usize,
    pub reviewed: usize,
    pub raw: String,
    pub updated: String,
    pub correct_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionIncorrectQuestion {
    pub incorrect: usize,
    pub reviewed: usize,
    pub raw: String,
    pub updated: String,
    pub correct_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub created: String,
    pub updated: String,
}

impl Session {

    pub fn new(name: &str, description: &str) -> Session {
        
        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Session {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            INSERT_SESSION,
            params![
                &self.name,
                &self.description,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted session");
                Ok(())
            },
            Err(err) => {
                println!("[session::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<Session>> {

        let mut stmt = conn.prepare(RETRIEVE_SESSIONS)?;

        let sessions: Result<Vec<Session>> = stmt
            .query_map([], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();
        sessions
    }

    pub fn retrieve_session_decks(conn: &Connection, session_id: usize) -> Result<Vec<Deck>> {

        let mut stmt = conn.prepare(RETRIEVE_SESSION_DECKS)?;

        let decks: Result<Vec<Deck>> = stmt
            .query_map(params![session_id], |row| {
                let tags_raw: String = row.get(3)?;
                Ok(Deck {
                    id: row.get(0)?,
                    subject_id: row.get(1)?,
                    name: row.get(2)?,
                    tags: Deck::parse_tags(tags_raw),
                    description: row.get(4)?,
                    reviewed: row.get(5)?,
                    created: row.get(6)?,
                    updated: row.get(7)?,
                })
            })?
            .collect();

        decks
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            UPDATE_SESSION, 
            params![
                &self.name,
                &self.description,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated session");
                Ok(())
            },
            Err(err) => {
                println!("[session::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {

        let session_questions = self.delete_session_review_questions(conn, id);
        match session_questions {
            Ok(_) => {                
                println!("[session::delete] Successfully deleted session review questions as part of delete session routine");
            }
            Err(err) => {                
                println!("[session::delete] Error deleting session review questions as part of the delete session routine {:?}", err);
            }
        }

        let session_metrics = self.delete_session_review_metrics(conn, id);
        match session_metrics {
            Ok(_) => {                
                println!("[session::delete] Successfully deleted session review questions as part of delete session routine");
            }
            Err(err) => {                
                println!("[session::delete] Error deleting session review questions as part of the delete session routine {:?}", err);
            }
        }
        
        let session_reviews = self.delete_session_reviews(conn, id);
        match session_reviews {
            Ok(_) => {                
                println!("[session::delete] Successfully deleted session reviews as part of delete session routine");
            }
            Err(err) => {                
                println!("[session::delete] Error deleting session reviews as part of the delete session routine {:?}", err);
            }
        }
        
        let decks = self.delete_decks(conn, id);
        match decks {
            Ok(_) => {                
                println!("[session::delete] Successfully deleted session decks as part of delete session routine");
            }
            Err(err) => {                
                println!("[session::delete] Error deleting decks as part of session delete routine: {:?}", err);
            }
        };
        
        let result = conn.execute("DELETE FROM SESSION WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("[session::delete] Successfully deleted session");
                Ok(())
            },
            Err(err) => {
                println!("[session::delete] Error deleting session {:?}", err);
                Err(err)
            }
        }
    }
    
    pub fn delete_session_reviews(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_SESSION, [id]);
        match result {
            Ok(_) => {
                println!("[session::delete_session_reviews] Successfully removed session review's from existing session");
                Ok(())
            }
            Err(err) => {
                println!(
                    "[session::delete_session_reviews] Error deleting session review metrics {:?}",
                    err
                );
                Err(err)
            }
        }
    }
    
    pub fn delete_session_review_questions(&mut self, conn: &Connection, id: &str) -> Result<()> {
 
        let result = conn.execute(DELETE_SESSION_REVIEW_QUESTIONS, [id]);
        match result {
            Ok(_) => {
                println!("[session::delete_session_review_questions] Successfully removed session review questions from existing session");
                Ok(())
            }
            Err(err) => {
                println!(
                    "[session::delete_session_review_questions] Error deleting session review questions from existing session {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    pub fn delete_session_review_metrics(&mut self, conn: &Connection, id: &str) -> Result<()> {
 
        let result = conn.execute(DELETE_SESSION_REVIEW_QUESTIONS, [id]);
        match result {
            Ok(_) => {
                println!("[session::delete_session_review_metrics] Successfully removed session review metrics from existing session");
                Ok(())
            }
            Err(err) => {
                println!(
                    "[session::delete_session_review_metrics] Error deleting session review metrics from existing session {:?}",
                    err
                );
                Err(err)
            }
        }
    }
    
    pub fn delete_decks(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute(DELETE_SESSION_DECKS, [id]);
        match result {
            Ok(_) => {
                println!("[session::delete_decks] Successfully removed all decks associated with session");
                Ok(())
            }
            Err(err) => {
                println!(
                    "[session::delete_decks] Error deleting decks associated with session {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<Session> {
        conn.query_row(
            VIEW_SESSION,
            &[&id],
            |row| {
                Ok(Session {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            }
        )
    }

    pub fn add_deck_to_session(&mut self, conn: &Connection, deck_id: &str) -> Result<String> {
        
        let result = conn.execute(
            ADD_DECK_TO_SESSION,
            params![
                &self.id,
                deck_id,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully added deck to session");
                Ok(("Success").to_string())
            },
            Err(duckdb::Error::DuckDBFailure(err, msg)) => {
                let msg_str = msg.as_deref().unwrap_or("");
                if msg_str.contains("violates unique constraint") {
                    println!("Deck already associated with session, skipping");
                    Ok("Duplicate".to_string())
                } else {
                    println!("[session::add_deck_to_session] duckdb error {:?}", msg_str);
                    Err(duckdb::Error::DuckDBFailure(err, msg))
                }
            },
            Err(err) => {
                println!("[session::add_deck_to_session] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn remove_deck_from_session(&mut self, conn: &Connection, deck_id: &str) -> Result<String> {
        
        let result = conn.execute(
            REMOVE_DECK_FROM_SESSION,
            params![
                &self.id,
                deck_id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully removed deck from session");
                Ok(("Success").to_string())
            },
            Err(err) => {
                println!("[session::remove_deck_from_session] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Session>> {

        let mut stmt = conn.prepare(&SEARCH_SESSION)?;

        let sessions: Result<Vec<Session>> = stmt
            .query_map([search_term], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();
        sessions
    }


    pub fn session_progress_status(conn: &Connection, id: &str) -> Result<(usize, usize)> {

        conn.query_row(
            SESSION_PROGRESS_STATUS,
            &[&id],
            |row| {
                let completed: Option<i64> = row.get(0)?; 
                let total: Option<i64> = row.get(1)?; 
                Ok((completed.unwrap_or(0) as usize, total.unwrap_or(0) as usize))
            }
        )
    }

    pub fn metrics(conn: &Connection, id: &str) -> Result<(f64, usize, usize, usize, usize)> {

        conn.query_row(
            SESSION_METRICS,
            &[&id],
            |row| {
                let avg_accuracy: Option<f64> = row.get(0)?; 
                let total_reviewed: Option<i64>  = row.get(1)?; 
                let total_review_time: Option<i64> = row.get(2)?; 
                let total_correct: Option<i64> = row.get(3)?; 
                let total_incorrect: Option<i64> = row.get(4)?; 
                Ok((
                    avg_accuracy.unwrap_or(0.0),
                    total_reviewed.unwrap_or(0) as usize,  
                    total_review_time.unwrap_or(0) as usize, 
                    total_correct.unwrap_or(0) as usize, 
                    total_incorrect.unwrap_or(0) as usize, 
                ))
            }
        )
    }

    pub fn question_breakdown(conn: &Connection, id: &str) -> Result<Vec<(String, i64)>> {

        let mut stmt = conn.prepare(SESSION_QUESTION_BREAKDOWN)?; 
        let rows = stmt.query_map(&[&id], |row| {
            let answer_type: String = row.get(0)?; 
            let count: i64 = row.get(1)?; 
            Ok((answer_type, count))
        })?; 

        rows.collect()
    }
    
    pub fn daily_accuracy_history(conn: &Connection, id: &str) -> Result<Vec<(String, f64)>> {

        let cutoff = (Utc::now() - Duration::days(30)).date_naive();
        let cutoff_str = cutoff.to_string();

        let mut stmt = conn.prepare(SESSION_DAILY_ACCURACY_HISTORY)?; 
        let rows = stmt.query_map(params![id, cutoff_str], |row| {
            let day_date: String = row.get(0)?;
            let accuracy: f64 = row.get(1)?;
            Ok((day_date, accuracy))
        })?;
        
        rows.collect()
    }

    pub fn correct_incorrect_questions(conn: &Connection, id: &str) -> Result<(Vec<SessionCorrectQuestion>, Vec<SessionIncorrectQuestion>)> {

        let mut stmt = conn.prepare(SESSION_CORRECT_QUESTIONS)?; 
        let correct_questions: Result<Vec<SessionCorrectQuestion>> = stmt
            .query_map([id], |row| {
                Ok(SessionCorrectQuestion {
                    correct: row.get(0)?,
                    reviewed: row.get(1)?,
                    raw: row.get(2)?,
                    updated: row.get(3)?,
                    correct_ratio: row.get(4)?,
                })
            })?
            .collect();
               
        let mut stmt = conn.prepare(SESSION_INCORRECT_QUESTIONS)?; 
        let incorrect_questions: Result<Vec<SessionIncorrectQuestion>> = stmt
            .query_map([id], |row| {
                Ok(SessionIncorrectQuestion {
                    incorrect: row.get(0)?,
                    reviewed: row.get(1)?,
                    raw: row.get(2)?,
                    updated: row.get(3)?,
                    correct_ratio: row.get(4)?,
                })
            })?
            .collect(); 

        Ok((correct_questions?, incorrect_questions?))
    }
    
}


#[tauri::command]
pub fn create_session(name: &str, description: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    let mut session: Session = Session::new(name, description);

    match session.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert session: {}", e),
    }
}

#[tauri::command]
pub fn edit_session(session_id: &str, name: &str, description: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
          
            session.name = name.to_string();            
            session.description = description.to_string();
            
            match session.update(&conn, session_id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update session: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating session: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_sessions(db_path: &str) -> Vec<Session> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Session::retrieve(&conn) {
        Ok(sessions) => sessions,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_session(db_path: &str, session_id: &str) -> Result<Session, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match Session::view(&conn, session_id) {
        Ok(session) => {
            return Ok(session);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving session with id: {:?} Cause: {:?}", session_id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_sessions(db_path: &str, search_term: &str) -> Vec<Session> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Session::search(&conn, search_term) {
        Ok(sessions) => sessions,
        Err(e) => {
            println!("Error searching sessions: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_session(db_path: &str, session_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
            let _ = session.delete(&conn, session_id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving subject with id: {:?} Cause: {:?}", session_id, e);
        }
    }
}


#[tauri::command]
pub fn view_session_decks(db_path: &str, session_id: &str) -> Vec<Deck> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    let session_id_usize: usize = session_id.parse::<usize>().unwrap();

    match Session::retrieve_session_decks(&conn, session_id_usize) {
        Ok(decks) => decks,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}


#[tauri::command]
pub fn add_deck_to_session(db_path: &str, session_id: &str, deck_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
            let result = session.add_deck_to_session(&conn, deck_id).unwrap();
            return result;
        }
        Err(e) => {
            return format!("Error adding deck to session with id: {:?} Cause: {:?}", session_id, e);
        }
    }
}


#[tauri::command]
pub fn remove_deck_from_session(db_path: &str, session_id: &str, deck_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Session::view(&conn, session_id) {
        Ok(mut session) => {
            let result = session.remove_deck_from_session(&conn, deck_id).unwrap();
            return result;
        }
        Err(e) => {
            return format!("Error adding deck to session with id: {:?} Cause: {:?}", session_id, e);
        }
    }
}


#[tauri::command]
pub fn session_progress_status(db_path: &str, session_id: &str) -> (usize, usize) {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return (0, 0);
        }
    };

    match Session::session_progress_status(&conn, session_id) {
        Ok((completed, total)) => {
            return (completed, total);
        }
        Err(e) => {
            println!("Error retrieving session progress status: {:?} Cause: {:?}", session_id, e);
            return (0, 0); 
        }
    }
}

#[tauri::command]
pub fn session_metrics(db_path: &str, session_id: &str) -> Result<(f64, usize, usize, usize, usize), String> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    };

    match Session::metrics(&conn, session_id) {
        Ok((avg_accuracy, total_reviewed, total_review_time, total_correct, total_incorrect)) => {
            Ok((avg_accuracy, total_reviewed, total_review_time, total_correct, total_incorrect))
        }
        Err(e) => {
            println!("Error retrieving session progress status: {:?} Cause: {:?}", session_id, e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn session_question_breakdown(db_path: &str, session_id: &str) -> Result<Vec<(String, i64)>, String> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string()); 
        }
    };

    match Session::question_breakdown(&conn, session_id) {
        Ok(rows) => Ok(rows), 
        Err(e) => {
            println!("Error retrieving session progress status: {:?} Cause: {:?}", session_id, e);
            return Err(e.to_string()) 
        }
    }
}

#[tauri::command]
pub fn session_daily_accuracy_history(db_path: &str, session_id: &str) -> Result<Vec<(String, f64)>, String> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string()); 
        }
    };

    match Session::daily_accuracy_history(&conn, session_id) {
        Ok(rows) => Ok(rows), 
        Err(e) => {
            println!("Error retrieving session daily accuracy history: {:?} Cause: {:?}", session_id, e);
            return Err(e.to_string()) 
        }
    }
}

#[tauri::command]
pub fn session_correct_incorrect_questions(db_path: &str, session_id: &str) -> Result<(Vec<SessionCorrectQuestion>, Vec<SessionIncorrectQuestion>), String> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string()); 
        }
    };

    match Session::correct_incorrect_questions(&conn, session_id) {
        Ok((correct_questions, incorrect_questions)) => Ok((correct_questions, incorrect_questions)), 
        Err(e) => {
            println!("Error retrieving session correct and incorrect questions: {:?} Cause: {:?}", session_id, e);
            return Err(e.to_string()) 
        }
    }
}

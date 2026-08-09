use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result};
use crate::flashi_api::deck::Deck; 


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
            "INSERT INTO SESSION
                (NAME, DESCRIPTION, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4)",
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
        let mut stmt = conn.prepare("SELECT ID, NAME, DESCRIPTION, CREATED::VARCHAR, UPDATED::VARCHAR FROM SESSION")?;

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

        let mut stmt = conn.prepare(
            "SELECT d.ID, d.SUBJECT_ID, d.NAME, d.TAGS::VARCHAR, d.DESCRIPTION, 
                    d.CREATED::VARCHAR, d.UPDATED::VARCHAR
             FROM DECK d
             INNER JOIN SESSION_DECK sd ON sd.DECK_ID = d.ID
             WHERE sd.SESSION_ID = ?1"
        )?;

        let decks: Result<Vec<Deck>> = stmt
            .query_map(params![session_id], |row| {
                let tags_raw: String = row.get(3)?;
                Ok(Deck {
                    id: row.get(0)?,
                    subject_id: row.get(1)?,
                    name: row.get(2)?,
                    tags: Deck::parse_tags(tags_raw),
                    description: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            })?
            .collect();

        decks
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            "UPDATE SESSION
                SET NAME=?1,
                    DESCRIPTION=?2,
                    UPDATED=?3
                WHERE ID=?4",
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
        let result = conn.execute("DELETE FROM SESSION_REVIEW WHERE SESSION_ID=?", [id]);
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

        let query = "
            DELETE FROM SESSION_REVIEW_QUESTION 
            WHERE SESSION_REVIEW_ID IN (
                SELECT ID FROM SESSION_REVIEW WHERE SESSION_ID = ?
            )
        ";
        
        let result = conn.execute(query, [id]);
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

        let query = "
            DELETE FROM SESSION_REVIEW_METRICS 
            WHERE SESSION_REVIEW_ID IN (
                SELECT ID FROM SESSION_REVIEW WHERE SESSION_ID = ?
            )
        ";
        
        let result = conn.execute(query, [id]);
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
        let result = conn.execute("DELETE FROM SESSION_DECK WHERE SESSION_ID=?", [id]);
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
            "SELECT ID, NAME, DESCRIPTION, CREATED::VARCHAR, UPDATED::VARCHAR FROM SESSION WHERE ID = ?",
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
            "INSERT INTO SESSION_DECK
                (SESSION_ID, DECK_ID, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4)",
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
            "DELETE FROM SESSION_DECK
                WHERE SESSION_ID = ?1 AND DECK_ID = ?2",
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
        let query = format!(
            "SELECT ID, NAME, DESCRIPTION, CREATED::VARCHAR, UPDATED::VARCHAR FROM SESSION WHERE NAME LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;

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



}

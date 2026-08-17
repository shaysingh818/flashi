use chrono;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result};
use crate::flashi_api::deck::Deck; 


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
            "SELECT d.ID, d.SUBJECT_ID, d.NAME, d.TAGS::VARCHAR, d.DESCRIPTION, d.REVIEWED, 
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


    pub fn session_progress_status(conn: &Connection, id: &str) -> Result<(usize, usize)> {

        let query = "
            WITH SESSION_QUESTIONS AS (
                SELECT * FROM SESSION_REVIEW_QUESTION 
                WHERE SESSION_REVIEW_ID IN (
                    SELECT ID FROM SESSION_REVIEW WHERE COMPLETE != TRUE AND SESSION_ID = ?)
            )
            SELECT 
                COUNT_IF(ACTIVE = FALSE) AS COMPLETED,
                COUNT(*) AS TOTAL
            FROM SESSION_QUESTIONS; 
        "; 
        
        conn.query_row(
            query,
            &[&id],
            |row| {
                let completed: Option<i64> = row.get(0)?; 
                let total: Option<i64> = row.get(1)?; 
                Ok((completed.unwrap_or(0) as usize, total.unwrap_or(0) as usize))
            }
        )
    }

    pub fn metrics(conn: &Connection, id: &str) -> Result<(f64, usize, usize, usize, usize)> {

        let query = "
            SELECT 
                SUM(ACCURACY) / COUNT(*) AS AVG_ACCURACY,
                SUM(TOTAL_REVIEW_COUNT) AS TOTAL_REVIEWED,
                SUM(REVIEW_TIME) AS TOTAL_REVIEW_TIME,
                SUM(QUESTIONS_CORRECT) AS TOTAL_CORRECT,
                SUM(QUESTIONS_INCORRECT) AS TOTAL_INCORRECT
            FROM SESSION_REVIEW_METRICS
            WHERE SESSION_REVIEW_ID IN (SELECT ID FROM SESSION_REVIEW WHERE SESSION_ID = ?)
        ";

        conn.query_row(
            query,
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

        let query = "
            WITH ANSWER_TYPE_GROUPING AS (
               	SELECT 
              		ANS_TYPE.NAME AS ANSWER_TYPE
               	FROM SESSION_REVIEW_QUESTION AS SRQ
               	LEFT JOIN QUESTION AS Q
               	ON Q.ID = SRQ.QUESTION_ID
               	LEFT JOIN ANSWER_TYPE AS ANS_TYPE
               	ON Q.ANSWER_TYPE_ID = ANS_TYPE.ID
               	WHERE SRQ.SESSION_REVIEW_ID IN (SELECT ID FROM SESSION_REVIEW WHERE SESSION_ID = ?)
           	)
            SELECT
                ANSWER_TYPE,
                COUNT(ANSWER_TYPE) AS ANSWER_TYPE_COUNT
            FROM ANSWER_TYPE_GROUPING
            GROUP BY ANSWER_TYPE ORDER BY ANSWER_TYPE_COUNT DESC
        ";

        let mut stmt = conn.prepare(query)?; 
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

        let query = "
            SELECT 
                CAST(DATE_TRUNC('day', CREATED) AS VARCHAR) AS DAY_DATE,
                AVG(ACCURACY) AS AVG_ACCURACY
            FROM SESSION_REVIEW_METRICS
            WHERE SESSION_REVIEW_ID IN (SELECT ID FROM SESSION_REVIEW WHERE SESSION_ID = ?)
            AND CREATED >= ?
            GROUP BY DATE_TRUNC('day', CREATED)
            ORDER BY DATE_TRUNC('day', CREATED)
        ";

        let mut stmt = conn.prepare(query)?; 
        let rows = stmt.query_map(params![id, cutoff_str], |row| {
            let day_date: String = row.get(0)?;
            let accuracy: f64 = row.get(1)?;
            Ok((day_date, accuracy))
        })?;
        
        rows.collect()
    }

    pub fn correct_incorrect_questions(conn: &Connection, id: &str) -> Result<(Vec<SessionCorrectQuestion>, Vec<SessionIncorrectQuestion>)> {

        let correct_query = "
            SELECT 
               	Q.CORRECT,
               	Q.REVIEWED,
               	Q.RAW,
               	Q.UPDATED::VARCHAR,
               	(Q.CORRECT / Q.REVIEWED) AS CORRECT_RATIO
            FROM SESSION_REVIEW_QUESTION AS SRQ
            LEFT JOIN QUESTION AS Q
            ON Q.ID = SRQ.QUESTION_ID
            WHERE SESSION_REVIEW_ID IN (SELECT ID FROM SESSION_REVIEW WHERE SESSION_ID = ?)
            ORDER BY CORRECT_RATIO DESC
        ";

        let incorrect_query = "
            SELECT 
               	Q.INCORRECT,
               	Q.REVIEWED,
               	Q.RAW,
               	Q.UPDATED::VARCHAR,
               	(Q.INCORRECT / Q.REVIEWED) AS INCORRECT_RATIO
            FROM SESSION_REVIEW_QUESTION AS SRQ
            LEFT JOIN QUESTION AS Q
            ON Q.ID = SRQ.QUESTION_ID
            WHERE SESSION_REVIEW_ID IN (SELECT ID FROM SESSION_REVIEW WHERE SESSION_ID = ?)
            ORDER BY INCORRECT_RATIO DESC
        ";

        let mut stmt = conn.prepare(correct_query)?; 
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
               
        let mut stmt = conn.prepare(incorrect_query)?; 
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

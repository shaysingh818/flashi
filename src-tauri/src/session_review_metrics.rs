use chrono;
use duckdb::{params, Connection, OptionalExt, Result};
use serde::{Deserialize, Serialize};

use crate::queries::session_review_metrics::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReviewMetrics {
    pub id: usize,
    pub session_review_id: usize,
    pub questions_correct: usize,
    pub questions_incorrect: usize,
    pub accuracy: f64,
    pub total_review_count: usize,
    pub number_of_cycles: usize,
    pub review_time: f64,
    pub created: String,
}

impl SessionReviewMetrics {

    pub fn new(session_review_id: usize, conn: &Connection) -> Result<SessionReviewMetrics> {
        let now = chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        conn.query_row(SESSION_REVIEW_METRICS_INSTANCE, &[&session_review_id, &session_review_id], |row| {
            Ok(SessionReviewMetrics {
                id: 0,
                session_review_id: row.get(0)?,
                questions_correct: row.get(1)?,
                questions_incorrect: row.get(2)?,
                accuracy: row.get(3)?,
                total_review_count: row.get(4)?,
                number_of_cycles: row.get(5)?,
                review_time: row.get(6)?,
                created: now,
            })
        })
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            INSERT_SESSION_REVIEW_METRICS,
            params![
                &self.session_review_id,
                &self.questions_correct,
                &self.questions_incorrect,
                &self.accuracy,
                &self.total_review_count,
                &self.number_of_cycles,
                &self.review_time,
                &self.created,
            ],
        );

        match result {
            Ok(_) => {
                println!("[session_review_metrics::insert] Successfully inserted session review metrics");
                Ok(())
            }
            Err(err) => {
                println!("[session_review_metrics::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<SessionReviewMetrics>> {
        let mut stmt = conn.prepare("SELECT * FROM SESSION_REVIEW_METRICS")?;

        let sessions: Result<Vec<SessionReviewMetrics>> = stmt
            .query_map([], |row| {
                Ok(SessionReviewMetrics {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?,
                    questions_correct: row.get(2)?,
                    questions_incorrect: row.get(3)?,
                    accuracy: row.get(4)?,
                    total_review_count: row.get(5)?,
                    number_of_cycles: row.get(6)?,
                    review_time: row.get(7)?,
                    created: row.get(8)?,
                })
            })?
            .collect();
        sessions
    }
    
    pub fn session_metrics_history(conn: &Connection, session_id: &str) -> Result<Vec<SessionReviewMetrics>> {
 
        let mut stmt = conn.prepare(SESSION_REVIEW_METRICS_HISTORY)?;

        let sessions: Result<Vec<SessionReviewMetrics>> = stmt
            .query_map([session_id], |row| {
                Ok(SessionReviewMetrics {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?,
                    questions_correct: row.get(2)?,
                    questions_incorrect: row.get(3)?,
                    accuracy: row.get(4)?,
                    total_review_count: row.get(5)?,
                    number_of_cycles: row.get(6)?,
                    review_time: row.get(7)?,
                    created: row.get(8)?,
                })
            })?
            .collect();
        sessions
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {

        let result = conn.execute(DELETE_SESSION_REVIEW_METRICS, [id]);
        match result {
            Ok(_) => {
                println!("[session_review_metrics::delete] Successfully deleted session review metrics");
                Ok(())
            }
            Err(err) => {
                println!(
                    "[session_review_metrics::delete] Error deleting session review metrics {:?}",
                    err
                );
                Err(err)
            }
        }
    }
    
    pub fn view(conn: &Connection, id: &str) -> Result<SessionReviewMetrics> {
        conn.query_row(
            VIEW_SESSION_REVIEW_METRICS,
            &[&id],
            |row| {
                Ok(SessionReviewMetrics {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?,
                    questions_correct: row.get(2)?,
                    questions_incorrect: row.get(3)?,
                    accuracy: row.get(4)?,
                    total_review_count: row.get(5)?,
                    number_of_cycles: row.get(6)?,
                    review_time: row.get(7)?,
                    created: row.get(8)?,
                })
            },
        )
    }
}


#[tauri::command]
pub fn retrieve_session_metrics_history(db_path: &str, session_id: &str) -> Vec<SessionReviewMetrics> {
       
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("[session_review_metrics::retrieve_session_metrics_history] error {:?}", e);
            return Vec::new();
        }
    };
    
    match SessionReviewMetrics::session_metrics_history(&conn, session_id) {
        Ok(session_metrics) => session_metrics,
        Err(e) => {
            println!("[session_review_metrics::retrieve_session_metrics_history] error {:?}", e);
            Vec::new()
        }
    }

}

#[tauri::command]
pub fn generate_session_review_metrics(session_review_id: &str, db_path: &str) -> String {
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    let session_review_id_usize: usize = session_review_id.parse::<usize>().unwrap();

    let mut session_review_metrics: SessionReviewMetrics =
        SessionReviewMetrics::new(session_review_id_usize, &conn).unwrap();

    match session_review_metrics.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert session review: {}", e),
    }
}

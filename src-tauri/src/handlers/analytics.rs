use duckdb::{Connection}; 
use crate::flashi_api::analytics::*;


#[tauri::command]
pub fn review_time_metrics(db_path: &str) -> (f64, f64, f64) {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return (0.0, 0.0, 0.0); 
        }
    };

    match review_time(&conn) {
        Ok((avg_daily_review_time, avg_weekly_review_time, avg_monthly_review_time)) => {
            (avg_daily_review_time, avg_weekly_review_time, avg_monthly_review_time)
        }
        Err(e) => {
            println!("Error retrieving general review time metrics: {:?}", e);
            return (0.0, 0.0, 0.0)
        }
    }
}

#[tauri::command]
pub fn question_review_time_metrics(db_path: &str) -> (usize, f64, f64) {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return (0, 0.0, 0.0); 
        }
    };

    match question_review_time(&conn) {
        Ok((all_time_review_time, avg_questions_reviewed_per_day, avg_questions_reviewed_per_week)) => {
            (all_time_review_time, avg_questions_reviewed_per_day, avg_questions_reviewed_per_week)
        }
        Err(e) => {
            println!("Error retrieving question review time metrics: {:?}", e);
            return (0, 0.0, 0.0)
        }
    }
}
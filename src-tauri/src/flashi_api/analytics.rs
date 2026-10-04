use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result};

use crate::flashi_api::queries::analytics::*;

pub fn general(conn: &Connection) -> Result<(usize, usize, usize)> {
 
    conn.query_row(
        GENERAL_METRICS,
        [],
        |row| {
            let subject_count: Option<i64> = row.get(0)?; 
            let deck_count: Option<i64> = row.get(1)?; 
            let session_count: Option<i64> = row.get(2)?; 
            Ok((
                subject_count.unwrap_or(0) as usize, 
                deck_count.unwrap_or(0) as usize,
                session_count.unwrap_or(0) as usize
            ))
        }
    )
    
}

pub fn historical(conn: &Connection) -> Result<Vec<(String, usize, usize, f64)>> {

    let cutoff = (Utc::now() - Duration::days(30)).date_naive();
    let cutoff_str = cutoff.to_string();
 
    let mut stmt = conn.prepare(HISTORICAL_METRICS)?; 
    let rows = stmt.query_map(params![cutoff_str], |row| {
        let day_date: String = row.get(0)?;
        let total_correct: i64 = row.get(1)?;
        let total_incorrect: i64 = row.get(2)?;
        let avg_accuracy: f64 = row.get(3)?;
        Ok((day_date, total_correct as usize, total_incorrect as usize, avg_accuracy))
    })?;
    
    rows.collect()
    
}

pub fn review_time(conn: &Connection) -> Result<(f64, f64, f64)> {

    conn.query_row(
        REVIEW_TIME_METRICS,
        [],
        |row| {
            let avg_daily_review_time: Option<f64> = row.get(0)?; 
            let avg_weekly_review_time: Option<f64> = row.get(1)?; 
            let avg_monthly_review_time: Option<f64> = row.get(2)?; 
            Ok((
                avg_daily_review_time.unwrap_or(0.0), 
                avg_weekly_review_time.unwrap_or(0.0),
                avg_monthly_review_time.unwrap_or(0.0)
            ))
        }
    )
}

pub fn question_review_time(conn: &Connection) -> Result<(usize, f64, f64)> {

    conn.query_row(
        QUESTION_REVIEW_TIME_METRICS,
        [],
        |row| {
            let all_time_reviewed: Option<i64> = row.get(0)?; 
            let avg_questions_reviewed_per_day: Option<f64> = row.get(1)?; 
            let avg_questions_reviewed_per_week: Option<f64> = row.get(2)?; 
            Ok((
                all_time_reviewed.unwrap_or(0) as usize, 
                avg_questions_reviewed_per_day.unwrap_or(0.0),
                avg_questions_reviewed_per_week.unwrap_or(0.0)
            ))
        }
    )
}


#[tauri::command]
pub fn general_metrics(db_path: &str) -> (usize, usize, usize) {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return (0, 0, 0); 
        }
    };

    match general(&conn) {
        Ok((subject_count, deck_count, session_count)) => {
            (subject_count, deck_count, session_count)
        }
        Err(e) => {
            println!("Error retrieving general metrics: {:?}", e);
            return (0, 0, 0)
        }
    }
}


#[tauri::command]
pub fn historical_metrics(db_path: &str) -> Result<Vec<(String, usize, usize, f64)>, String> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    };

    match historical(&conn) {
        Ok(rows) => Ok(rows), 
        Err(e) => {
            println!("Error retrieving general metrics: {:?}", e);
            return Err(e.to_string())
        }
    }
}

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

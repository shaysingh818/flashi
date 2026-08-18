use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


pub fn review_time(conn: &Connection) -> Result<(f64, f64, f64)> {

    let query = "
        WITH DAILY_ENTRIES AS (
           	SELECT 
          		CAST(DATE_TRUNC('day', CREATED) AS VARCHAR) AS DAY_DATE,
          		SUM(REVIEW_TIME) AS DAILY_REVIEW_TIME
           	FROM SESSION_REVIEW_METRICS
           	GROUP BY DATE_TRUNC('day', CREATED)
           	ORDER BY DATE_TRUNC('day', CREATED)
        ),
        WEEKLY_ENTRIES AS (
           	SELECT 
          		CAST(DATE_TRUNC('week', CREATED) AS VARCHAR) AS WEEK_DATE,
          		SUM(REVIEW_TIME) AS WEEKLY_REVIEW_TIME
           	FROM SESSION_REVIEW_METRICS
           	GROUP BY DATE_TRUNC('week', CREATED)
           	ORDER BY DATE_TRUNC('week', CREATED)
        ),
        MONTHLY_ENTRIES AS (
           	SELECT 
          		CAST(DATE_TRUNC('month', CREATED) AS VARCHAR) AS MONTH_DATE,
          		SUM(REVIEW_TIME) AS MONTHLY_REVIEW_TIME
           	FROM SESSION_REVIEW_METRICS
           	GROUP BY DATE_TRUNC('month', CREATED)
           	ORDER BY DATE_TRUNC('month', CREATED)
        )
        SELECT 
           	AVG(DAILY_ENTRIES.DAILY_REVIEW_TIME) AS AVG_DAILY_REVIEW_TIME,
           	AVG(WEEKLY_ENTRIES.WEEKLY_REVIEW_TIME ) AS AVG_WEEKLY_REVIEW_TIME,
           	AVG(MONTHLY_ENTRIES.MONTHLY_REVIEW_TIME ) AS AVG_MONTHLY_REVIEW_TIME
        FROM DAILY_ENTRIES, WEEKLY_ENTRIES, MONTHLY_ENTRIES
    "; 

    conn.query_row(
        query,
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

    let query = "
        WITH ALL_QUESTIONS AS (
           	SELECT
          		SUM(REVIEWED) AS ALL_TIME_REVIEWED
           	FROM QUESTION
        ),
        DAILY_QUESTIONS AS (
           	SELECT
          		CAST(DATE_TRUNC('day', CREATED) AS VARCHAR) AS DAY_DATE,
          		SUM(REVIEWED) AS REVIEWED_PER_DAY
           	FROM QUESTION
           	GROUP BY DATE_TRUNC('day', CREATED)
        ),
        WEEKLY_QUESTIONS AS (
           	SELECT
          		CAST(DATE_TRUNC('week', CREATED) AS VARCHAR) AS WEEK_DATE,
          		SUM(REVIEWED) AS REVIEWED_PER_WEEK
           	FROM QUESTION
           	GROUP BY DATE_TRUNC('week', CREATED)
        )
        SELECT
        (SELECT ALL_TIME_REVIEWED FROM ALL_QUESTIONS) AS ALL_TIME_REVIEWED,
        (SELECT AVG(REVIEWED_PER_DAY) FROM DAILY_QUESTIONS) AS AVG_QUESTIONS_REVIEWED_PER_DAY,
        (SELECT AVG(REVIEWED_PER_WEEK) FROM WEEKLY_QUESTIONS) AS AVG_QUESTIONS_REVIEWED_PER_WEEK
    "; 

    conn.query_row(
        query,
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
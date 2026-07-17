use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result, OptionalExt}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReview {
    pub id: usize,
    pub session_id: usize,
    pub complete: bool,
    pub cycle: usize,
    pub current_question: usize,
    pub start_time: String,
    pub end_time: String,
    pub created: String, 
    pub updated: String,
}

impl SessionReview {

    pub fn new(session_id: usize) -> SessionReview {
        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();        
        SessionReview {
            id: 0,
            session_id: session_id,
            complete: false,
            cycle: 0,
            current_question: 0,
            start_time: now.clone(),
            end_time: now.clone(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<SessionReview> {
        conn.query_row(
            "INSERT INTO SESSION_REVIEW
                (SESSION_ID, COMPLETE, CYCLE, CURRENT_QUESTION, START_TIME, END_TIME, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            RETURNING ID, SESSION_ID, COMPLETE, CYCLE, CURRENT_QUESTION, START_TIME::VARCHAR, END_TIME::VARCHAR, CREATED::VARCHAR, UPDATED::VARCHAR",
            params![
                &self.session_id,
                &self.complete,
                &self.cycle,
                &self.current_question,
                &self.start_time,
                &self.end_time,
                &self.created, 
                &self.updated
            ],
            |row| {
                Ok(SessionReview {
                    id: row.get(0)?,
                    session_id: row.get(1)?, 
                    complete: row.get(2)?,
                    cycle: row.get(3)?,
                    current_question: row.get(4)?,
                    start_time: row.get(5)?, 
                    end_time: row.get(6)?,
                    created: row.get(7)?, 
                    updated: row.get(8)?,
                })
            },
        )
    }

    pub fn session_review_active(conn: &Connection, session_id: &str) -> Result<i64> {
        let query = "
        SELECT 
            ID
        FROM SESSION_REVIEW WHERE COMPLETE = FALSE AND SESSION_ID = ?";

        let mut stmt = conn.prepare(query)?; 
        let session_id: Option<i64> = stmt.query_row(
            params![session_id],
            |row| row.get(0)
        ).optional()?;

        Ok(session_id.unwrap_or(-1))
    }

    pub fn session_review_complete(conn: &Connection, session_review_id: &str) -> Result<bool> {
        
        let query = "
            WITH ACTIVE_QUESTIONS AS (
                SELECT 
                    * 
                FROM SESSION_REVIEW_QUESTION 
                WHERE SESSION_REVIEW_ID = ? AND ACTIVE = TRUE
            )
            SELECT COUNT(*) AS ACTIVE_QUESTIONS_COUNT FROM ACTIVE_QUESTIONS";

        let mut stmt = conn.prepare(query)?; 
        let active_question_count: i64 = stmt.query_row(
            params![session_review_id],
            |row| row.get(0)
        )?;

        Ok(active_question_count == 0)
    }
    
    pub fn retrieve(conn: &Connection, session_id: &str) -> Result<Vec<SessionReview>> {

        let query = "
            SELECT
                ID,
                SESSION_ID,
                COMPLETE,
                CYCLE,
                CURRENT_QUESTION,
                START_TIME::VARCHAR,
                END_TIME::VARCHAR,
                CREATED::VARCHAR,
                UPDATED::VARCHAR
            FROM SESSION_REVIEW WHERE SESSION_ID = ?";
        let mut stmt = conn.prepare(query)?;

        let sessions: Result<Vec<SessionReview>> = stmt
            .query_map([session_id], |row| {
                Ok(SessionReview {
                    id: row.get(0)?,
                    session_id: row.get(1)?, 
                    complete: row.get(2)?,
                    cycle: row.get(3)?,
                    current_question: row.get(4)?, 
                    start_time: row.get(5)?,
                    end_time: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            })?
            .collect();
        sessions
    }
    
    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        self.updated = chrono::offset::Local::now().to_string();

        let result = conn.execute(
            "UPDATE SESSION_REVIEW
                SET SESSION_ID=?1,
                    COMPLETE=?2,
                    CYCLE=?3
                    CURRENT_QUESTION=?4
                    START_TIME=?5
                    END_TIME=?6
                    UPDATED=?7
                WHERE ID=?8",
            params![
                &self.session_id,
                &self.complete,
                &self.cycle,
                &self.current_question,
                &self.start_time,
                &self.end_time,
                &self.updated, 
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated session review");
                Ok(())
            },
            Err(err) => {
                println!("[session_review::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM SESSION_REVIEW WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted session review");
                Ok(())
            },
            Err(err) => {
                println!("[session_review::delete] Error deleting session {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<SessionReview> {
        conn.query_row(
            "SELECT 
                ID, 
                SESSION_ID, 
                COMPLETE, 
                CYCLE, 
                CURRENT_QUESTION, 
                START_TIME::VARCHAR, 
                END_TIME::VARCHAR, 
                CREATED::VARCHAR, 
                UPDATED::VARCHAR FROM SESSION_REVIEW WHERE ID = ?",
            &[&id],
            |row| {
                Ok(SessionReview {
                    id: row.get(0)?,
                    session_id: row.get(1)?, 
                    complete: row.get(2)?,
                    cycle: row.get(3)?,
                    current_question: row.get(4)?, 
                    start_time: row.get(5)?,
                    end_time: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            }
        )
    }

    pub fn generate_questions(&mut self, conn: &Connection) -> Result<()> {
        let query = "
            WITH DECK_IDS AS (
	            SELECT 
                    DECK_ID 
                FROM SESSION_DECK WHERE SESSION_ID = ?
            ),
            QUESTIONS AS (
	            SELECT 
                    Q.ID, 
                    Q.DECK_ID, 
                    Q.CORRECT, 
                    Q.INCORRECT, 
                    Q.REVIEWED 
            FROM QUESTION Q
            WHERE Q.DECK_ID IN (SELECT DECK_ID FROM DECK_IDS)
            )
            SELECT 
                Q.ID
            FROM QUESTIONS AS Q
            LEFT JOIN DECK_IDS AS DIDS
            ON Q.DECK_ID = DIDS.DECK_ID            
            ORDER BY Q.CORRECT DESC";

        let mut stmt = conn.prepare(query)?; 
        
        let question_ids: Vec<usize> = stmt
            .query_map(&[&self.session_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        for (idx, question_id) in question_ids.into_iter().enumerate() {
            let mut session_question = SessionReviewQuestion::new(
                self.id, 
                question_id,
                idx
            );
            session_question.insert(conn);
        }
        
        Ok(())

    }
    
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReviewQuestion {
    pub id: usize,
    pub session_review_id: usize,
    pub question_id: usize,
    pub priority_index: usize,
    pub incorrect_count: usize,
    pub review_count: usize,
    pub active: bool,
    pub created: String, 
    pub updated: String,
}

impl SessionReviewQuestion {

    pub fn new(session_review_id: usize, question_id: usize, priority_index: usize) -> SessionReviewQuestion {
        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();        
        SessionReviewQuestion {
            id: 0,
            session_review_id: session_review_id,
            question_id: question_id,
            priority_index: priority_index,
            incorrect_count: 0,
            review_count: 0,
            active: true,
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            "INSERT INTO SESSION_REVIEW_QUESTION
                (
                    SESSION_REVIEW_ID, 
                    QUESTION_ID, 
                    PRIORITY_INDEX, 
                    INCORRECT_COUNT, 
                    REVIEW_COUNT, 
                    ACTIVE, 
                    CREATED, 
                    UPDATED
                )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &self.session_review_id,
                &self.question_id,
                &self.priority_index,
                &self.incorrect_count,
                &self.review_count,
                &self.active,
                &self.created, 
                &self.updated
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted session review question");
                Ok(())
            },
            Err(err) => {
                println!("[session_review_question::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<SessionReviewQuestion>> {
        let mut stmt = conn.prepare("SELECT * FROM SESSION_REVIEW_QUESTION")?;

        let sessions: Result<Vec<SessionReviewQuestion>> = stmt
            .query_map([], |row| {
                Ok(SessionReviewQuestion {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?, 
                    question_id: row.get(2)?,
                    priority_index: row.get(3)?,
                    incorrect_count: row.get(4)?, 
                    review_count: row.get(5)?,
                    active: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            })?
            .collect();
        sessions
    }


    pub fn retrieve_by_session_review_id(conn: &Connection, session_review_id: &str) -> Result<Vec<SessionReviewQuestion>> {

        let query = "
        SELECT 
	          ID,
            SESSION_REVIEW_ID,
            QUESTION_ID,
            PRIORITY_INDEX,
            INCORRECT_COUNT,
            REVIEW_COUNT,
            ACTIVE,
            UPDATED::VARCHAR,
            CREATED::VARCHAR
        FROM SESSION_REVIEW_QUESTION WHERE SESSION_REVIEW_ID = ? AND ACTIVE = TRUE ORDER BY PRIORITY_INDEX ASC";

        let mut stmt = conn.prepare(query)?;

        let sessions: Result<Vec<SessionReviewQuestion>> = stmt
            .query_map([&session_review_id], |row| {
                Ok(SessionReviewQuestion {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?, 
                    question_id: row.get(2)?,
                    priority_index: row.get(3)?,
                    incorrect_count: row.get(4)?, 
                    review_count: row.get(5)?,
                    active: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            })?
            .collect();
        sessions
    }

    
    pub fn retrieve_active_questions(conn: &Connection, session_id: &str) -> Result<Vec<SessionReviewQuestion>> {
        let query = "
            WITH DECK_IDS AS (
	            SELECT 
                    DECK_ID 
                FROM SESSION_DECK WHERE SESSION_ID = ?
            ),
            QUESTIONS AS (
	            SELECT 
                    Q.ID, 
                    Q.DECK_ID, 
                    Q.CORRECT, 
                    Q.INCORRECT, 
                    Q.REVIEWED 
            FROM QUESTION Q
            WHERE Q.DECK_ID IN (SELECT DECK_ID FROM DECK_IDS)
            )
            SELECT 
              SRQ.ID,
              SRQ.SESSION_REVIEW_ID,
              SRQ.QUESTION_ID,
              SRQ.PRIORITY_INDEX,
              SRQ.INCORRECT_COUNT,
              SRQ.REVIEW_COUNT,
              SRQ.ACTIVE,
              SRQ.UPDATED::VARCHAR,
              SRQ.CREATED::VARCHAR
            FROM QUESTIONS AS Q
            LEFT JOIN DECK_IDS AS DIDS
            ON Q.DECK_ID = DIDS.DECK_ID            
            LEFT JOIN SESSION_REVIEW_QUESTION AS SRQ
            ON Q.ID = SRQ.QUESTION_ID
            WHERE SRQ.ACTIVE = TRUE
            ORDER BY Q.CORRECT DESC";


        let mut stmt = conn.prepare(query)?;        
        let sessions: Result<Vec<SessionReviewQuestion>> = stmt
            .query_map([session_id], |row| {
                Ok(SessionReviewQuestion {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?, 
                    question_id: row.get(2)?,
                    priority_index: row.get(3)?,
                    incorrect_count: row.get(4)?, 
                    review_count: row.get(5)?,
                    active: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            })?
            .collect();
         
        sessions
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();        

        let result = conn.execute(
            "UPDATE SESSION_REVIEW_QUESTION
                SET SESSION_REVIEW_ID=?1,
                    QUESTION_ID=?2,
                    PRIORITY_INDEX=?3,
                    INCORRECT_COUNT=?4,
                    REVIEW_COUNT=?5,
                    ACTIVE=?6,
                    UPDATED=?7,
                WHERE ID=?8",
            params![
                &self.session_review_id,
                &self.question_id,
                &self.priority_index,
                &self.incorrect_count,
                &self.review_count,
                &self.active,
                &self.updated, 
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated session review question");
                Ok(())
            },
            Err(err) => {
                println!("[session_review_question::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM SESSION_REVIEW_QUESTION WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted session review question");
                Ok(())
            },
            Err(err) => {
                println!("[session_review_question::delete] Error deleting session review question {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<SessionReviewQuestion> {

        let query = "
        SELECT
	        ID,
            SESSION_REVIEW_ID,
            QUESTION_ID,
            PRIORITY_INDEX,
            INCORRECT_COUNT,
            REVIEW_COUNT,
            ACTIVE,
            UPDATED::VARCHAR,
            CREATED::VARCHAR
        FROM SESSION_REVIEW_QUESTION WHERE ID = ?"; 

        conn.query_row(
            query,
            &[&id],
            |row| {
                Ok(SessionReviewQuestion {
                    id: row.get(0)?,
                    session_review_id: row.get(1)?, 
                    question_id: row.get(2)?,
                    priority_index: row.get(3)?,
                    incorrect_count: row.get(4)?, 
                    review_count: row.get(5)?,
                    active: row.get(6)?,
                    created: row.get(7)?,
                    updated: row.get(8)?,
                })
            }
        )
    }

}


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

    pub fn new(
        session_review_id: usize, 
        questions_correct: usize, 
        questions_incorrect: usize, 
        accuracy: f64,
        total_review_count: usize,
        number_of_cycles: usize,
        review_time: f64) -> SessionReviewMetrics {

        let now = chrono::offset::Local::now().to_string();

        SessionReviewMetrics {
            id: 0,
            session_review_id: session_review_id,
            questions_correct: questions_correct,
            questions_incorrect: questions_incorrect,
            accuracy: accuracy,
            total_review_count: total_review_count,
            number_of_cycles: number_of_cycles,
            review_time: review_time,
            created: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            "INSERT INTO SESSION_REVIEW_METRICS
                (
                    SESSION_REVIEW_ID, 
                    QUESTIONS_CORRECT, 
                    QUESTIONS_INCORRECT, 
                    ACCURACY, 
                    TOTAL_REVIEW_COUNT, 
                    NUMBER_OF_CYCLES, 
                    REVIEW_TIME,
                    CREATED, 
                )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
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
                println!("Successfully inserted session review metrics");
                Ok(())
            },
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

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM SESSION_REVIEW_QUESTION WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted session review question");
                Ok(())
            },
            Err(err) => {
                println!("[session_review_metrics::delete] Error deleting session review metrics {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<SessionReviewMetrics> {
        conn.query_row(
            "SELECT * FROM SESSION_REVIEW_METRICS WHERE ID = ?",
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

            }
        )
    }

}

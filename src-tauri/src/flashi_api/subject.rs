use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


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
            "INSERT INTO SUBJECT 
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
                println!("Successfully inserted subject");
                Ok(())
            },
            Err(err) => {
                println!("[subject::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<Subject>> {
        
        let mut stmt = conn.prepare("SELECT ID, NAME, DESCRIPTION, CREATED::VARCHAR, UPDATED::VARCHAR FROM SUBJECT")?;

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
            "UPDATE SUBJECT
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
                println!("Successfully updated subject");
                Ok(())
            },
            Err(err) => {
                println!("[subject::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM SUBJECT WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted subject");
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
            "SELECT ID, NAME, DESCRIPTION, CREATED::VARCHAR, UPDATED::VARCHAR FROM SUBJECT WHERE ID = ?",
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
        let query = format!(
            "SELECT ID, NAME, DESCRIPTION, CREATED::VARCHAR, UPDATED::VARCHAR FROM SUBJECT WHERE NAME LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;

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

    pub fn metrics(conn: &Connection, id: &str) -> Result<(usize, usize, f64)> {

        let query = "
            WITH ALL_QUESTIONS AS (
                SELECT COUNT(*) AS TOTAL_QUESTION_COUNT
                FROM QUESTION
                WHERE DECK_ID IN (SELECT DISTINCT ID FROM DECK WHERE SUBJECT_ID = ?)
            ),
            QUESTIONS_REVIEWED AS (
                SELECT SUM(REVIEWED) AS TOTAL_REVIEWED
                FROM QUESTION
                WHERE DECK_ID IN (SELECT DISTINCT ID FROM DECK WHERE SUBJECT_ID = ?)
            ),
            QUESTIONS_ACCURACY AS (
              	 SELECT 
             	 	SUM(REVIEWED) AS TOTAL_REVIEWED,
             	 	SUM(CORRECT) AS TOTAL_CORRECT
              	 FROM QUESTION WHERE DECK_ID IN (SELECT DISTINCT ID FROM DECK WHERE SUBJECT_ID = ?)
              	 GROUP BY CORRECT, REVIEWED
            )
            SELECT
                ALL_QUESTIONS.TOTAL_QUESTION_COUNT,
                QUESTIONS_REVIEWED.TOTAL_REVIEWED,
                (QUESTIONS_ACCURACY.TOTAL_CORRECT / QUESTIONS_ACCURACY.TOTAL_REVIEWED) AS TOTAL_ACCURACY
            FROM ALL_QUESTIONS, QUESTIONS_REVIEWED, QUESTIONS_ACCURACY;
        ";

        conn.query_row(
            query,
            &[&id],
            |row| {
                let question_count: Option<i64> = row.get(0)?;
                let total_reviewed: Option<i64> = row.get(1)?;
                let total_accuracy: Option<i64> = row.get(2)?; 
                Ok((
                    question_count.unwrap_or(0) as usize,
                    total_reviewed.unwrap_or(0) as usize,
                    total_accuracy.unwrap_or(0) as f64
                ))
            }
        )
    }
}

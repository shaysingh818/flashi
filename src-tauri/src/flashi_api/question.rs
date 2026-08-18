use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub deck_id: usize,
    pub answer_type_id: usize,
    pub answer_id: usize,
    pub reviewed: usize,
    pub correct: usize, 
    pub incorrect: usize,
    pub raw: String,
    pub created: String,
    pub updated: String
}

impl Question {

    pub fn new(deck_id: usize, answer_type_id: usize, answer_id: usize, raw: &str) -> Question {

        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      
        Question {
            id: 0,
            deck_id: deck_id,
            answer_type_id: answer_type_id,
            answer_id: answer_id,
            reviewed: 0,
            correct: 0, 
            incorrect: 0,
            raw: raw.to_string(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
 
        let result = conn.execute(
            "INSERT INTO QUESTION 
                (DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &self.deck_id,
                &self.answer_type_id,
                &self.answer_id,
                &self.reviewed,
                &self.correct,
                &self.incorrect,                
                &self.raw,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted question");
                Ok(())
            },
            Err(err) => {
                println!("[question::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection, deck_id: &str) -> Result<Vec<Question>> {
 
        let mut stmt = conn.prepare("SELECT ID, DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED::VARCHAR AS CREATED, UPDATED::VARCHAR AS UPDATED FROM QUESTION WHERE DECK_ID = ?")?;

        let questions: Result<Vec<Question>> = stmt
            .query_map([&deck_id], |row| {
                Ok(Question {
                    id: row.get("ID")?,
                    deck_id: row.get("DECK_ID")?,
                    answer_type_id: row.get("ANSWER_TYPE_ID")?,
                    answer_id: row.get("ANSWER_ID")?,
                    reviewed: row.get("REVIEWED")?,
                    correct: row.get("CORRECT")?, 
                    incorrect: row.get("INCORRECT")?,
                    raw: row.get("RAW")?,
                    created: row.get("CREATED")?,
                    updated: row.get("UPDATED")?,
                })
            })?
            .collect();
        questions
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {

      self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            "UPDATE QUESTION
                SET 
                    RAW=?1,
                    REVIEWED=?2, 
                    CORRECT=?3, 
                    INCORRECT=?4,
                    UPDATED=?5
                WHERE ID=?6",
            params![
                &self.raw,
                &self.reviewed,
                &self.correct,
                &self.incorrect,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated question");
                Ok(())
            },
            Err(err) => {
                println!("[question::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM QUESTION WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted question");
                Ok(())
            },
            Err(err) => {
                println!("[question::delete] Error deleting question {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<Question> {
        conn.query_row(
            "SELECT ID, DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED::VARCHAR, UPDATED::VARCHAR FROM QUESTION WHERE ID = ?",
            &[&id],
            |row| {
                Ok(Question {
                    id: row.get(0)?,
                    deck_id: row.get(1)?,
                    answer_type_id: row.get(2)?,
                    answer_id: row.get(3)?,
                    reviewed: row.get(4)?,
                    correct: row.get(5)?,
                    incorrect: row.get(6)?,
                    raw: row.get(7)?,
                    created: row.get(8)?,
                    updated: row.get(9)?,
                })
            }
        )
    }

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Question>> {
        let query = format!(
            "SELECT ID, DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED::VARCHAR, UPDATED::VARCHAR FROM QUESTION WHERE RAW LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;

        let questions: Result<Vec<Question>> = stmt
            .query_map([], |row| {
                Ok(Question {
                    id: row.get(0)?,
                    deck_id: row.get(1)?,
                    answer_type_id: row.get(2)?,
                    answer_id: row.get(3)?,
                    reviewed: row.get(4)?,
                    correct: row.get(5)?,
                    incorrect: row.get(6)?,
                    raw: row.get(7)?,
                    created: row.get(8)?,
                    updated: row.get(9)?,
                })
            })?
            .collect();
        questions
    }

}

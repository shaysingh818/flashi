use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoiceAnswer {
    pub answer_id: usize,
    pub is_correct: bool,
    pub answer: String,
    pub explanation: String,
    pub created: String,
    pub updated: String
}

impl MultipleChoiceAnswer {
    
    pub fn new(answer: &str, explanation: &str, is_correct: bool) -> MultipleChoiceAnswer {
        MultipleChoiceAnswer {
            answer_id: 0,
            is_correct: is_correct,
            answer: answer.to_string(),
            explanation: explanation.to_string(),
            created: chrono::offset::Local::now().to_string(),
            updated: chrono::offset::Local::now().to_string(),
        }       
    }


    pub fn insert(&mut self, conn: &Connection) -> Result<()> {

        let result = conn.execute(
            "INSERT INTO ANSWER_TYPE 
                (IS_CORRECT, ANSWER, EXPLANATION, CREATED, UPDATED) 
                VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &self.is_correct,
                &self.answer,
                &self.explanation,
                &self.created,
                &self.updated 
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted answer type");
                return Ok(())
            },
            Err(err) => {
                println!("[answer_type::insert] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<MultipleChoiceAnswer>> {
        let mut stmt = conn.prepare("SELECT * FROM OPEN_MULTIPLE_CHOICE_ANSWER")?;

        let multiple_choice_answers: Result<Vec<MultipleChoiceAnswer>> = stmt
            .query_map([], |row| {
                Ok(MultipleChoiceAnswer {
                    answer_id: row.get(0)?,
                    is_correct: row.get(1)?,
                    answer: row.get(2)?,
                    explanation: row.get(3)?,
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            })?
            .collect();

        multiple_choice_answers
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        
        self.updated = chrono::offset::Local::now().to_string();

        let result = conn.execute(
            "UPDATE MULTIPLE_CHOICE_ANSWER
                SET IS_CORRECT=?,
                ANSWER=?,
                EXPLANATION=?,
                UPDATED=?
                WHERE ID=?",
            params![
                &self.is_correct,
                &self.answer,
                &self.explanation,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated multiple choice answer");
                return Ok(())
            },
            Err(err) => {
                println!("[multiple_choice_answer::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM MULTIPLE_CHOICE_ANSWER WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted multiple choice answer"); 
                return Ok(());
            },
            Err(err) => {
                println!("[multiple_choice_answer::delete] Error deleting multiple choice answer {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<MultipleChoiceAnswer> {
        let query = "SELECT * FROM MULTIPLE_CHOICE_ANSWER WHERE ID = ?";
        conn.query_row(query, &[&id], |row| {
            Ok(MultipleChoiceAnswer {
                answer_id: row.get(0)?,
                is_correct: row.get(1)?,
                answer: row.get(2)?,
                explanation: row.get(3)?,
                created: row.get(4)?,
                updated: row.get(5)?,
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<MultipleChoiceAnswer>> {

        let query = format!(
            "SELECT * FROM MULTIPLE_CHOICE_ANSWER WHERE ANSWER LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;
        let multiple_choice_answers: Result<Vec<MultipleChoiceAnswer>> = stmt
            .query_map([], |row| {
                Ok(MultipleChoiceAnswer {
                    answer_id: row.get(0)?,
                    is_correct: row.get(1)?,
                    answer: row.get(2)?,
                    explanation: row.get(3)?,
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            })?
            .collect();
        multiple_choice_answers
    }


}

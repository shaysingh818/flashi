use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoiceAnswer {
    pub id: usize,
    pub answer_id: usize,
    pub is_correct: bool,
    pub answer: String,
    pub explanation: String,
    pub created: String,
    pub updated: String
}

impl MultipleChoiceAnswer {
    
    pub fn new(answer_id: usize, answer: &str, explanation: &str, is_correct: bool) -> MultipleChoiceAnswer {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      MultipleChoiceAnswer {
            id: 0,
            answer_id: answer_id,
            is_correct: is_correct,
            answer: answer.to_string(),
            explanation: explanation.to_string(),
            created: now.clone(),
            updated: now,
        }       
    }
    
    pub fn insert(&mut self, conn: &Connection) -> Result<MultipleChoiceAnswer> {
        conn.query_row(
            " INSERT INTO MULTIPLE_CHOICE_ANSWER 
              (ANSWER_ID, IS_CORRECT, ANSWER, EXPLANATION, CREATED, UPDATED) 
              VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            RETURNING ID, ANSWER_ID, IS_CORRECT, ANSWER, EXPLANATION, CREATED::VARCHAR, UPDATED::VARCHAR",
            params![
                &self.answer_id,
                &self.is_correct,
                &self.answer,
                &self.explanation,
                &self.created,
                &self.updated
            ],
            |row| {
                Ok(MultipleChoiceAnswer {
                    id: row.get(0)?, 
                    answer_id: row.get(1)?,
                    is_correct: row.get(2)?,
                    answer: row.get(3)?,
                    explanation: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            },
        )
    }

    pub fn retrieve(answer_id: usize, conn: &Connection) -> Result<Vec<MultipleChoiceAnswer>> {
        let mut stmt = conn.prepare("SELECT ID, ANSWER_ID, IS_CORRECT, ANSWER, EXPLANATION, CREATED::VARCHAR, UPDATED::VARCHAR FROM MULTIPLE_CHOICE_ANSWER WHERE ANSWER_ID = ?")?;

        let multiple_choice_answers: Result<Vec<MultipleChoiceAnswer>> = stmt
            .query_map([&answer_id], |row| {
                Ok(MultipleChoiceAnswer {
                    id: row.get(0)?, 
                    answer_id: row.get(1)?,
                    is_correct: row.get(2)?,
                    answer: row.get(3)?,
                    explanation: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            })?
            .collect();

        multiple_choice_answers
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

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
        let query = "SELECT ID, ANSWER_ID, IS_CORRECT, ANSWER, EXPLANATION, CREATED::VARCHAR, UPDATED::VARCHAR FROM MULTIPLE_CHOICE_ANSWER WHERE ID = ?";
        conn.query_row(query, &[&id], |row| {
            Ok(MultipleChoiceAnswer {
                id: row.get(0)?,
                answer_id: row.get(1)?,
                is_correct: row.get(2)?,
                answer: row.get(3)?,
                explanation: row.get(4)?,
                created: row.get(5)?,
                updated: row.get(6)?,
            })
        })
    }
    

}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleChoice {
    pub answer_id: usize,
    pub created: String,
    pub updated: String
}

impl MultipleChoice {
    
    pub fn new() -> MultipleChoice {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      MultipleChoice {
            answer_id: 0,
            created: now.clone(),
            updated: now,
        }       
    }
    
    pub fn insert(&mut self, conn: &Connection) -> Result<MultipleChoice> {
        conn.query_row(
            "INSERT INTO MULTIPLE_CHOICE 
              (CREATED, UPDATED) 
              VALUES (?1, ?2)
            RETURNING ANSWER_ID, CREATED::VARCHAR, UPDATED::VARCHAR",
            params![
                &self.created,
                &self.updated
            ],
            |row| {
                Ok(MultipleChoice {
                    answer_id: row.get(0)?,
                    created: row.get(1)?,
                    updated: row.get(2)?,
                })
            },
        )
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM MULTIPLE_CHOICE WHERE ANSWER_ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted multiple choice"); 
                return Ok(());
            },
            Err(err) => {
                println!("[multiple_choice::delete] Error deleting multiple choice {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<MultipleChoice> {
        let query = "SELECT ANSWER_ID, CREATED::VARCHAR, UPDATED::VARCHAR FROM MULTIPLE_CHOICE_ANSWER WHERE ANSWER_ID = ?";
        conn.query_row(query, &[&id], |row| {
            Ok(MultipleChoice {
                answer_id: row.get(0)?,
                created: row.get(1)?,
                updated: row.get(2)?,
            })
        })
    }

    pub fn validate(&mut self, conn: &Connection, selected_answers: Vec<String>) -> Result<(bool, String)> {

        let query = "SELECT ID::VARCHAR FROM MULTIPLE_CHOICE_ANSWER WHERE ANSWER_ID = ? AND IS_CORRECT = TRUE";
        
        let mut stmt  = conn.prepare(query)?;
        let correct_answers: Vec<String> = stmt
            .query_map(&[&self.answer_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        println!("Answer id: {:?}", self.answer_id); 
        println!("Correct answers: {:?}", correct_answers);
        println!("Selected answers: {:?}", selected_answers); 

        if selected_answers == correct_answers {
            Ok((true, String::from("Correct!")))
        } else {
            Ok((false, String::from("Missing one or more selected answers"))) 
        }

    }
  
}

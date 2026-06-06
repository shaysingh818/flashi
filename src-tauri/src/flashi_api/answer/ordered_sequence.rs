use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderedSequenceAnswer {
    pub answer_id: usize,
    pub sequence: Vec<String>,
    pub explanation: String,
    pub created: String,
    pub updated: String
}

impl OrderedSequenceAnswer {
    
    pub fn new(sequence: Vec<&str>, explanation: &str) -> OrderedSequenceAnswer {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      OrderedSequenceAnswer {
            answer_id: 0,
            sequence: sequence.iter().map(|k| k.to_string()).collect(),
            explanation: explanation.to_string(),
            created: now.clone(),
            updated: now,
        }       
    }

    pub fn parse_sequence(raw: String) -> Vec<String> {
        raw.trim_matches(|c| c == '[' || c == ']')
            .split(", ")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_matches('\'').to_string())
            .collect()
    }   

    pub fn insert(&mut self, conn: &Connection) -> Result<OrderedSequenceAnswer> {

        let sequence_literal = format!(
            "[{}]",
            self.sequence
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''")))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let sql = format!(
            "INSERT INTO ORDERED_SEQUENCE_ANSWER 
                (SEQUENCE, EXPLANATION, CREATED, UPDATED)
            VALUES ({}, ?1, ?2, ?3)
            RETURNING ANSWER_ID, SEQUENCE::VARCHAR, EXPLANATION, CREATED::VARCHAR, UPDATED::VARCHAR",
            sequence_literal
        );
        
        conn.query_row(
            &sql,
            params![
                &self.explanation,
                &self.created,
                &self.updated
            ],
            |row| {              
                let sequence_raw: String = row.get(1)?;
                Ok(OrderedSequenceAnswer {
                    answer_id: row.get(0)?,
                    sequence: Self::parse_sequence(sequence_raw),
                    explanation: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            },
        )

    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<OrderedSequenceAnswer>> {

        let mut stmt = conn.prepare("SELECT ANSWER_ID, SEQUENCE::VARCHAR, EXPLANATION, CREATED::VARCHAR, UPDATED::VARCHAR FROM ORDERED_SEQUENCE_ANSWER")?;

        let ordered_sequence_answers: Result<Vec<OrderedSequenceAnswer>> = stmt
            .query_map([], |row| {
                let sequence_raw: String = row.get(1)?;
                Ok(OrderedSequenceAnswer {
                    answer_id: row.get(0)?,
                    sequence: Self::parse_sequence(sequence_raw),
                    explanation: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();

        ordered_sequence_answers
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let sequence_literal = format!(
            "[{}]",
            self.sequence
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''"))) // escape single quotes
                .collect::<Vec<_>>()
                .join(", ")
        );

        let sql = format!(
            "UPDATE ORDERED_SEQUENCE_ANSWER
                SET SEQUENCE={},
                    EXPLANATION=?1,
                    UPDATED=?2
                WHERE ANSWER_ID=?3",
            sequence_literal
        );

        let result = conn.execute(
            &sql,
            params![
                &self.explanation,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated open ended answer");
                return Ok(())
            },
            Err(err) => {
                println!("[ordered_sequence_answer::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }


    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM ORDERED_SEQUENCE_ANSWER WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted ordered sequence answer"); 
                return Ok(());
            },
            Err(err) => {
                println!("[ordered_sequence_answer::delete] Error deleting open ended answer {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<OrderedSequenceAnswer> {
        let query = "SELECT ANSWER_ID, SEQUENCE::VARCHAR, EXPLANATION, CREATED::VARCHAR, UPDATED::VARCHAR FROM ORDERED_SEQUENCE_ANSWER WHERE ANSWER_ID = ?";
        conn.query_row(query, &[&id], |row| {
            let sequence_raw: String = row.get(1)?;
            Ok(OrderedSequenceAnswer {
                answer_id: row.get(0)?,
                sequence: Self::parse_sequence(sequence_raw),
                explanation: row.get(2)?,
                created: row.get(3)?,
                updated: row.get(4)?,
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<OrderedSequenceAnswer>> {
        let query = format!(
            "SELECT ANSWER_ID, SEQUENCE::VARCHAR, EXPLANATION, CREATED::VARCHAR, UPDATED::VARCHAR FROM ORDERED_SEQUENCE_ANSWER WHERE EXPLANATION LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;
        let ordered_sequence_answers: Result<Vec<OrderedSequenceAnswer>> = stmt
            .query_map([], |row| {
                let sequence_raw: String = row.get(1)?;
                Ok(OrderedSequenceAnswer {
                    answer_id: row.get(0)?,
                    sequence: Self::parse_sequence(sequence_raw),
                    explanation: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();
        ordered_sequence_answers
    }

    pub fn validate(&mut self, answer_sequence: Vec<String>) -> (bool, String) {

        let mut result = true;
        let mut explanation = String::new();
        
        if answer_sequence == self.sequence {
            (true, String::from("Correct!"))
        } else {
            (false, format!("{:?}", self.sequence))
        }

    }

}

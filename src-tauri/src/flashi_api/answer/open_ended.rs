use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenEndedAnswer {
    pub answer_id: usize,
    pub answer: String,
    pub explanation: String,
    pub keywords: Vec<String>,
    pub created: String,
    pub updated: String
}

impl OpenEndedAnswer {
    
    pub fn new(answer: &str, explanation: &str, keywords: Vec<&str>) -> OpenEndedAnswer {

      let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

      OpenEndedAnswer {
            answer_id: 0,
            answer: answer.to_string(),
            explanation: explanation.to_string(),
            keywords: keywords.iter().map(|k| k.to_string()).collect(),
            created: now.clone(),
            updated: now,
        }       
    }

    pub fn parse_keywords(raw: String) -> Vec<String> {
        raw.trim_matches(|c| c == '[' || c == ']')
            .split(", ")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_matches('\'').to_string())
            .collect()
    }  
   
    pub fn insert(&mut self, conn: &Connection) -> Result<OpenEndedAnswer> {

        let keywords_literal = format!(
            "[{}]",
            self.keywords
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''")))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let sql = format!(
            "INSERT INTO OPEN_ENDED_ANSWER 
                (ANSWER, EXPLANATION, KEYWORDS, CREATED, UPDATED)
            VALUES (?1, ?2, {}, ?3, ?4)
            RETURNING ANSWER_ID, ANSWER, EXPLANATION, KEYWORDS::VARCHAR, CREATED::VARCHAR, UPDATED::VARCHAR",
            keywords_literal
        );

        conn.query_row(
            &sql,
            params![
                &self.answer,
                &self.explanation,
                &self.created,
                &self.updated
            ],
            |row| {              
              let keywords_raw: String = row.get(3)?;
                Ok(OpenEndedAnswer {
                    answer_id: row.get(0)?,
                    answer: row.get(1)?,
                    explanation: row.get(2)?,
                    keywords: Self::parse_keywords(keywords_raw),
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            },
        )
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<OpenEndedAnswer>> {
        let mut stmt = conn.prepare("SELECT ANSWER_ID, ANSWER, EXPLANATION, KEYWORDS::VARCHAR, CREATED::VARCHAR, UPDATED::VARCHAR FROM OPEN_ENDED_ANSWER")?;

        let open_ended_answers: Result<Vec<OpenEndedAnswer>> = stmt
            .query_map([], |row| {
                let keywords_raw: String = row.get(3)?;
                Ok(OpenEndedAnswer {
                    answer_id: row.get(0)?,
                    answer: row.get(1)?,
                    explanation: row.get(2)?,
                    keywords: Self::parse_keywords(keywords_raw),
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            })?
            .collect();

        open_ended_answers
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        /* change date modified */
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let keywords_literal = format!(
            "[{}]",
            self.keywords
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''"))) // escape single quotes
                .collect::<Vec<_>>()
                .join(", ")
        );

        let sql = format!(
            "UPDATE OPEN_ENDED_ANSWER
                SET ANSWER=?1,
                    EXPLANATION=?2,
                    KEYWORDS={},
                    UPDATED=?3
                WHERE ANSWER_ID=?4",
            keywords_literal
        );

        let result = conn.execute(
            &sql,
            params![
                &self.answer,
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
                println!("[open_ended_answer::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }


    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM OPEN_ENDED_ANSWER WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted open ended answer"); 
                return Ok(());
            },
            Err(err) => {
                println!("[open_ended_answer::delete] Error deleting open ended answer {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<OpenEndedAnswer> {
        let query = "SELECT ANSWER_ID, ANSWER, EXPLANATION, KEYWORDS::VARCHAR, CREATED::VARCHAR, UPDATED::VARCHAR FROM OPEN_ENDED_ANSWER WHERE ANSWER_ID = ?";
        conn.query_row(query, &[&id], |row| {
            let keywords_raw: String = row.get(3)?;
            Ok(OpenEndedAnswer {
                answer_id: row.get(0)?,
                answer: row.get(1)?,
                explanation: row.get(2)?,
                keywords: Self::parse_keywords(keywords_raw),
                created: row.get(4)?,
                updated: row.get(5)?
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<OpenEndedAnswer>> {
        let query = format!(
            "SELECT ANSWER_ID, ANSWER, EXPLANATION, KEYWORDS::VARCHAR, CREATED::VARCHAR, UPDATED::VARCHAR FROM ANSWER_TYPE WHERE ANSWER LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;
        let open_ended_answers: Result<Vec<OpenEndedAnswer>> = stmt
            .query_map([], |row| {
                let keywords_raw: String = row.get(3)?;
                Ok(OpenEndedAnswer {
                    answer_id: row.get(0)?,
                    answer: row.get(1)?,
                    explanation: row.get(2)?,
                    keywords: Self::parse_keywords(keywords_raw),
                    created: row.get(4)?,
                    updated: row.get(5)?
                })
            })?
            .collect();
        open_ended_answers
    }

}

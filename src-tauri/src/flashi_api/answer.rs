use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

/*
CREATE TABLE IF NOT EXISTS ANSWER_TYPE (
    ID          INTEGER     PRIMARY KEY,
    NAME        VARCHAR     NOT NULL,
    CREATED     TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UPDATED     TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);



CREATE TABLE IF NOT EXISTS ORDERED_SEQUENCE_ANSWER (
    ANSWER_ID     INTEGER     PRIMARY KEY,
    SEQUENCE      VARCHAR[],
    EXPLANATION   TEXT,
    CREATED       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UPDATED       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE IF NOT EXISTS MATCHING_ANSWER (
    ANSWER_ID     INTEGER     PRIMARY KEY,
    MATCHES       JSON,
    EXPLANATION   TEXT,
    CREATED       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UPDATED       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerType {
    pub answer_type_id: usize,
    pub name: String,
    pub created: String,
    pub updated: String
}

impl AnswerType {
    
    pub fn new(name: &str) -> AnswerType {
        AnswerType {
            answer_type_id: 0,
            name: name.to_string(),
            created: chrono::offset::Local::now().to_string(),
            updated: chrono::offset::Local::now().to_string(),
        }       
    }


    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            "INSERT INTO ANSWER_TYPE 
                (NAME, CREATED, UPDATED) 
                VALUES (?1, ?2, ?3)",
            params![
                &self.name,
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

    pub fn retrieve(conn: &Connection) -> Result<Vec<AnswerType>> {
        let mut stmt = conn.prepare("SELECT * FROM ANSWER_TYPE")?;

        let answer_types: Result<Vec<AnswerType>> = stmt
            .query_map([], |row| {
                Ok(AnswerType {
                    answer_type_id: row.get(0)?,
                    name: row.get(1)?,
                    created: row.get(2)?,
                    updated: row.get(3)?,
                })
            })?
            .collect();

        answer_types
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        /* change date modified */
        self.updated = chrono::offset::Local::now().to_string();

        let result = conn.execute(
            "UPDATE ANSWER_TYPE
                SET NAME=?, 
                UPDATED=?
                WHERE ID=?",
            [
                &self.name,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated answer type");
                return Ok(())
            },
            Err(err) => {
                println!("[answer_type::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }


    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM ANSWER_TYPE WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted answer type"); 
                return Ok(());
            },
            Err(err) => {
                println!("[answer_type::delete] Error deleting answer type {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<AnswerType> {
        let query = "SELECT * FROM ANSWER_TYPE WHERE ID = ?";
        conn.query_row(query, &[&id], |row| {
            Ok(AnswerType {
                answer_type_id: row.get(0)?,
                name: row.get(1)?,
                created: row.get(2)?,
                updated: row.get(3)?,
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<AnswerType>> {
        let query = format!(
            "SELECT * FROM ANSWER_TYPE WHERE NAME LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;
        let answer_types: Result<Vec<AnswerType>> = stmt
            .query_map([], |row| {
                Ok(AnswerType {
                    answer_type_id: row.get(0)?,
                    name: row.get(1)?,
                    created: row.get(2)?,
                    updated: row.get(3)?,
                })
            })?
            .collect();
        answer_types
    }

}


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
        OpenEndedAnswer {
            answer_id: 0,
            answer: answer.to_string(),
            explanation: explanation.to_string(),
            keywords: keywords.iter().map(|k| k.to_string()).collect(),
            created: chrono::offset::Local::now().to_string(),
            updated: chrono::offset::Local::now().to_string(),
        }       
    }

    pub fn parse_keywords(raw: String) -> Vec<String> {
        raw.trim_matches(|c| c == '[' || c == ']')
            .split(", ")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_matches('\'').to_string())
            .collect()
    }   

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {

        let keywords_literal = format!(
            "[{}]",
            self.keywords
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''"))) // escape single quotes
                .collect::<Vec<_>>()
                .join(", ")
        );

        let sql = format!(
            "INSERT INTO OPEN_ENDED_ANSWER 
                (ANSWER, EXPLANATION, KEYWORDS, CREATED, UPDATED)
            VALUES (?1, ?2, {}, ?3, ?4)",
            keywords_literal
        );

        let result = conn.execute(
            &sql, 
            params![
                &self.answer,
                &self.explanation,
                &self.created,
                &self.updated 
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted open ended answer");
                return Ok(())
            },
            Err(err) => {
                println!("[open_ended_answer::insert] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<OpenEndedAnswer>> {
        let mut stmt = conn.prepare("SELECT * FROM OPEN_ENDED_ANSWER")?;

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
        self.updated = chrono::offset::Local::now().to_string();

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
        let query = "SELECT * FROM OPEN_ENDED_ANSWER WHERE ID = ?";
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
            "SELECT * FROM ANSWER_TYPE WHERE ANSWER LIKE '%{}%'",
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

/*

CREATE TABLE IF NOT EXISTS MULTIPLE_CHOICE_ANSWER (
    ANSWER_ID     INTEGER     PRIMARY KEY,
    IS_CORRECT    BOOLEAN     NOT NULL,
    ANSWER        TEXT,
    EXPLANATION   TEXT,
    CREATED       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UPDATED       TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

*/

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
        OpenEndedAnswer {
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

    pub fn retrieve(conn: &Connection) -> Result<Vec<OpenEndedAnswer>> {
        let mut stmt = conn.prepare("SELECT * FROM OPEN_ENDED_ANSWER")?;

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
        self.updated = chrono::offset::Local::now().to_string();

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
        let query = "SELECT * FROM OPEN_ENDED_ANSWER WHERE ID = ?";
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
            "SELECT * FROM ANSWER_TYPE WHERE ANSWER LIKE '%{}%'",
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



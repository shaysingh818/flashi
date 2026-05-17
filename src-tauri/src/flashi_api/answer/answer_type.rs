use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerType {
    pub answer_type_id: usize,
    pub name: String,
    pub created: String,
    pub updated: String
}

impl AnswerType {
    
    pub fn new(name: &str) -> AnswerType {

        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      
        AnswerType {
            answer_type_id: 0,
            name: name.to_string(),
            created: now.clone(),
            updated: now,
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
        let mut stmt = conn.prepare("SELECT ID, NAME, CREATED::VARCHAR, UPDATED::VARCHAR FROM ANSWER_TYPE")?;

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
        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
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
        let query = "SELECT ID, NAME, CREATED::VARCHAR, UPDATED::VARCHAR FROM ANSWER_TYPE WHERE ID = ?";
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
            "SELECT ID, NAME, CREATED::VARCHAR, UPDATED::VARCHAR FROM ANSWER_TYPE WHERE NAME LIKE '%{}%'",
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

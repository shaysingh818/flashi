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
        Subject {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            created: chrono::offset::Local::now().to_string(),
            updated: chrono::offset::Local::now().to_string(),
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
        let mut stmt = conn.prepare("SELECT * FROM SUBJECT")?;

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
        self.updated = chrono::offset::Local::now().to_string();

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
            "SELECT * FROM SUBJECT WHERE ID = ?",
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
            "SELECT * FROM SUBJECT WHERE NAME LIKE '%{}%'",
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
}

use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: usize,
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub created: String,
    pub updated: String,
}

impl Session {

    pub fn new(name: &str) -> Session {
        let now = chrono::offset::Local::now().to_string();
        Session {
            id: 0,
            name: name.to_string(),
            start_time: now.clone(),
            end_time: now.clone(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            "INSERT INTO SESSION
                (NAME, START_TIME, END_TIME, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &self.name,
                &self.start_time,
                &self.end_time,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted session");
                Ok(())
            },
            Err(err) => {
                println!("[session::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<Session>> {
        let mut stmt = conn.prepare("SELECT * FROM SESSION")?;

        let sessions: Result<Vec<Session>> = stmt
            .query_map([], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    start_time: row.get(2)?,
                    end_time: row.get(3)?,
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            })?
            .collect();
        sessions
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {
        self.updated = chrono::offset::Local::now().to_string();

        let result = conn.execute(
            "UPDATE SESSION
                SET NAME=?1,
                    START_TIME=?2,
                    END_TIME=?3,
                    UPDATED=?4
                WHERE ID=?5",
            params![
                &self.name,
                &self.start_time,
                &self.end_time,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated session");
                Ok(())
            },
            Err(err) => {
                println!("[session::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM SESSION WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted session");
                Ok(())
            },
            Err(err) => {
                println!("[session::delete] Error deleting session {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<Session> {
        conn.query_row(
            "SELECT * FROM SESSION WHERE ID = ?",
            &[&id],
            |row| {
                Ok(Session {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    start_time: row.get(2)?,
                    end_time: row.get(3)?,
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            }
        )
    }

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Session>> {
        let query = format!(
            "SELECT * FROM SESSION WHERE NAME LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;

        let sessions: Result<Vec<Session>> = stmt
            .query_map([], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    start_time: row.get(2)?,
                    end_time: row.get(3)?,
                    created: row.get(4)?,
                    updated: row.get(5)?,
                })
            })?
            .collect();
        sessions
    }

}

use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub created: String,
    pub updated: String,
}

impl Session {

    pub fn new(name: &str, description: &str) -> Session {
        let now = chrono::offset::Local::now().to_string();
        Session {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        let result = conn.execute(
            "INSERT INTO SESSION
                (NAME, DESCRIPTION, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &self.name,
                &self.description,
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
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
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
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            }
        )
    }

    pub fn add_deck_to_session(&mut self, conn: &Connection, deck_id: &str) -> Result<String> {
        
        let result = conn.execute(
            "INSERT INTO SESSION_DECK
                (SESSION_ID, DECK_ID, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4)",
            params![
                &self.id,
                deck_id,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully added deck to session");
                Ok(("Success").to_string())
            },
            Err(err) => {
                println!("[session::add_deck_to_session] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn remove_deck_from_session(&mut self, conn: &Connection, deck_id: &str) -> Result<String> {
        
        let result = conn.execute(
            "DELETE FROM SESSION_DECK
                WHERE SESSION_ID = ?1 AND DECK_ID = ?2",
            params![
                &self.id,
                deck_id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully removed deck from session");
                Ok(("Success").to_string())
            },
            Err(err) => {
                println!("[session::remove_deck_from_session] duckdb error {:?}", err);
                Err(err)
            }
        }
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
                    description: row.get(2)?,
                    created: row.get(3)?,
                    updated: row.get(4)?,
                })
            })?
            .collect();
        sessions
    }



}

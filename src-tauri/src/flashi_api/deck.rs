use chrono;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: usize,
    pub subject_id: usize,
    pub name: String,
    pub tags: Vec<String>,
    pub description: String,
    pub created: String,
    pub updated: String
}

impl Deck {
    
    pub fn new(subject_id: usize, name: &str, tags: Vec<String>, description: String) -> Deck {
        Deck {
            id: 0,
            subject_id: subject_id,
            name: name.to_string(),
            tags: tags.iter().map(|k| k.to_string()).collect(),
            description: description.to_string(),
            created: chrono::offset::Local::now().to_string(),
            updated: chrono::offset::Local::now().to_string(),
        }       
    }

    pub fn parse_tags(raw: String) -> Vec<String> {
        raw.trim_matches(|c| c == '[' || c == ']')
            .split(", ")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_matches('\'').to_string())
            .collect()
    }   

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {

        let tags_literal = format!(
            "[{}]",
            self.tags
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''")))
                .collect::<Vec<_>>()
                .join(", ")
        );

        let sql = format!(
            "INSERT INTO DECK 
                (SUBJECT_ID, NAME, TAGS, DESCRIPTION, CREATED, UPDATED)
            VALUES (?1, ?2, {}, ?3, ?4, ?5)",
            tags_literal
        );

        let result = conn.execute(
            &sql, 
            params![
                &self.subject_id,
                &self.name,
                &self.description,
                &self.created,
                &self.updated 
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted deck");
                return Ok(())
            },
            Err(err) => {
                println!("[deck::insert] duckdb error {:?}", err);
                return Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<Deck>> {
        let mut stmt = conn.prepare("SELECT * FROM DECK")?;

        let decks: Result<Vec<Deck>> = stmt
            .query_map([], |row| {
                let tags_raw: String = row.get(2)?;
                Ok(Deck {
                    id: row.get(0)?,
                    subject_id: row.get(1)?,
                    name: row.get(2)?,
                    tags: Self::parse_tags(tags_raw),
                    description: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            })?
            .collect();
        decks
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {

        self.updated = chrono::offset::Local::now().to_string();

        let tags_literal = format!(
            "[{}]",
            self.tags
                .iter()
                .map(|k| format!("'{}'", k.replace("'", "''"))) // escape single quotes
                .collect::<Vec<_>>()
                .join(", ")
        );

        let sql = format!(
            "UPDATE DECK
                SET SUBJECT_ID=?1,
                    NAME=?2,
                    TAGS={},
                    DESCRIPTION=?3,
                    UPDATED=?4
                WHERE ID=?5",
            tags_literal
        );

        let result = conn.execute(
            &sql,
            params![
                &self.subject_id,
                &self.name,
                &self.description,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated deck");
                return Ok(())
            },
            Err(err) => {
                println!("[deck::update] duckdb error {:?}", err);
                return Err(err)
            }
        }
    
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM DECK WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Succesfully deleted deck"); 
                return Ok(());
            },
            Err(err) => {
                println!("[deck::delete] Error deleting deck {:?}", err); 
                return Err(err);
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<Deck> {
        let query = "SELECT * FROM DECK WHERE ID = ?";
        conn.query_row(query, &[&id], |row| {
            let tags_raw: String = row.get(2)?;
            Ok(Deck {
                id: row.get(0)?,
                subject_id: row.get(1)?,
                name: row.get(2)?,
                tags: Self::parse_tags(tags_raw),
                description: row.get(4)?,
                created: row.get(5)?,
                updated: row.get(6)?,
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Deck>> {

        let query = format!(
            "SELECT * FROM DECK WHERE NAME LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;
        let decks: Result<Vec<Deck>> = stmt
            .query_map([], |row| { 
                let tags_raw: String = row.get(2)?;
                Ok(Deck {
                    id: row.get(0)?,
                    subject_id: row.get(1)?,
                    name: row.get(2)?,
                    tags: Self::parse_tags(tags_raw),
                    description: row.get(4)?,
                    created: row.get(5)?,
                    updated: row.get(6)?,
                })
            })?
            .collect();
        decks
    }

}

use chrono;
use std::fs;
use std::path::Path;
use std::io::ErrorKind;
use tauri::Manager;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

use crate::flashi_api::queries;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: usize,
    pub subject_id: usize,
    pub name: String,
    pub tags: Vec<String>,
    pub description: String,
    pub reviewed: usize,
    pub created: String,
    pub updated: String
}

impl Deck {
    
    pub fn new(subject_id: usize, name: &str, tags: Vec<String>, description: &str) -> Deck {
      
        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        Deck {
          id: 0,
          subject_id: subject_id,
          name: name.to_string(),
          tags: tags.iter().map(|k| k.to_string()).collect(),
          description: description.to_string(),
          reviewed: 0,
          created: now.clone(),
          updated: now,
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
                (SUBJECT_ID, NAME, TAGS, DESCRIPTION, REVIEWED, CREATED, UPDATED)
            VALUES (?1, ?2, {}, ?3, ?4, ?5, ?6)",
            tags_literal
        );

        let result = conn.execute(
            &sql, 
            params![
                &self.subject_id,
                &self.name,
                &self.description,
                &self.reviewed,
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
        let mut stmt = conn.prepare("SELECT ID, SUBJECT_ID, NAME, TAGS::VARCHAR, DESCRIPTION, REVIEWED, CREATED::VARCHAR, UPDATED::VARCHAR FROM DECK")?;

        let decks: Result<Vec<Deck>> = stmt
            .query_map([], |row| {
                let tags_raw: String = row.get(3)?;
                Ok(Deck {
                    id: row.get(0)?,
                    subject_id: row.get(1)?,
                    name: row.get(2)?,
                    tags: Self::parse_tags(tags_raw),
                    description: row.get(4)?,
                    reviewed: row.get(5)?,
                    created: row.get(6)?,
                    updated: row.get(7)?,
                })
            })?
            .collect();
        decks
    }

    pub fn retrieve_by_subject(conn: &Connection, subject_id: &str) -> Result<Vec<Deck>> {
        let mut stmt = conn.prepare("SELECT ID, SUBJECT_ID, NAME, TAGS::VARCHAR, DESCRIPTION, REVIEWED, CREATED::VARCHAR, UPDATED::VARCHAR FROM DECK WHERE SUBJECT_ID = ?")?;

        let decks: Result<Vec<Deck>> = stmt
            .query_map([&subject_id], |row| {
                let tags_raw: String = row.get(3)?;
                Ok(Deck {
                    id: row.get(0)?,
                    subject_id: row.get(1)?,
                    name: row.get(2)?,
                    tags: Self::parse_tags(tags_raw),
                    description: row.get(4)?,
                    reviewed: row.get(5)?,
                    created: row.get(6)?,
                    updated: row.get(7)?,
                })
            })?
            .collect();
        decks
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {

        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
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
                    REVIEWED=?4,
                    UPDATED=?5
                WHERE ID=?6",
            tags_literal
        );

        let result = conn.execute(
            &sql,
            params![
                &self.subject_id,
                &self.name,
                &self.description,
                &self.reviewed,
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
        let query = "SELECT ID, SUBJECT_ID, NAME, TAGS::VARCHAR, DESCRIPTION, REVIEWED, CREATED::VARCHAR, UPDATED::VARCHAR FROM DECK WHERE ID = ?";
        conn.query_row(query, &[&id], |row| {
            let tags_raw: String = row.get(3)?;
            Ok(Deck {
                id: row.get(0)?,
                subject_id: row.get(1)?,
                name: row.get(2)?,
                tags: Self::parse_tags(tags_raw),
                description: row.get(4)?,
                reviewed: row.get(5)?,
                created: row.get(6)?,
                updated: row.get(7)?,
            })
        })
    }
    

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Deck>> {

        let mut stmt = conn.prepare(&queries::SEARCH_DECK)?;

        let decks: Result<Vec<Deck>> = stmt
            .query_map([search_term], |row| { 
                let tags_raw: String = row.get(3)?;
                Ok(Deck {
                    id: row.get(0)?,
                    subject_id: row.get(1)?,
                    name: row.get(2)?,
                    tags: Self::parse_tags(tags_raw),
                    description: row.get(4)?,
                    reviewed: row.get(5)?, 
                    created: row.get(6)?,
                    updated: row.get(7)?,
                })
            })?
            .collect();
        decks
    }

    pub fn import(conn: &Connection, id: &str, subject_id: &str, path: &str) -> Result<()> {
       
        // Look for a parquet file in the folder, format the full path to the parquet file
        let import_directory = Path::new(path);
        let parquet_path = fs::read_dir(import_directory)
            .map_err(|_| duckdb::Error::InvalidPath(import_directory.to_path_buf()))?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .find(|p| {
                p.is_file() && p.extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("parquet"))
            })
            .ok_or_else(|| duckdb::Error::InvalidPath(import_directory.to_path_buf()))?; 

        let parquet_path_str = parquet_path
            .to_str()
            .ok_or_else(|| duckdb::Error::InvalidPath(parquet_path.clone()))?
            .replace('\'', "''");

        println!("PARQUET PATH: {:?}", parquet_path_str);


        // format the /images folder on top of the path
        
        // Copy the images to the images directory of the application


        /*
        let import_deck_sql: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"), 
            "/database/sql/ROUTINES/IMPORT_DECK.sql"
        ));

        let sql = import_deck_sql
            .replace("{path}", &path.replace('\'', "''"))
            .replace("{subject_id}", &subject_id.to_string()); 

        let tx = conn.transaction()?; 
        tx.execute_batch(&sql)?; 
        tx.commit() */
        Ok(())

    }

    pub fn export(&mut self, conn: &Connection, deck_id: &str, path: &str, app_image_dir: &Path) -> Result<()> {
       
        let curr_time = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let deck_id_parsed: i64 = deck_id.parse().map_err(|_| {
            duckdb::Error::InvalidParameterName(format!("Invalid deck id: {deck_id}"))
        })?; 

        let safe_name: String = self.name
            .trim()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect();
        let deck_export_name = format!("{0}_{curr_time}", safe_name); 

        let format_path = Path::new(path).join(&deck_export_name);
        fs::create_dir_all(&format_path)
            .map_err(|_| duckdb::Error::InvalidPath(format_path.clone()))?; 

        // Extract all images associated with deck and put in images sub folder
        let mut image_query_stmt = conn.prepare(queries::DECK_IMAGE_PATHS)?;
        let rows: Vec<String> = image_query_stmt
            .query_map(params![deck_id], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<String>>>()?;
        println!("Image rows: {:?}", rows);

        // Place deck structured data csv file in root of export folder  
        let export_images_dir = format_path.join("images");
        fs::create_dir_all(&export_images_dir)
            .map_err(|_| duckdb::Error::InvalidPath(export_images_dir.clone()))?; 

        for image_file in &rows {

            let src = app_image_dir.join(image_file);
            let Some(file_name) = src.file_name() else {
                eprintln!("[deck::export] Skipping image with no file name: {image_file}");
                continue;
            }; 

            let dest = export_images_dir.join(file_name);
            if let Err(e) = fs::copy(src, &dest) {
                eprintln!("[deck::export] failed to copy {image_file} -> {:?}: {e}", dest); 
            }
        }
    
        let parquet_file_name = format!("{0}.parquet", deck_export_name);
        let parquet_path = format_path.join(parquet_file_name); 
        let parquet_path_str = parquet_path
            .to_str()
            .ok_or_else(|| duckdb::Error::InvalidPath(parquet_path.clone()))?
            .replace('\'', "''");

        let parquet_sql = queries::EXPORT_DECK_PARQUET
            .replace("{deck_id}", &deck_id_parsed.to_string())
            .replace("{path}", &parquet_path_str); 

        match conn.execute_batch(&parquet_sql) {
            Ok(_) => {
                println!("Succesfully exported deck"); 
                return Ok(());
            },
            Err(err) => {
                println!("[deck::export] Error exporting deck {:?}", err); 
                return Err(err);
            }
        }

    }

}

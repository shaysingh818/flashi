use chrono;
use std::io;
use std::fs;
use std::fmt;
use std::ffi::OsStr;
use std::path::Path;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub deck_id: usize,
    pub answer_type_id: usize,
    pub answer_id: usize,
    pub reviewed: usize,
    pub correct: usize, 
    pub incorrect: usize,
    pub raw: String,
    pub created: String,
    pub updated: String
}

impl Question {

    pub fn new(deck_id: usize, answer_type_id: usize, answer_id: usize, raw: &str) -> Question {

        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      
        Question {
            id: 0,
            deck_id: deck_id,
            answer_type_id: answer_type_id,
            answer_id: answer_id,
            reviewed: 0,
            correct: 0, 
            incorrect: 0,
            raw: raw.to_string(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
 
        let result = conn.execute(
            "INSERT INTO QUESTION 
                (DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &self.deck_id,
                &self.answer_type_id,
                &self.answer_id,
                &self.reviewed,
                &self.correct,
                &self.incorrect,                
                &self.raw,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted question");
                Ok(())
            },
            Err(err) => {
                println!("[question::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection, deck_id: &str) -> Result<Vec<Question>> {
 
        let mut stmt = conn.prepare("SELECT ID, DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED::VARCHAR AS CREATED, UPDATED::VARCHAR AS UPDATED FROM QUESTION WHERE DECK_ID = ?")?;

        let questions: Result<Vec<Question>> = stmt
            .query_map([&deck_id], |row| {
                Ok(Question {
                    id: row.get("ID")?,
                    deck_id: row.get("DECK_ID")?,
                    answer_type_id: row.get("ANSWER_TYPE_ID")?,
                    answer_id: row.get("ANSWER_ID")?,
                    reviewed: row.get("REVIEWED")?,
                    correct: row.get("CORRECT")?, 
                    incorrect: row.get("INCORRECT")?,
                    raw: row.get("RAW")?,
                    created: row.get("CREATED")?,
                    updated: row.get("UPDATED")?,
                })
            })?
            .collect();
        questions
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {

      self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            "UPDATE QUESTION
                SET 
                    RAW=?1,
                    REVIEWED=?2, 
                    CORRECT=?3, 
                    INCORRECT=?4,
                    UPDATED=?5
                WHERE ID=?6",
            params![
                &self.raw,
                &self.reviewed,
                &self.correct,
                &self.incorrect,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated question");
                Ok(())
            },
            Err(err) => {
                println!("[question::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM QUESTION WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted question");
                Ok(())
            },
            Err(err) => {
                println!("[question::delete] Error deleting question {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<Question> {
        conn.query_row(
            "SELECT ID, DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED::VARCHAR, UPDATED::VARCHAR FROM QUESTION WHERE ID = ?",
            &[&id],
            |row| {
                Ok(Question {
                    id: row.get(0)?,
                    deck_id: row.get(1)?,
                    answer_type_id: row.get(2)?,
                    answer_id: row.get(3)?,
                    reviewed: row.get(4)?,
                    correct: row.get(5)?,
                    incorrect: row.get(6)?,
                    raw: row.get(7)?,
                    created: row.get(8)?,
                    updated: row.get(9)?,
                })
            }
        )
    }

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<Question>> {
        let query = format!(
            "SELECT ID, DECK_ID, ANSWER_TYPE_ID, ANSWER_ID, REVIEWED, CORRECT, INCORRECT, RAW, CREATED::VARCHAR, UPDATED::VARCHAR FROM QUESTION WHERE RAW LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;

        let questions: Result<Vec<Question>> = stmt
            .query_map([], |row| {
                Ok(Question {
                    id: row.get(0)?,
                    deck_id: row.get(1)?,
                    answer_type_id: row.get(2)?,
                    answer_id: row.get(3)?,
                    reviewed: row.get(4)?,
                    correct: row.get(5)?,
                    incorrect: row.get(6)?,
                    raw: row.get(7)?,
                    created: row.get(8)?,
                    updated: row.get(9)?,
                })
            })?
            .collect();
        questions
    }

    pub fn add_image_artifact_to_question(&mut self, conn: &Connection, image_id: &str) -> Result<String> {

        let result = conn.execute(
            "INSERT INTO QUESTION_IMAGE
                (QUESTION_ID, IMAGE_ID, CREATED, UPDATED)
            VALUES (?1, ?2, ?3, ?4)",
            params![
                &self.id,
                image_id,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully added image artifact to question");
                Ok(("Success").to_string())
            },
            Err(duckdb::Error::DuckDBFailure(err, msg)) => {
                let msg_str = msg.as_deref().unwrap_or("");
                if msg_str.contains("violates unique constraint") {
                    println!("Image artifact already associated with question, skipping");
                    Ok("Duplicate".to_string())
                } else {
                    println!("[question::add_image_artifact_to_question] duckdb error {:?}", msg_str);
                    Err(duckdb::Error::DuckDBFailure(err, msg))
                }
            },
            Err(err) => {
                println!("[question::add_image_artifact_to_question] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageArtifact {
    pub id: usize,
    pub image_path: String,
    pub created: String,
    pub updated: String,
}

impl ImageArtifact {

    pub fn new(image_path: &str) -> ImageArtifact {

        let now = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      
        ImageArtifact {
            id: 0,
            image_path: image_path.to_string(),
            created: now.clone(),
            updated: now,
        }
    }

    pub fn insert(&mut self, conn: &Connection) -> Result<()> {
        
        let result = conn.execute(
            "INSERT INTO IMAGE_ARTIFACT 
                (IMAGE_PATH, CREATED, UPDATED)
            VALUES (?1, ?2, ?3)",
            params![
                &self.image_path,
                &self.created,
                &self.updated,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully inserted image artifact");
                Ok(())
            },
            Err(err) => {
                println!("[image_artifact::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<ImageArtifact>> {
        
        let mut stmt = conn.prepare("SELECT ID, IMAGE_PATH, CREATED::VARCHAR, UPDATED::VARCHAR FROM IMAGE_ARTIFACT")?;

        let image_artifacts: Result<Vec<ImageArtifact>> = stmt
            .query_map([], |row| {
                Ok(ImageArtifact {
                    id: row.get(0)?,
                    image_path: row.get(1)?,
                    created: row.get(2)?,
                    updated: row.get(3)?,
                })
            })?
            .collect();
        image_artifacts
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {

        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            "UPDATE IMAGE_ARTIFACT
                SET IAMGE_PATH=?1,
                    UPDATED=?2
                WHERE ID=?3",
            params![
                &self.image_path,
                &self.updated,
                id,
            ],
        );

        match result {
            Ok(_) => {
                println!("Successfully updated image artifact");
                Ok(())
            },
            Err(err) => {
                println!("[image_artifact::update] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, conn: &Connection, id: &str) -> Result<()> {
        let result = conn.execute("DELETE FROM IMAGE_ARTIFACT WHERE ID=?", [id]);
        match result {
            Ok(_) => {
                println!("Successfully deleted image artifact");
                Ok(())
            },
            Err(err) => {
                println!("[image_artifact::delete] Error deleting image artifact {:?}", err);
                Err(err)
            }
        }
    }

    pub fn view(conn: &Connection, id: &str) -> Result<ImageArtifact> {
        conn.query_row(
            "SELECT ID, IMAGE_PATH, CREATED::VARCHAR, UPDATED::VARCHAR FROM IMAGE_ARTIFACT WHERE ID = ?",
            &[&id],
            |row| {
                Ok(ImageArtifact {
                    id: row.get(0)?,
                    image_path: row.get(1)?,
                    created: row.get(2)?,
                    updated: row.get(3)?,
                })
            }
        )
    }

    pub fn search(conn: &Connection, search_term: &str) -> Result<Vec<ImageArtifact>> {
        let query = format!(
            "SELECT ID, IMAGE_PATH, CREATED::VARCHAR, UPDATED::VARCHAR FROM IMAGE_ARTIFACT WHERE NAME LIKE '%{}%'",
            search_term
        );
        let mut stmt = conn.prepare(&query)?;

        let image_artifacts: Result<Vec<ImageArtifact>> = stmt
            .query_map([], |row| {
                Ok(ImageArtifact {
                    id: row.get(0)?,
                    image_path: row.get(1)?,
                    created: row.get(2)?,
                    updated: row.get(3)?,
                })
            })?
            .collect();
        image_artifacts
    }

}



#[derive(Debug)]
pub enum UploadImageError {
    /// The provided file name has no extension (e.g. "photo" instead of "photo.jpg").
    MissingExtension(String),
    /// Copying the file from its local path to the thumbnail directory failed.
    Copy {
        source: io::Error,
        from: String,
        to: String,
    },
}

impl fmt::Display for UploadImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UploadImageError::MissingExtension(name) => {
                write!(f, "'{name}' has no file extension")
            }
            UploadImageError::Copy { source, from, to } => {
                write!(f, "failed to copy '{from}' to '{to}': {source}")
            }
        }
    }
}

impl std::error::Error for UploadImageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UploadImageError::Copy { source, .. } => Some(source),
            UploadImageError::MissingExtension(_) => None,
        }
    }
}

pub fn upload_image(
    image_file_name: &str,
    user_thumbnail_path: &str,
    user_local_image_path: &str,
) -> Result<String, UploadImageError> {
    if image_file_name.is_empty() {
        return Ok("default.webp".to_string());
    }

    let image_ext = Path::new(image_file_name)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| UploadImageError::MissingExtension(image_file_name.to_string()))?;

    let image_file_name_final = format!("{}.{}", Uuid::new_v4(), image_ext);
    let dest_path = format!("{user_thumbnail_path}/{image_file_name_final}");

    let bytes = fs::copy(user_local_image_path, &dest_path).map_err(|source| {
        UploadImageError::Copy {
            source,
            from: user_local_image_path.to_string(),
            to: dest_path.clone(),
        }
    })?;

    println!("Successfully copied thumbnail with {bytes} bytes ({dest_path})");

    Ok(image_file_name_final)
}
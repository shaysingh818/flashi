use chrono;
use std::io;
use std::fs;
use std::fmt;
use std::ffi::OsStr;
use std::path::Path;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use duckdb::{params, Connection, Result}; 

use crate::queries::image_artifact::*;


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

    pub fn insert(&mut self, conn: &Connection) -> Result<i64> {
        
        let result = conn.prepare(INSERT_IMAGE_ARTIFACT)
          .and_then(|mut stmt| {
              stmt.query_row(
                  params![&self.image_path, &self.created, &self.updated],
                  |row| row.get::<_, i64>(0),
              )
          });
        
        match result {
            Ok(id) => {
                println!("Successfully inserted image artifact with id: {:?}", id);
                Ok(id)
            },
            Err(err) => {
                println!("[image_artifact::insert] duckdb error {:?}", err);
                Err(err)
            }
        }
    }

    pub fn retrieve(conn: &Connection) -> Result<Vec<ImageArtifact>> {
        
        let mut stmt = conn.prepare(RETRIEVE_IMAGE_ARTIFACTS)?;

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

    pub fn retrieve_question_image_artifacts(conn: &Connection, question_id: &str) -> Result<Vec<ImageArtifact>> {

        let mut stmt = conn.prepare(RETRIEVE_QUESTION_IMAGE_ARTIFACTS)?;

        let images: Result<Vec<ImageArtifact>> = stmt
            .query_map(params![question_id], |row| {
                Ok(ImageArtifact {
                    id: row.get(0)?,
                    image_path: row.get(1)?,
                    created: row.get(2)?,
                    updated: row.get(3)?,   
                })
            })?
            .collect();
        images
    }

    pub fn update(&mut self, conn: &Connection, id: &str) -> Result<()> {

        self.updated = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            UPDATE_IMAGE_ARTIFACT,
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
        let result = conn.execute(DELETE_IMAGE_ARTIFACT, [id]);
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
            VIEW_IMAGE_ARTIFACT,
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

        let mut stmt = conn.prepare(&SEARCH_IMAGE_ARTIFACT)?;

        let image_artifacts: Result<Vec<ImageArtifact>> = stmt
            .query_map([search_term], |row| {
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

#[derive(Debug)]
pub enum RemoveImageError {
    /// Deleting the file at the given path failed.
    Delete {
        source: io::Error,
        path: String,
    },
}

impl fmt::Display for RemoveImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemoveImageError::Delete { source, path } => {
                write!(f, "failed to delete '{path}': {source}")
            }
        }
    }
}

impl std::error::Error for RemoveImageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RemoveImageError::Delete { source, .. } => Some(source),
        }
    }
}

pub fn upload_image(
    image_file_name: &str,    
    user_local_image_path: &str,
    app_image_path: &str,
) -> Result<String, UploadImageError> {
    
    if image_file_name.is_empty() {
        return Ok("default.webp".to_string());
    }

    let image_ext = Path::new(image_file_name)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| UploadImageError::MissingExtension(image_file_name.to_string()))?;

    let image_file_name_final = format!("{}.{}", Uuid::new_v4(), image_ext);
    let src_path = format!("{user_local_image_path}/{image_file_name}"); 
    let dest_path = format!("{app_image_path}/{image_file_name_final}");

    let bytes = fs::copy(&src_path, &dest_path).map_err(|source| {
        UploadImageError::Copy {
            source,
            from: src_path.to_string(),
            to: dest_path.clone(),
        }
    })?;

    println!("Successfully copied thumbnail with {bytes} bytes ({dest_path})");

    Ok(image_file_name_final)
}

pub fn remove_image(
    image_file_name: &str,
    app_image_path: &str,
) -> Result<String, RemoveImageError> {
    let image_delete_path = format!("{}/{}", app_image_path, image_file_name);

    fs::remove_file(&image_delete_path).map_err(|source| RemoveImageError::Delete {
        source,
        path: image_delete_path.clone(),
    })?;

    println!("Successfully removed thumbnail {}", image_delete_path);
    Ok(image_file_name.to_string())
}


#[tauri::command]
pub fn create_image_artifact(
    image_file_name: &str,
    user_local_image_path: &str, 
    app_image_path: &str,
    db_path: &str) -> (i64, String) {
 
    let image_file_name_final = match upload_image(
        image_file_name,
        user_local_image_path,
        app_image_path,
      ) {
          Ok(name) => name,
          Err(e) => return (-1, format!("Failed to upload image: {}", e)),
      };

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return (-1, format!("Failed to open database: {}", e)),
    };
 
    let mut image_artifact: ImageArtifact = ImageArtifact::new(&image_file_name_final);
    match image_artifact.insert(&conn) {
        Ok(id) => {
            let _ = conn.close();
            return (id, "Success".to_string());
        }
        Err(e) => (-1, format!("Failed to insert question: {}", e)),
    }
}


#[tauri::command]
pub fn view_question_image_artifacts(db_path: &str, question_id: &str) -> Vec<ImageArtifact> {
 
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match ImageArtifact::retrieve_question_image_artifacts(&conn, question_id) {
        Ok(images) => images,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}


#[tauri::command]
pub fn edit_image_artifact(
    id: &str,    
    image_file_name: &str,
    user_local_image_path: &str, 
    user_image_path: &str,
    db_path: &str) -> String {
 
    let image_file_name_final = match upload_image(
        image_file_name,
        user_local_image_path,
        user_image_path,
      ) {
          Ok(name) => name,
          Err(e) => return format!("Failed to upload image: {}", e),
      };

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    match ImageArtifact::view(&conn, id) {
        Ok(mut image_artifact) => {
            image_artifact.image_path = image_file_name_final; 
            match image_artifact.update(&conn, id) {
                Ok(()) => {
                    let _ = conn.close();
                    match remove_image(
                          image_file_name,
                          user_image_path,
                      ) {
                          Ok(name) => println!("removed image file name: {:?} ", name),
                          Err(e) => return format!("Failed to remove image: {}", e),
                      };
                    
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update deck: {}", e),
            }
            
        }
        Err(e) => {            
            let msg = format!("Error updating image artifact: {:?}", e);
            return msg.to_string();
        }
    }
 
}


#[tauri::command]
pub fn delete_image_artifact(
    id: &str,
    image_file_name: &str,
    app_image_path: &str,
    db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };
    
    match ImageArtifact::view(&conn, id) {
        Ok(mut image_artifact) => {

            match image_artifact.delete(&conn, id) {
                Ok(_) => {

                    match remove_image(image_file_name, app_image_path) {
                          Ok(name) => println!("Removed image file name: {:?}", name),
                          Err(e) => return format!("Failed to remove image: {}", e),
                      };
                    
                    return format!("Success");
                },
                Err(err) => {
                    return format!("Error deleting question {}: {:?}", id, err); 
                }
            }    
        }
        Err(e) => {
            return format!("Error retrieving question with id: {:?} Cause: {:?}", id, e);
        }
    }
 
}

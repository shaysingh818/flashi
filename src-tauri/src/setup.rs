use std::io;
use std::fs;
use std::path::Path;
use reqwest::StatusCode;
use duckdb::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use crate::settings::{write_settings, Settings};

const DATABASE_URL: &str = "https://raw.githubusercontent.com/shaysingh818/flashi/main/src-tauri/database/build/flashi.db";


pub fn fetch_database_file(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    
    let resp = reqwest::blocking::Client::builder()
        .user_agent("flashi")
        .build()?
        .get(url)
        .send()?; 

    match resp.status() {
        StatusCode::OK => {}
        StatusCode::NOT_FOUND => return Err("database file not found (404)".into()),
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => return Err("not authorized".into()),
        other => return Err(format!("unexpected status: {other}").into()),
    }

    let bytes = resp.bytes()?;
    fs::write(dest, &bytes)?;
    Ok(())

}


pub fn setup_database(app_path: &Path) -> Result<(), Box<dyn std::error::Error>> {

    let database_path = Path::new(app_path)
        .join("metadata")
        .join("flashi.db");

    if database_path.is_file() {
        return Ok(());
    }

    let download_path = database_path.with_extension("db.download");
    fetch_database_file(DATABASE_URL, &download_path)?;

    if let Err(e) = Connection::open(&download_path) {
        let _ = fs::remove_file(&download_path);
        return Err(format!("downloaded database is not valid: {e}").into());
    }

    fs::rename(&download_path, &database_path)?;
    Ok(())

}

pub fn setup_directories(app_path: &Path) -> io::Result<()> {

    let metadata_path = Path::new(app_path).join("metadata");
    fs::create_dir_all(&metadata_path)?;

    let images_path = Path::new(app_path).join("images");
    fs::create_dir_all(&images_path)?;

    Ok(())

}

pub fn setup_settings_files(app_path: &Path, export_path: &Path) -> Result<(), Box<dyn std::error::Error>> {

    let settings_path = Path::new(app_path)
        .join("metadata")
        .join("settings.json");

    if settings_path.is_file() {
        return Ok(());
    }

    let settings = Settings {
        export_path: export_path.to_string_lossy().into_owned(),
    };

    write_settings(&settings_path, &settings)?;
    Ok(())

}


#[tauri::command]
pub async fn setup(app: tauri::AppHandle) -> Result<(), String> {

    // matches the `${dataDir}/flashi` path the frontend builds
    let app_path = app.path()
        .data_dir()
        .map_err(|e| format!("Failed to resolve data directory: {}", e))?
        .join("flashi");

    // exports land in the user's documents folder until they pick another one
    let export_path = app.path()
        .document_dir()
        .unwrap_or_else(|_| app_path.clone());

    // the download blocks, keep it off the main thread
    let result = tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        setup_directories(&app_path)
            .map_err(|e| format!("Failed to create app directories: {}", e))?;
        setup_database(&app_path)
            .map_err(|e| format!("Failed to set up database: {}", e))?;
        setup_settings_files(&app_path, &export_path)
            .map_err(|e| format!("Failed to set up settings file: {}", e))?;
        Ok(())
    }).await;

    match result {
        Ok(outcome) => outcome,
        Err(e) => Err(format!("Setup task failed: {}", e)),
    }
}


#[cfg(test)]
mod setup_tests {

    use super::*;

    #[test]
    fn test_fetch_database_file() -> Result<(), Box<dyn std::error::Error>> {

        let url = "https://raw.githubusercontent.com/shaysingh818/flashi/main/src-tauri/database/build/flashi.db";
        
        let tmp = tempfile::tempdir().unwrap();
        let dest = tmp.path().join("flashi.db");
        fetch_database_file(url, &dest).unwrap();

        assert_eq!(dest.is_file(), true);

        let db_path: &str = dest.to_str().ok_or("path is not valid UTF-8")?; 

        let conn = match Connection::open(db_path) {
            Ok(connection) => connection,
            Err(e) => panic!("Failed to open downloaded database: {e:?}"),
        };

        let mut stmt = conn.prepare("SHOW TABLES")?;

        let mut tables: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, duckdb::Error>>()?;

        let mut expected_tables = vec![
            "ANSWER_TYPE", "MULTIPLE_CHOICE", "MULTIPLE_CHOICE_ANSWER", 
            "OPEN_ENDED_ANSWER", "ORDERED_SEQUENCE_ANSWER", "QUESTION",
            "QUESTION_IMAGE", "SESSION", "SESSION_DECK", "SESSION_REVIEW",
            "SESSION_REVIEW_METRICS", "SESSION_REVIEW_QUESTION", "SUBJECT",
            "SUBJECT_DECK", "DECK", "IMAGE_ARTIFACT", "flyway_schema_history"
        ];

        assert_eq!(tables.len(), expected_tables.len());

        tables.sort();
        expected_tables.sort(); 

        assert_eq!(tables, expected_tables);
        Ok(())
 
    }

    #[test]
    fn test_setup_database() {

        let tmp = tempfile::tempdir().unwrap();
        setup_directories(tmp.path()).unwrap();
        setup_database(tmp.path()).unwrap();

        let metadata_path = tmp.path().join("metadata");
        let database_path = metadata_path.join("flashi.db");

        assert_eq!(database_path.is_file(), true);
        assert_eq!(metadata_path.join("flashi.db.download").exists(), false);

        let conn = Connection::open(&database_path).unwrap();
        let table_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM information_schema.tables", [], |row| row.get(0))
            .unwrap();

        assert_eq!(table_count > 0, true);

    }

    #[test]
    fn test_setup_database_keeps_existing() {

        let tmp = tempfile::tempdir().unwrap();
        setup_directories(tmp.path()).unwrap();

        let database_path = tmp.path().join("metadata").join("flashi.db");
        fs::write(&database_path, b"existing").unwrap();

        setup_database(tmp.path()).unwrap();

        assert_eq!(fs::read(&database_path).unwrap(), b"existing");

    }

    #[test]
    fn test_setup_settings_files() {

        let tmp = tempfile::tempdir().unwrap();
        setup_directories(tmp.path()).unwrap();
        setup_settings_files(tmp.path(), Path::new("/tmp/exports")).unwrap();

        let settings_path = tmp.path().join("metadata").join("settings.json");
        assert_eq!(settings_path.is_file(), true);

        let settings = crate::settings::read_settings(&settings_path).unwrap();
        assert_eq!(settings.export_path, "/tmp/exports");

    }

    #[test]
    fn test_setup_settings_files_keeps_existing() {

        let tmp = tempfile::tempdir().unwrap();
        setup_directories(tmp.path()).unwrap();

        setup_settings_files(tmp.path(), Path::new("/tmp/exports")).unwrap();
        setup_settings_files(tmp.path(), Path::new("/tmp/other")).unwrap();

        let settings_path = tmp.path().join("metadata").join("settings.json");
        let settings = crate::settings::read_settings(&settings_path).unwrap();
        assert_eq!(settings.export_path, "/tmp/exports");

    }

    #[test]
    fn test_directories_setup() {

        let tmp = tempfile::tempdir().unwrap();
        setup_directories(tmp.path()).unwrap();

        let expected_metadata_path = tmp.path().join("metadata");
        let expected_images_path = tmp.path().join("images");

        assert_eq!(expected_metadata_path.is_dir(), true);
        assert_eq!(expected_images_path.is_dir(), true);

    }
 

}


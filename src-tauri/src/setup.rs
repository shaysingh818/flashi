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

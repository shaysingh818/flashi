use std::io;
use std::fs;
use std::fmt;
use std::ffi::OsStr;
use std::path::Path;
use chrono;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};


pub fn setup_database(app_path: &Path) {
    todo!() 

    let database_path = Path::new(app_path)
        .join("metadata")
        .join("flashi.db");


}

pub fn setup_directories(app_path: &Path) -> io::Result<()> {

    let metadata_path = Path::new(app_path).join("metadata");
    fs::create_dir_all(&metadata_path)?;

    let images_path = Path::new(app_path).join("images");
    fs::create_dir_all(&images_path)?;

    Ok(())

}

pub fn setup_settings_files() {
    todo!()
}


#[tauri::command]
pub fn setup(app: tauri::AppHandle) {
    setup_database();
}


#[cfg(test)]
mod setup_tests {

    use super::*;

    /*
    #[test]
    fn setup_handler_runs() {
        let app = tauri::test::mock_app();
        setup(app.handle().clone()).unwrap();
    } */

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


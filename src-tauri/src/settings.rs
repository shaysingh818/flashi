use std::fs; 
use std::path::Path; 
use serde::{Deserialize, Serialize}; 

#[derive(Serialize, Deserialize, Debug)] 
pub struct Settings {
    pub export_path: String
}

pub fn write_settings(path: &Path, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(settings)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn read_settings(path: &Path) -> Result<Settings, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let settings = serde_json::from_str(&contents)?;
    Ok(settings)
}


#[tauri::command]
pub fn read_settings_file(settings_path: &str) -> Result<Settings, String> {
    match read_settings(Path::new(settings_path)) {
        Ok(settings) => Ok(settings),
        Err(e) => Err(format!("Failed to read settings file: {}", e)),
    }
}


#[tauri::command]
pub fn write_settings_file(settings: Settings, settings_path: &str) -> Result<Settings, String> {
    match write_settings(Path::new(settings_path), &settings) {
        Ok(()) => Ok(settings),
        Err(e) => Err(format!("Failed to write settings file: {}", e)),
    }
}


#[cfg(test)]
mod settings_tests {

    use super::*;

    #[test]
    fn test_write_and_read_settings_file() {

        let tmp = tempfile::tempdir().unwrap();
        let settings_path = tmp.path().join("settings.json");
        let settings_path_str = settings_path.to_str().unwrap();

        let settings = Settings { export_path: String::from("/tmp/exports") };
        let written = write_settings_file(settings, settings_path_str).unwrap();
        assert_eq!(written.export_path, "/tmp/exports");

        let read = read_settings_file(settings_path_str).unwrap();
        assert_eq!(read.export_path, "/tmp/exports");

    }

    #[test]
    fn test_read_missing_settings_file() {

        let tmp = tempfile::tempdir().unwrap();
        let settings_path = tmp.path().join("settings.json");

        let result = read_settings_file(settings_path.to_str().unwrap());
        assert_eq!(result.is_err(), true);

    }

}

use std::fs;
use std::path::Path;
use duckdb::Connection;
use flashi_lib::setup::*;
use flashi_lib::settings::read_settings;


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

    let settings = read_settings(&settings_path).unwrap();
    assert_eq!(settings.export_path, "/tmp/exports");

}

#[test]
fn test_setup_settings_files_keeps_existing() {

    let tmp = tempfile::tempdir().unwrap();
    setup_directories(tmp.path()).unwrap();

    setup_settings_files(tmp.path(), Path::new("/tmp/exports")).unwrap();
    setup_settings_files(tmp.path(), Path::new("/tmp/other")).unwrap();

    let settings_path = tmp.path().join("metadata").join("settings.json");
    let settings = read_settings(&settings_path).unwrap();
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

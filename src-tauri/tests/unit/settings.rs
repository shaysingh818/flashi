use std::fs;
use flashi_lib::settings::*;

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

#[test]
fn test_write_settings_overwrites_existing() {

    let tmp = tempfile::tempdir().unwrap();
    let settings_path = tmp.path().join("settings.json");

    write_settings(&settings_path, &Settings { export_path: "/first".to_string() }).unwrap();
    write_settings(&settings_path, &Settings { export_path: "/second".to_string() }).unwrap();

    assert_eq!(read_settings(&settings_path).unwrap().export_path, "/second");

}

#[test]
fn test_read_invalid_settings_file() {

    let tmp = tempfile::tempdir().unwrap();
    let settings_path = tmp.path().join("settings.json");
    fs::write(&settings_path, "not json").unwrap();

    assert_eq!(read_settings(&settings_path).is_err(), true);
    assert_eq!(read_settings_file(settings_path.to_str().unwrap()).is_err(), true);

}

#[test]
fn test_write_settings_to_missing_directory() {

    let tmp = tempfile::tempdir().unwrap();
    let settings_path = tmp.path().join("missing").join("settings.json");

    let settings = Settings { export_path: "/tmp/exports".to_string() };
    assert_eq!(write_settings_file(settings, settings_path.to_str().unwrap()).is_err(), true);

}

use std::fs;
use crate::common::TestDb;
use flashi_lib::image_artifact::*;

#[test]
fn test_create_image_artifact() {

    let db = TestDb::new();
    let conn = db.conn();

    let id = ImageArtifact::new("image1.png").insert(&conn).unwrap();
    assert_eq!(id > 0, true);

    let images = ImageArtifact::retrieve(&conn).unwrap();
    assert_eq!(images.len(), 1);
    assert_eq!(images[0].id, id as usize);
    assert_eq!(images[0].image_path, "image1.png");

}

#[test]
fn test_view_image_artifact() {

    let db = TestDb::new();
    let conn = db.conn();
    let id = ImageArtifact::new("image1.png").insert(&conn).unwrap().to_string();

    assert_eq!(ImageArtifact::view(&conn, &id).unwrap().image_path, "image1.png");
    assert_eq!(ImageArtifact::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_image_artifact() {

    let db = TestDb::new();
    let conn = db.conn();
    let id = ImageArtifact::new("image1.png").insert(&conn).unwrap().to_string();

    let mut image = ImageArtifact::view(&conn, &id).unwrap();
    image.image_path = "image2.png".to_string();
    image.update(&conn, &id).unwrap();

    assert_eq!(ImageArtifact::view(&conn, &id).unwrap().image_path, "image2.png");

}

#[test]
fn test_delete_image_artifact() {

    let db = TestDb::new();
    let conn = db.conn();
    let id = ImageArtifact::new("image1.png").insert(&conn).unwrap().to_string();
    ImageArtifact::new("image2.png").insert(&conn).unwrap();

    let mut image = ImageArtifact::view(&conn, &id).unwrap();
    image.delete(&conn, &id).unwrap();

    assert_eq!(ImageArtifact::view(&conn, &id).is_err(), true);
    assert_eq!(ImageArtifact::retrieve(&conn).unwrap().len(), 1);

}

#[test]
fn test_search_image_artifact() {

    let db = TestDb::new();
    let conn = db.conn();

    for path in ["cat.png", "cat.webp", "dog.png"] {
        ImageArtifact::new(path).insert(&conn).unwrap();
    }

    assert_eq!(ImageArtifact::search(&conn, "cat").unwrap().len(), 2);
    assert_eq!(ImageArtifact::search(&conn, "dog").unwrap()[0].image_path, "dog.png");
    assert_eq!(ImageArtifact::search(&conn, "Missing").unwrap().len(), 0);

}

#[test]
fn test_upload_image() {

    let src = tempfile::tempdir().unwrap();
    let dest = tempfile::tempdir().unwrap();
    fs::write(src.path().join("photo.png"), b"image bytes").unwrap();

    let name = upload_image(
        "photo.png",
        src.path().to_str().unwrap(),
        dest.path().to_str().unwrap(),
    ).unwrap();

    // copied under a new uuid name, keeping the extension
    assert_eq!(name.ends_with(".png"), true);
    assert_ne!(name, "photo.png");
    assert_eq!(fs::read(dest.path().join(&name)).unwrap(), b"image bytes");
    assert_eq!(src.path().join("photo.png").is_file(), true);

}

#[test]
fn test_upload_image_unique_names() {

    let src = tempfile::tempdir().unwrap();
    let dest = tempfile::tempdir().unwrap();
    fs::write(src.path().join("photo.png"), b"image bytes").unwrap();

    let src_str = src.path().to_str().unwrap();
    let dest_str = dest.path().to_str().unwrap();
    let name1 = upload_image("photo.png", src_str, dest_str).unwrap();
    let name2 = upload_image("photo.png", src_str, dest_str).unwrap();
    assert_ne!(name1, name2);

}

#[test]
fn test_upload_image_empty_name_uses_default() {

    let dest = tempfile::tempdir().unwrap();
    let name = upload_image("", "/does/not/matter", dest.path().to_str().unwrap()).unwrap();
    assert_eq!(name, "default.webp");

}

#[test]
fn test_upload_image_missing_extension() {

    let dest = tempfile::tempdir().unwrap();
    let result = upload_image("photo", "/does/not/matter", dest.path().to_str().unwrap());
    assert_eq!(matches!(result, Err(UploadImageError::MissingExtension(_))), true);

}

#[test]
fn test_upload_image_missing_source() {

    let src = tempfile::tempdir().unwrap();
    let dest = tempfile::tempdir().unwrap();
    let result = upload_image(
        "missing.png",
        src.path().to_str().unwrap(),
        dest.path().to_str().unwrap(),
    );
    assert_eq!(matches!(result, Err(UploadImageError::Copy { .. })), true);

}

#[test]
fn test_remove_image() {

    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("photo.png"), b"image bytes").unwrap();

    let name = remove_image("photo.png", dir.path().to_str().unwrap()).unwrap();
    assert_eq!(name, "photo.png");
    assert_eq!(dir.path().join("photo.png").exists(), false);

}

#[test]
fn test_remove_missing_image() {

    let dir = tempfile::tempdir().unwrap();
    let result = remove_image("missing.png", dir.path().to_str().unwrap());
    assert_eq!(matches!(result, Err(RemoveImageError::Delete { .. })), true);

}

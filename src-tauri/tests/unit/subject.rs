use crate::common::TestDb; 
use flashi_lib::subject::*;

#[test]
fn test_create_subject() {

    let db = TestDb::new(); 
    let conn = db.conn();
    let mut subject = Subject::new("Test subject", "Test subject description");
    subject.insert(&conn).unwrap();

    let subjects = Subject::retrieve(&conn).unwrap();
    assert_eq!(subjects.len(), 1); 
    assert_eq!(subjects[0].name, "Test subject");
    assert_eq!(subjects[0].description, "Test subject description");

}

#[test]
fn test_update_subject() {

    let db = TestDb::new(); 
    let conn = db.conn();
    let mut subject = Subject::new("Test subject update", "Test update subject description");
    subject.insert(&conn).unwrap();

    let subjects = Subject::retrieve(&conn).unwrap();
    assert_eq!(subjects[0].id, 1);

    let mut subject = subjects[0].clone();
    subject.name = "New Subject Name".to_string(); 
    subject.description = "New Subject Description".to_string(); 
    subject.update(&conn, "1");

    assert_eq!(subject.name, "New Subject Name");
    assert_eq!(subject.description, "New Subject Description");

}

#[test]
fn test_delete_subject() {

    let db = TestDb::new(); 
    let conn = db.conn();
    let mut subject = Subject::new("Test delete subject", "Test delete subject description");
    subject.insert(&conn).unwrap();

    let mut subjects = Subject::retrieve(&conn).unwrap();
    assert_eq!(subjects.len(), 1);

    let mut subject = subjects[0].clone();
    subject.delete(&conn, "1");

    subjects = Subject::retrieve(&conn).unwrap();
    assert_eq!(subjects.len(), 0);

}

#[test]
fn test_search_subject() {

    let db = TestDb::new(); 
    let conn = db.conn();

    let mut subject1 = Subject::new(
        "Subject 1", 
        "Subject 1 description"
    ).insert(&conn).unwrap();

    let mut subject2 = Subject::new(
        "Subject 2", 
        "Subject 2 description"
    ).insert(&conn).unwrap();

    let mut subject3 = Subject::new(
        "Subject 3", 
        "Subject 3 description"
    ).insert(&conn).unwrap();

    let mut results = Subject::search(&conn, "Subject").unwrap();
    assert_eq!(results.len(), 3);

    results = Subject::search(&conn, "Subject 1").unwrap();
    assert_eq!(results.len(), 1);

}

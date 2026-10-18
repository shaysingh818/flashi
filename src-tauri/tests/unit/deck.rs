use duckdb::Connection;
use crate::common::TestDb;
use flashi_lib::subject::*;
use flashi_lib::deck::*;

fn insert_subject(conn: &Connection, name: &str) -> Subject {

    let mut subject = Subject::new(name, "Test subject description");
    subject.insert(conn).unwrap();

    Subject::retrieve(conn).unwrap()
        .into_iter()
        .find(|s| s.name == name)
        .unwrap()

}

fn tags(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_create_deck() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject = insert_subject(&conn, "Test subject");

    let mut deck = Deck::new(subject.id, "Deck 1", tags(&["tag1", "tag2"]), "Deck Description");
    deck.insert(&conn).unwrap();

    let decks = Deck::retrieve(&conn).unwrap();
    assert_eq!(decks.len(), 1);
    assert_eq!(decks[0].subject_id, subject.id);
    assert_eq!(decks[0].name, "Deck 1");
    assert_eq!(decks[0].description, "Deck Description");
    assert_eq!(decks[0].tags, vec!["tag1", "tag2"]);
    assert_eq!(decks[0].reviewed, 0);

}

#[test]
fn test_create_deck_tags_with_quotes() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject = insert_subject(&conn, "Test subject");

    let mut deck = Deck::new(subject.id, "Deck 1", tags(&["it's", "plain"]), "Deck Description");
    deck.insert(&conn).unwrap();

    let decks = Deck::retrieve(&conn).unwrap();
    assert_eq!(decks[0].tags, vec!["it's", "plain"]);

}

#[test]
fn test_deck_special_tags_survive_updates() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject = insert_subject(&conn, "Test subject");

    let expected = ["it's", "plain", "a, b", "back\\slash", "it's a\\b", "[x]"];
    let mut deck = Deck::new(subject.id, "Deck 1", tags(&expected), "Deck Description");
    deck.insert(&conn).unwrap();

    let decks = Deck::retrieve(&conn).unwrap();
    assert_eq!(decks[0].tags, expected);

    let id = decks[0].id.to_string();
    let mut deck = decks[0].clone();
    deck.update(&conn, &id).unwrap();
    deck.update(&conn, &id).unwrap();
    assert_eq!(Deck::view(&conn, &id).unwrap().tags, expected);

}

#[test]
fn test_view_deck() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject = insert_subject(&conn, "Test subject");

    let mut deck = Deck::new(subject.id, "Deck view", tags(&["tag1"]), "Deck view description");
    deck.insert(&conn).unwrap();

    let decks = Deck::retrieve(&conn).unwrap();
    let id = decks[0].id.to_string();

    let viewed = Deck::view(&conn, &id).unwrap();
    assert_eq!(viewed.id, decks[0].id);
    assert_eq!(viewed.name, "Deck view");
    assert_eq!(viewed.description, "Deck view description");
    assert_eq!(viewed.tags, vec!["tag1"]);

    assert_eq!(Deck::view(&conn, "999").is_err(), true);

}

#[test]
fn test_update_deck() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject = insert_subject(&conn, "Test subject");

    let mut deck = Deck::new(subject.id, "Deck update", tags(&["tag1"]), "Deck update description");
    deck.insert(&conn).unwrap();

    let decks = Deck::retrieve(&conn).unwrap();
    let id = decks[0].id.to_string();

    let mut deck = decks[0].clone();
    deck.name = "New Deck Name".to_string();
    deck.description = "New Deck Description".to_string();
    deck.tags = tags(&["new1", "new2", "new3"]);
    deck.reviewed = 4;
    deck.update(&conn, &id).unwrap();

    let updated = Deck::view(&conn, &id).unwrap();
    assert_eq!(updated.name, "New Deck Name");
    assert_eq!(updated.description, "New Deck Description");
    assert_eq!(updated.tags, vec!["new1", "new2", "new3"]);
    assert_eq!(updated.reviewed, 4);
    assert_eq!(updated.subject_id, subject.id);

}

#[test]
fn test_delete_deck() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject = insert_subject(&conn, "Test subject");

    let mut deck = Deck::new(subject.id, "Deck delete", tags(&[]), "Deck delete description");
    deck.insert(&conn).unwrap();

    let mut decks = Deck::retrieve(&conn).unwrap();
    assert_eq!(decks.len(), 1);

    let id = decks[0].id.to_string();
    let mut deck = decks[0].clone();
    deck.delete(&conn, &id).unwrap();

    decks = Deck::retrieve(&conn).unwrap();
    assert_eq!(decks.len(), 0);

}

#[test]
fn test_search_deck() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject = insert_subject(&conn, "Test subject");

    for name in ["Deck 1", "Deck 2", "Deck 3"] {
        Deck::new(subject.id, name, tags(&[]), "Deck description")
            .insert(&conn)
            .unwrap();
    }

    let mut results = Deck::search(&conn, "Deck").unwrap();
    assert_eq!(results.len(), 3);

    results = Deck::search(&conn, "Deck 1").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Deck 1");

    results = Deck::search(&conn, "Missing").unwrap();
    assert_eq!(results.len(), 0);

}

#[test]
fn test_retrieve_decks_by_subject() {

    let db = TestDb::new();
    let conn = db.conn();
    let subject1 = insert_subject(&conn, "Subject 1");
    let subject2 = insert_subject(&conn, "Subject 2");

    Deck::new(subject1.id, "Deck 1", tags(&[]), "Deck description").insert(&conn).unwrap();
    Deck::new(subject1.id, "Deck 2", tags(&[]), "Deck description").insert(&conn).unwrap();
    Deck::new(subject2.id, "Deck 3", tags(&[]), "Deck description").insert(&conn).unwrap();

    let decks = Deck::retrieve_by_subject(&conn, &subject1.id.to_string()).unwrap();
    assert_eq!(decks.len(), 2);
    assert_eq!(decks.iter().all(|d| d.subject_id == subject1.id), true);

    let decks = Deck::retrieve_by_subject(&conn, &subject2.id.to_string()).unwrap();
    assert_eq!(decks.len(), 1);
    assert_eq!(decks[0].name, "Deck 3");

}

#[test]
fn test_parse_tags() {

    assert_eq!(Deck::parse_tags("['tag1', 'tag2']".to_string()), vec!["tag1", "tag2"]);
    assert_eq!(Deck::parse_tags("[tag1, tag2]".to_string()), vec!["tag1", "tag2"]);
    assert_eq!(Deck::parse_tags("[]".to_string()).len(), 0);

}

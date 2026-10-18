use duckdb::Connection;
use std::path::PathBuf;
use tempfile::TempDir;
use flashi_lib::subject::Subject;
use flashi_lib::deck::Deck;
use flashi_lib::question::Question;
use flashi_lib::session::Session;


pub struct TestDb {
    _dir: TempDir,          // dropped => directory deleted
    pub path: PathBuf,
}

impl TestDb {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("flashi.db");
        let template = concat!(env!("CARGO_MANIFEST_DIR"), "/database/build/flashi.db");
        std::fs::copy(template, &path).unwrap();
        TestDb { _dir: dir, path }
    }

    pub fn path_str(&self) -> &str { self.path.to_str().unwrap() }

    pub fn conn(&self) -> Connection { Connection::open(&self.path).unwrap() }
}

// ANSWER_TYPE is seeded with 1 = Open Ended, 2 = Multiple Choice, 3 = Ordered Sequence
pub const OPEN_ENDED: usize = 1;
pub const MULTIPLE_CHOICE: usize = 2;

pub fn seed_subject(conn: &Connection, name: &str) -> usize {
    Subject::new(name, "Seed subject").insert(conn).unwrap();
    Subject::retrieve(conn).unwrap()
        .into_iter()
        .find(|s| s.name == name)
        .unwrap()
        .id
}

pub fn seed_deck(conn: &Connection, subject_id: usize, name: &str) -> usize {
    Deck::new(subject_id, name, vec![], "Seed deck").insert(conn).unwrap();
    Deck::retrieve(conn).unwrap()
        .into_iter()
        .find(|d| d.name == name)
        .unwrap()
        .id
}

pub fn seed_question(conn: &Connection, deck_id: usize, answer_type_id: usize, raw: &str) -> usize {
    Question::new(deck_id, answer_type_id, 1, raw).insert(conn).unwrap() as usize
}

pub fn seed_session(conn: &Connection, name: &str) -> Session {
    Session::new(name, "Seed session").insert(conn).unwrap();
    Session::retrieve(conn).unwrap()
        .into_iter()
        .find(|s| s.name == name)
        .unwrap()
}

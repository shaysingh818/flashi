use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use uuid::Uuid;

use duckdb::{Connection, Result}; 

use crate::flashi_api::deck::*; 


#[tauri::command]
pub fn create_deck(subject_id: &str, name: &str, tags: Vec<String>, description: &str, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => return format!("Failed to open database: {}", e),
    };

    let subject_id_usize: usize = subject_id.parse::<usize>().unwrap();

    let mut deck: Deck = Deck::new(subject_id_usize, name, tags, description);

    match deck.insert(&conn) {
        Ok(()) => {
            let _ = conn.close();
            return "Success".to_string();
        }
        Err(e) => format!("Failed to insert deck: {}", e),
    }
}

#[tauri::command]
pub fn edit_deck(deck_id: &str, name: &str, description: &str, tags: Vec<String>, reviewed: usize, db_path: &str) -> String {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return e.to_string();
        }
    };

    match Deck::view(&conn, deck_id) {
        Ok(mut deck) => {
          
            deck.name = name.to_string();            
            deck.description = description.to_string();
            deck.reviewed = reviewed;
            deck.tags = tags;
            
            match deck.update(&conn, deck_id) {
                Ok(()) => {
                    let _ = conn.close();
                    return "Success".to_string();
                }
                Err(e) => format!("Failed to update deck: {}", e),
            }
        }
        Err(e) => {
            let msg = format!("Error updating deck: {:?}", e);
            return msg.to_string();
        }
    }
}

#[tauri::command]
pub fn view_decks(db_path: &str) -> Vec<Deck> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Deck::retrieve(&conn) {
        Ok(decks) => decks,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_decks_by_subject(db_path: &str, subject_id: &str) -> Vec<Deck> {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Deck::retrieve_by_subject(&conn, subject_id) {
        Ok(decks) => decks,
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn view_deck(db_path: &str, deck_id: &str) -> Result<Deck, String> {

    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            let err_msg = format!("Database connection error: {:?}", e);
            return Err(err_msg.to_string());
        }
    };

    match Deck::view(&conn, deck_id) {
        Ok(deck) => {
            return Ok(deck);
        }
        Err(e) => {
            let err_msg = format!("Error retrieving deck with id: {:?} Cause: {:?}", deck_id, e);
            return Err(err_msg.to_string());
        }
    }
}

#[tauri::command]
pub fn search_decks(db_path: &str, search_term: &str) -> Vec<Deck> {

  let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return Vec::new();
        }
    };

    match Deck::search(&conn, search_term) {
        Ok(decks) => decks,
        Err(e) => {
            println!("Error searching deck: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
pub fn delete_deck(db_path: &str, deck_id: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Deck::view(&conn, deck_id) {
        Ok(mut deck) => {
            let _ = deck.delete(&conn, deck_id);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving deck with id: {:?} Cause: {:?}", deck_id, e);
        }
    }
}

#[tauri::command]
pub fn export_deck(db_path: &str, deck_id: &str, path: &str) -> String {
  
    let conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Deck::view(&conn, deck_id) {
        Ok(mut deck) => {
            let _ = deck.export(&conn, deck_id, path);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error retrieving deck with id: {:?} Cause: {:?}", deck_id, e);
        }
    }
}

#[tauri::command]
pub fn import_deck(db_path: &str, deck_id: &str, subject_id: &str, path: &str) -> String {
  
    let mut conn = match Connection::open(db_path) {
        Ok(connection) => connection,
        Err(e) => {
            println!("{:?}", e);
            return format!("Database connection error: {:?}", e);
        }
    };

    match Deck::view(&conn, deck_id) {
        Ok(mut deck) => {
            let _ = deck.import(&mut conn, deck_id, subject_id, path);
            return format!("Success");
        }
        Err(e) => {
            return format!("Error importing deck with subject id: {:?} Cause: {:?}", subject_id, e);
        }
    }
}

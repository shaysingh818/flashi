pub mod flashi_api; 
pub mod handlers;

use crate::handlers::subject::{
    create_subject, 
    delete_subject, 
    edit_subject, 
    search_subjects,
    view_subject, 
    view_subjects,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![

          /* Subjects  */
          create_subject, 
          delete_subject, 
          edit_subject, 
          search_subjects,
          view_subject, 
          view_subjects

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

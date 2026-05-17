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

use crate::handlers::deck::{
    create_deck, 
    delete_deck, 
    edit_deck, 
    search_decks,
    view_deck, 
    view_decks,
};

use crate::handlers::question::{
    create_question, 
    delete_question, 
    edit_question, 
    search_questions,
    view_question, 
    view_questions,
};

use crate::handlers::answer::answer_type::{
    create_answer_type, 
    delete_answer_type, 
    edit_answer_type, 
    search_answer_types,
    view_answer_type, 
    view_answer_types,
};

use crate::handlers::answer::multiple_choice::{
    create_multiple_choice_answer, 
    delete_multiple_choice_answer, 
    edit_multiple_choice_answer, 
    search_multiple_choice_answers,
    view_multiple_choice_answer, 
    view_multiple_choice_answers,
};

use crate::handlers::answer::open_ended::{
    create_open_ended_answer, 
    delete_open_ended_answer, 
    edit_open_ended_answer, 
    search_open_ended_answers,
    view_open_ended_answer, 
    view_open_ended_answers,
};

use crate::handlers::answer::ordered_sequence::{
    create_ordered_sequence_answer, 
    delete_ordered_sequence_answer, 
    edit_ordered_sequence_answer, 
    search_ordered_sequence_answers,
    view_ordered_sequence_answer, 
    view_ordered_sequence_answers,
};

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
          view_subjects,

          /* Decks  */
          create_deck, 
          delete_deck, 
          edit_deck, 
          search_decks,
          view_deck, 
          view_decks,

         /* Questions  */ 
          create_question, 
          delete_question, 
          edit_question, 
          search_questions,
          view_question, 
          view_questions,

          /* Answer Type */
          create_answer_type, 
          delete_answer_type, 
          edit_answer_type, 
          search_answer_types,
          view_answer_type, 
          view_answer_types,

          /* Multiple choice  */ 
          create_multiple_choice_answer, 
          delete_multiple_choice_answer, 
          edit_multiple_choice_answer, 
          search_multiple_choice_answers,
          view_multiple_choice_answer, 
          view_multiple_choice_answers,

          /* Open Ended  */
          create_open_ended_answer, 
          delete_open_ended_answer, 
          edit_open_ended_answer, 
          search_open_ended_answers,
          view_open_ended_answer, 
          view_open_ended_answers,

          /* Ordered Sequence */
          create_ordered_sequence_answer, 
          delete_ordered_sequence_answer, 
          edit_ordered_sequence_answer, 
          search_ordered_sequence_answers,
          view_ordered_sequence_answer, 
          view_ordered_sequence_answers,
          

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

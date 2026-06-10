pub mod flashi_api; 
pub mod handlers;


use crate::handlers::session::{
    create_session, 
    delete_session, 
    edit_session, 
    search_sessions,
    view_session, 
    view_sessions,
    add_deck_to_session, 
    remove_deck_from_session,
    view_session_decks,
};

use crate::handlers::session_review::{
    create_session_review, 
    edit_session_review, 
    view_session_reviews,
    view_session_review_questions,
};

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
    view_decks_by_subject,
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
    create_multiple_choice,
    create_multiple_choice_answer, 
    delete_multiple_choice,
    delete_multiple_choice_answer, 
    edit_multiple_choice_answer,
    view_multiple_choice,
    view_multiple_choice_answer, 
    view_multiple_choice_answers,
    validate_multiple_choice_answer,
};

use crate::handlers::answer::open_ended::{
    create_open_ended_answer, 
    delete_open_ended_answer, 
    edit_open_ended_answer, 
    search_open_ended_answers,
    view_open_ended_answer, 
    view_open_ended_answers,
    validate_open_ended_answer,
};

use crate::handlers::answer::ordered_sequence::{
    create_ordered_sequence_answer, 
    delete_ordered_sequence_answer, 
    edit_ordered_sequence_answer, 
    search_ordered_sequence_answers,
    view_ordered_sequence_answer, 
    view_ordered_sequence_answers,
    validate_ordered_sequence_answer,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
        
            /* Sessions */
            create_session, 
            delete_session, 
            edit_session, 
            search_sessions,
            view_session, 
            view_sessions,
            add_deck_to_session, 
            remove_deck_from_session,
            view_session_decks,

            /* Session Review */ 
            create_session_review, 
            edit_session_review, 
            view_session_reviews,
            view_session_review_questions,

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
            view_decks_by_subject,

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
            create_multiple_choice,
            create_multiple_choice_answer,
            delete_multiple_choice,
            delete_multiple_choice_answer, 
            edit_multiple_choice_answer,
            view_multiple_choice,
            view_multiple_choice_answer, 
            view_multiple_choice_answers,
            validate_multiple_choice_answer,

            /* Open Ended  */
            create_open_ended_answer, 
            delete_open_ended_answer, 
            edit_open_ended_answer, 
            search_open_ended_answers,
            view_open_ended_answer, 
            view_open_ended_answers,
            validate_open_ended_answer,

            /* Ordered Sequence */
            create_ordered_sequence_answer, 
            delete_ordered_sequence_answer, 
            edit_ordered_sequence_answer, 
            search_ordered_sequence_answers,
            view_ordered_sequence_answer, 
            view_ordered_sequence_answers,
            validate_ordered_sequence_answer,
          

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

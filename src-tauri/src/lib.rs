pub mod flashi_api;
pub mod handlers;

use crate::handlers::session::{
    add_deck_to_session, create_session, delete_session, edit_session, remove_deck_from_session,
    search_sessions, view_session, view_session_decks, view_sessions, session_progress_status,
    session_metrics, session_question_breakdown, session_correct_incorrect_questions,
    session_daily_accuracy_history
};

use crate::handlers::session_review::{
    check_session_review_active, check_session_review_complete, create_session_review,
    edit_session_review, edit_session_review_question, generate_session_review_metrics,
    view_active_session_review_questions, view_session_review, view_session_review_question,
    view_session_review_questions, view_session_reviews, retrieve_session_metrics_history
};

use crate::handlers::subject::{
    create_subject, delete_subject, edit_subject, search_subjects, view_subject, view_subjects, subject_metrics
};

use crate::handlers::deck::{
    create_deck, delete_deck, edit_deck, search_decks, view_deck, view_decks, view_decks_by_subject,
};

use crate::handlers::question::{
    create_question, delete_question, edit_question, search_questions, view_question,
    view_questions, create_image_artifact, edit_image_artifact, delete_image_artifact, view_question_image_artifacts,
    add_question_image_artifact, remove_question_image_artifact
};

use crate::handlers::answer::answer_type::{
    create_answer_type, delete_answer_type, edit_answer_type, search_answer_types,
    view_answer_type, view_answer_types,
};

use crate::handlers::answer::multiple_choice::{
    create_multiple_choice, create_multiple_choice_answer, delete_multiple_choice,
    delete_multiple_choice_answer, edit_multiple_choice_answer, validate_multiple_choice_answer,
    view_multiple_choice, view_multiple_choice_answer, view_multiple_choice_answers,
};

use crate::handlers::answer::open_ended::{
    create_open_ended_answer, delete_open_ended_answer, edit_open_ended_answer,
    search_open_ended_answers, validate_open_ended_answer, view_open_ended_answer,
    view_open_ended_answers,
};

use crate::handlers::answer::ordered_sequence::{
    create_ordered_sequence_answer, delete_ordered_sequence_answer, edit_ordered_sequence_answer,
    search_ordered_sequence_answers, validate_ordered_sequence_answer,
    view_ordered_sequence_answer, view_ordered_sequence_answers,
};

use crate::handlers::analytics:: {
    general_metrics, review_time_metrics, question_review_time_metrics, historical_metrics
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
            session_progress_status,
            session_metrics,
            session_question_breakdown,
            session_correct_incorrect_questions,
            session_daily_accuracy_history,
            /* Session Review */
            create_session_review,
            edit_session_review,
            view_session_review,
            view_session_reviews,
            check_session_review_active,
            check_session_review_complete,
            view_session_review_questions,
            view_active_session_review_questions,
            view_session_review_question,
            edit_session_review_question,
            generate_session_review_metrics,
            retrieve_session_metrics_history,
            /* Subjects  */
            create_subject,
            delete_subject,
            edit_subject,
            search_subjects,
            view_subject,
            view_subjects,
            subject_metrics,
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
            create_image_artifact,
            edit_image_artifact,
            delete_image_artifact,
            view_question_image_artifacts,
            add_question_image_artifact,
            remove_question_image_artifact,
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
            /* analytics  */
            general_metrics,
            historical_metrics,
            review_time_metrics,
            question_review_time_metrics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

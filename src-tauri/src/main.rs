#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod services;

use services::file_service::FileService;
use services::parser_service::ParserService;
use services::database_service::{DatabaseService, Word};
use services::translation_service::TranslationService;

use std::sync::Mutex;
use tauri::State;

/// State chứa DatabaseService, được chia sẻ giữa các command
struct AppState {
    db: Mutex<DatabaseService>,
    translator: Mutex<TranslationService>,
}

/// Command: Đọc file text
#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    FileService::read_text_file(&path)
}

/// Command: Tách văn bản thành câu
#[tauri::command]
fn split_into_sentences(text: String) -> Vec<String> {
    ParserService::split_into_sentences(&text)
}

/// Command: Tách văn bản thành từ
#[tauri::command]
fn split_into_words(text: String) -> Vec<String> {
    ParserService::split_into_words(&text)
}

/// Command: Dịch text từ tiếng Anh sang tiếng Việt
#[tauri::command]
async fn translate_text(text: String, _state: State<'_, AppState>) -> Result<String, String> {
    // Tạo translator mới cho mỗi request để tránh lock issues
    let translator = TranslationService::new();
    translator.translate_en_to_vi(&text).await
}

/// Command: Lưu từ vào database
#[tauri::command]
fn save_word(
    word: String,
    translation: String,
    sentence: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.save_word(&word, &translation, &sentence)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// Command: Lấy tất cả từ đã lưu
#[tauri::command]
fn get_all_words(state: State<'_, AppState>) -> Result<Vec<Word>, String> {
    let db = state.db.lock().unwrap();
    db.get_all_words().map_err(|e| e.to_string())
}

/// Command: Đánh dấu từ đã học
#[tauri::command]
fn mark_as_learned(word_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.mark_as_learned(word_id).map_err(|e| e.to_string())
}

/// Command: Xóa từ khỏi database
#[tauri::command]
fn delete_word(word_id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_word(word_id).map_err(|e| e.to_string())
}

/// Command: Lấy thống kê từ vựng
#[tauri::command]
fn get_vocabulary_stats(state: State<'_, AppState>) -> Result<(i64, i64), String> {
    let db = state.db.lock().unwrap();
    let total = db.count_total_words().map_err(|e| e.to_string())?;
    let learned = db.count_learned_words().map_err(|e| e.to_string())?;
    Ok((total, learned))
}

/// Command: Xử lý file - đọc file và tách thành từ và câu
#[tauri::command]
fn process_text_file(path: String) -> Result<(Vec<String>, Vec<String>), String> {
    // Đọc file
    let content = FileService::read_text_file(&path)?;

    // Tách thành câu
    let sentences = ParserService::split_into_sentences(&content);

    // Tách thành từ (loại bỏ từ ngắn < 3 ký tự)
    let all_words = ParserService::split_into_words(&content);
    let words = ParserService::filter_words_by_length(all_words, 3);

    Ok((sentences, words))
}

/// Command: Lưu nhiều từ cùng lúc sau khi dịch
#[tauri::command]
async fn save_words_from_file(
    words: Vec<String>,
    sentences: Vec<String>,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    // Tạo translator mới để tránh lock issues
    let translator = TranslationService::new();
    let mut words_to_save = Vec::new();

    for word in words {
        // Dịch từ
        match translator.translate_en_to_vi(&word).await {
            Ok(translation) => {
                // Tìm câu chứa từ này
                let sentence = ParserService::find_sentences_with_word(&word, &sentences)
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "No context".to_string());

                words_to_save.push((word, translation, sentence));
            }
            Err(_) => continue, // Skip nếu không dịch được
        }

        // Delay để tránh rate limit
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    let db = state.db.lock().unwrap();
    db.save_words_batch(words_to_save)
        .map_err(|e| e.to_string())
}

/// Command: Tìm từ trong database
#[tauri::command]
fn find_word(word: String, state: State<'_, AppState>) -> Result<Option<Word>, String> {
    let db = state.db.lock().unwrap();
    db.find_word_by_text(&word).map_err(|e| e.to_string())
}

/// Command: Làm sạch từ
#[tauri::command]
fn clean_word(word: String) -> String {
    ParserService::clean_word(&word)
}

fn main() {
    // Khởi tạo database
    let db_service = DatabaseService::new("vocabulary.db");
    db_service.init().expect("Failed to initialize database");

    // Khởi tạo translation service
    let translation_service = TranslationService::new();

    // Tạo app state
    let app_state = AppState {
        db: Mutex::new(db_service),
        translator: Mutex::new(translation_service),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            read_text_file,
            split_into_sentences,
            split_into_words,
            translate_text,
            save_word,
            get_all_words,
            mark_as_learned,
            delete_word,
            get_vocabulary_stats,
            process_text_file,
            save_words_from_file,
            find_word,
            clean_word,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

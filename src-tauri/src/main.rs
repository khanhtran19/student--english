#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result};

#[derive(Debug, Serialize, Deserialize)]
struct Word {
    id: Option<i64>,
    word: String,
    translation: String,
    sentence: String,
    learned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sentence {
    text: String,
    translation: String,
}

// Khởi tạo database
fn init_db() -> Result<Connection> {
    let conn = Connection::open("vocabulary.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS words (
            id INTEGER PRIMARY KEY,
            word TEXT NOT NULL UNIQUE,
            translation TEXT,
            sentence TEXT,
            learned INTEGER DEFAULT 0
        )",
        [],
    )?;
    
    Ok(conn)
}

// Đọc file text
#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| e.to_string())
}

// Tách văn bản thành câu
#[tauri::command]
fn split_into_sentences(text: String) -> Vec<String> {
    let re = Regex::new(r"[.!?]+\s+").unwrap();
    re.split(&text)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

// Tách câu thành từ
#[tauri::command]
fn split_into_words(text: String) -> Vec<String> {
    let re = Regex::new(r"\b[a-zA-Z]+\b").unwrap();
    re.find_iter(&text)
        .map(|m| m.as_str().to_lowercase())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}

// Dịch text bằng API (MyMemory - free)
#[tauri::command]
async fn translate_text(text: String) -> Result<String, String> {
    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair=en|vi",
        urlencoding::encode(&text)
    );
    
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let json: serde_json::Value = response.json()
        .await
        .map_err(|e| e.to_string())?;
    
    let translation = json["responseData"]["translatedText"]
        .as_str()
        .unwrap_or("Translation error")
        .to_string();
    
    Ok(translation)
}

// Lưu từ vào database
#[tauri::command]
fn save_word(word: String, translation: String, sentence: String) -> Result<(), String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT OR REPLACE INTO words (word, translation, sentence, learned) 
         VALUES (?1, ?2, ?3, 0)",
        &[&word, &translation, &sentence],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

// Lấy tất cả từ đã lưu
#[tauri::command]
fn get_all_words() -> Result<Vec<Word>, String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT id, word, translation, sentence, learned FROM words"
    ).map_err(|e| e.to_string())?;
    
    let words = stmt.query_map([], |row| {
        Ok(Word {
            id: row.get(0)?,
            word: row.get(1)?,
            translation: row.get(2)?,
            sentence: row.get(3)?,
            learned: row.get::<_, i32>(4)? == 1,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for word in words {
        result.push(word.map_err(|e| e.to_string())?);
    }
    
    Ok(result)
}

// Đánh dấu từ đã học
#[tauri::command]
fn mark_as_learned(word_id: i64) -> Result<(), String> {
    let conn = init_db().map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE words SET learned = 1 WHERE id = ?1",
        &[&word_id],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

fn main() {
    // Khởi tạo database khi app start
    init_db().expect("Failed to initialize database");
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_text_file,
            split_into_sentences,
            split_into_words,
            translate_text,
            save_word,
            get_all_words,
            mark_as_learned
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Word {
    pub id: Option<i64>,
    pub word: String,
    pub translation: String,
    pub sentence: String,
    pub learned: bool,
}

/// Service quản lý database SQLite
pub struct DatabaseService {
    db_path: String,
}

impl DatabaseService {
    /// Tạo instance mới của DatabaseService
    pub fn new(db_path: &str) -> Self {
        DatabaseService {
            db_path: db_path.to_string(),
        }
    }

    /// Khởi tạo connection đến database
    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    /// Khởi tạo database và tạo bảng nếu chưa tồn tại
    pub fn init(&self) -> Result<()> {
        let conn = self.get_connection()?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS words (
                id INTEGER PRIMARY KEY,
                word TEXT NOT NULL UNIQUE,
                translation TEXT,
                sentence TEXT,
                learned INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Tạo bảng common words (từ thông dụng)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS common (
                id INTEGER PRIMARY KEY,
                word TEXT NOT NULL UNIQUE,
                translation TEXT,
                sentence TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Tạo index để tìm kiếm nhanh hơn
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_word ON words(word)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_common_word ON common(word)",
            [],
        )?;

        Ok(())
    }

    /// Lưu một từ vào database
    pub fn save_word(&self, word: &str, translation: &str, sentence: &str) -> Result<i64> {
        let conn = self.get_connection()?;

        conn.execute(
            "INSERT OR REPLACE INTO words (word, translation, sentence, learned)
             VALUES (?1, ?2, ?3, 0)",
            &[word, translation, sentence],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Lưu nhiều từ cùng lúc (batch insert)
    pub fn save_words_batch(&self, words: Vec<(String, String, String)>) -> Result<usize> {
        let conn = self.get_connection()?;
        let mut inserted = 0;

        for (word, translation, sentence) in words {
            match conn.execute(
                "INSERT OR IGNORE INTO words (word, translation, sentence, learned)
                 VALUES (?1, ?2, ?3, 0)",
                &[&word, &translation, &sentence],
            ) {
                Ok(count) => inserted += count,
                Err(_) => continue,
            }
        }

        Ok(inserted)
    }

    /// Lấy tất cả các từ từ database
    pub fn get_all_words(&self) -> Result<Vec<Word>> {
        let conn = self.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, word, translation, sentence, learned FROM words ORDER BY created_at DESC"
        )?;

        let words = stmt.query_map([], |row| {
            Ok(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                translation: row.get(2)?,
                sentence: row.get(3)?,
                learned: row.get::<_, i32>(4)? == 1,
            })
        })?;

        let mut result = Vec::new();
        for word in words {
            result.push(word?);
        }

        Ok(result)
    }

    /// Lấy từ theo ID
    pub fn get_word_by_id(&self, id: i64) -> Result<Option<Word>> {
        let conn = self.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, word, translation, sentence, learned FROM words WHERE id = ?1"
        )?;

        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                translation: row.get(2)?,
                sentence: row.get(3)?,
                learned: row.get::<_, i32>(4)? == 1,
            }))
        } else {
            Ok(None)
        }
    }

    /// Tìm từ theo text
    pub fn find_word_by_text(&self, word_text: &str) -> Result<Option<Word>> {
        let conn = self.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, word, translation, sentence, learned FROM words WHERE word = ?1"
        )?;

        let mut rows = stmt.query([word_text])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                translation: row.get(2)?,
                sentence: row.get(3)?,
                learned: row.get::<_, i32>(4)? == 1,
            }))
        } else {
            Ok(None)
        }
    }

    /// Đánh dấu từ là đã học
    pub fn mark_as_learned(&self, word_id: i64) -> Result<()> {
        let conn = self.get_connection()?;

        conn.execute(
            "UPDATE words SET learned = 1 WHERE id = ?1",
            &[&word_id],
        )?;

        Ok(())
    }

    /// Đánh dấu từ là chưa học
    pub fn mark_as_unlearned(&self, word_id: i64) -> Result<()> {
        let conn = self.get_connection()?;

        conn.execute(
            "UPDATE words SET learned = 0 WHERE id = ?1",
            &[&word_id],
        )?;

        Ok(())
    }

    /// Xóa từ khỏi database
    pub fn delete_word(&self, word_id: i64) -> Result<()> {
        let conn = self.get_connection()?;

        conn.execute(
            "DELETE FROM words WHERE id = ?1",
            &[&word_id],
        )?;

        Ok(())
    }

    /// Lấy số lượng từ đã học
    pub fn count_learned_words(&self) -> Result<i64> {
        let conn = self.get_connection()?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM words WHERE learned = 1",
            [],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Lấy tổng số từ
    pub fn count_total_words(&self) -> Result<i64> {
        let conn = self.get_connection()?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM words",
            [],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// Xóa tất cả dữ liệu
    pub fn clear_all(&self) -> Result<()> {
        let conn = self.get_connection()?;
        conn.execute("DELETE FROM words", [])?;
        Ok(())
    }

    /// Lưu từ vào bảng common
    pub fn save_common_word(&self, word: &str, translation: &str, sentence: &str) -> Result<i64> {
        let conn = self.get_connection()?;

        conn.execute(
            "INSERT OR REPLACE INTO common (word, translation, sentence)
             VALUES (?1, ?2, ?3)",
            &[word, translation, sentence],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Kiểm tra từ có tồn tại trong bảng words không
    pub fn word_exists_in_words(&self, word: &str) -> Result<bool> {
        let conn = self.get_connection()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM words WHERE word = ?1",
            &[word],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Kiểm tra từ có tồn tại trong bảng common không
    pub fn word_exists_in_common(&self, word: &str) -> Result<bool> {
        let conn = self.get_connection()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM common WHERE word = ?1",
            &[word],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Kiểm tra từ có tồn tại trong bất kỳ bảng nào không
    pub fn word_exists(&self, word: &str) -> Result<bool> {
        Ok(self.word_exists_in_words(word)? || self.word_exists_in_common(word)?)
    }

    /// Lấy tất cả từ từ bảng common
    pub fn get_all_common_words(&self) -> Result<Vec<Word>> {
        let conn = self.get_connection()?;

        let mut stmt = conn.prepare(
            "SELECT id, word, translation, sentence, 0 as learned FROM common ORDER BY created_at DESC"
        )?;

        let words = stmt.query_map([], |row| {
            Ok(Word {
                id: row.get(0)?,
                word: row.get(1)?,
                translation: row.get(2)?,
                sentence: row.get(3)?,
                learned: false,
            })
        })?;

        let mut result = Vec::new();
        for word in words {
            result.push(word?);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_operations() {
        let db = DatabaseService::new(":memory:");
        db.init().unwrap();

        // Test save word
        let id = db.save_word("hello", "xin chào", "Hello world!").unwrap();
        assert!(id > 0);

        // Test get all words
        let words = db.get_all_words().unwrap();
        assert_eq!(words.len(), 1);

        // Test mark as learned
        db.mark_as_learned(id).unwrap();
        let word = db.get_word_by_id(id).unwrap().unwrap();
        assert!(word.learned);
    }
}

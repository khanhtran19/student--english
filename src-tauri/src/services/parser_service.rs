use regex::Regex;
use std::collections::HashSet;

/// Service xử lý việc phân tích và tách text thành từ và câu
pub struct ParserService;

impl ParserService {
    /// Tách văn bản thành các câu
    ///
    /// # Arguments
    /// * `text` - Văn bản cần tách
    ///
    /// # Returns
    /// Vector chứa các câu đã được tách và làm sạch
    pub fn split_into_sentences(text: &str) -> Vec<String> {
        let re = Regex::new(r"[.!?]+\s+").unwrap();
        re.split(text)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Tách text thành các từ tiếng Anh duy nhất
    ///
    /// # Arguments
    /// * `text` - Văn bản cần tách
    ///
    /// # Returns
    /// Vector chứa các từ tiếng Anh (lowercase, không trùng lặp)
    pub fn split_into_words(text: &str) -> Vec<String> {
        let re = Regex::new(r"\b[a-zA-Z]+\b").unwrap();
        re.find_iter(text)
            .map(|m| m.as_str().to_lowercase())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Lọc các từ có độ dài tối thiểu
    ///
    /// # Arguments
    /// * `words` - Vector các từ cần lọc
    /// * `min_length` - Độ dài tối thiểu
    ///
    /// # Returns
    /// Vector các từ đã được lọc
    pub fn filter_words_by_length(words: Vec<String>, min_length: usize) -> Vec<String> {
        words.into_iter()
            .filter(|w| w.len() >= min_length)
            .collect()
    }

    /// Tìm câu chứa từ cụ thể
    ///
    /// # Arguments
    /// * `word` - Từ cần tìm
    /// * `sentences` - Vector các câu
    ///
    /// # Returns
    /// Vector các câu chứa từ đó
    pub fn find_sentences_with_word(word: &str, sentences: &[String]) -> Vec<String> {
        let word_pattern = format!(r"\b{}\b", regex::escape(word));
        let re = Regex::new(&word_pattern).unwrap();

        sentences.iter()
            .filter(|s| re.is_match(&s.to_lowercase()))
            .cloned()
            .collect()
    }

    /// Làm sạch từ (loại bỏ ký tự đặc biệt)
    ///
    /// # Arguments
    /// * `word` - Từ cần làm sạch
    ///
    /// # Returns
    /// Từ đã được làm sạch
    pub fn clean_word(word: &str) -> String {
        let re = Regex::new(r"[^a-zA-Z]").unwrap();
        re.replace_all(word, "").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_sentences() {
        let text = "Hello world. This is a test! How are you?";
        let sentences = ParserService::split_into_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "Hello world");
    }

    #[test]
    fn test_split_into_words() {
        let text = "Hello world, this is a test!";
        let words = ParserService::split_into_words(text);
        assert!(words.contains(&"hello".to_string()));
        assert!(words.contains(&"world".to_string()));
    }

    #[test]
    fn test_filter_words_by_length() {
        let words = vec!["a".to_string(), "hi".to_string(), "hello".to_string()];
        let filtered = ParserService::filter_words_by_length(words, 3);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0], "hello");
    }

    #[test]
    fn test_clean_word() {
        assert_eq!(ParserService::clean_word("hello!"), "hello");
        assert_eq!(ParserService::clean_word("wo'rld"), "world");
    }
}

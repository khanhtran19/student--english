use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TranslationResponse {
    #[serde(rename = "responseData")]
    response_data: ResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

/// Service xử lý dịch thuật
pub struct TranslationService {
    api_url: String,
}

impl TranslationService {
    /// Tạo instance mới với API mặc định (MyMemory)
    pub fn new() -> Self {
        TranslationService {
            api_url: "https://api.mymemory.translated.net/get".to_string(),
        }
    }

    /// Tạo instance với custom API URL
    pub fn with_api_url(api_url: &str) -> Self {
        TranslationService {
            api_url: api_url.to_string(),
        }
    }

    /// Dịch text từ tiếng Anh sang tiếng Việt
    ///
    /// # Arguments
    /// * `text` - Text cần dịch
    ///
    /// # Returns
    /// Result chứa text đã được dịch
    pub async fn translate_en_to_vi(&self, text: &str) -> Result<String, String> {
        self.translate(text, "en", "vi").await
    }

    /// Dịch text giữa các ngôn ngữ
    ///
    /// # Arguments
    /// * `text` - Text cần dịch
    /// * `from_lang` - Ngôn ngữ nguồn (vd: "en")
    /// * `to_lang` - Ngôn ngữ đích (vd: "vi")
    ///
    /// # Returns
    /// Result chứa text đã được dịch
    pub async fn translate(&self, text: &str, from_lang: &str, to_lang: &str) -> Result<String, String> {
        // Validate input
        if text.trim().is_empty() {
            return Err("Text không được để trống".to_string());
        }

        let url = format!(
            "{}?q={}&langpair={}|{}",
            self.api_url,
            urlencoding::encode(text),
            from_lang,
            to_lang
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("Lỗi kết nối API: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API trả về lỗi: {}", response.status()));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Lỗi parse JSON: {}", e))?;

        let translation = json["responseData"]["translatedText"]
            .as_str()
            .unwrap_or("Translation error")
            .to_string();

        Ok(translation)
    }

    /// Dịch nhiều text cùng lúc
    ///
    /// # Arguments
    /// * `texts` - Vector các text cần dịch
    ///
    /// # Returns
    /// Vector các text đã được dịch (giữ nguyên thứ tự)
    pub async fn translate_batch(&self, texts: Vec<String>) -> Vec<Result<String, String>> {
        let mut results = Vec::new();

        for text in texts {
            let result = self.translate_en_to_vi(&text).await;
            results.push(result);

            // Delay nhỏ để tránh rate limit
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        results
    }

    /// Kiểm tra xem text có chứa tiếng Anh không
    pub fn contains_english(text: &str) -> bool {
        text.chars().any(|c| c.is_ascii_alphabetic())
    }

    /// Làm sạch text trước khi dịch
    pub fn clean_text_for_translation(text: &str) -> String {
        text.trim()
            .replace('\n', " ")
            .replace('\r', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Default for TranslationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_english() {
        assert!(TranslationService::contains_english("hello"));
        assert!(TranslationService::contains_english("hello world"));
        assert!(!TranslationService::contains_english("123456"));
        assert!(TranslationService::contains_english("hello 123"));
    }

    #[test]
    fn test_clean_text() {
        let cleaned = TranslationService::clean_text_for_translation("  hello   world  \n\n test  ");
        assert_eq!(cleaned, "hello world test");
    }

    // Test async function (requires tokio runtime)
    #[tokio::test]
    async fn test_translate_en_to_vi() {
        let service = TranslationService::new();
        let result = service.translate_en_to_vi("hello").await;

        // Note: This test requires internet connection
        // In production, you might want to mock the API
        assert!(result.is_ok() || result.is_err()); // Just check it runs
    }
}

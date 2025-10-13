use std::fs;

/// Service xử lý các thao tác với file
pub struct FileService;

impl FileService {
    /// Đọc nội dung file text
    pub fn read_text_file(path: &str) -> Result<String, String> {
        fs::read_to_string(path)
            .map_err(|e| format!("Lỗi đọc file: {}", e))
    }

    /// Kiểm tra file có phải là file text hợp lệ không
    pub fn is_valid_text_file(path: &str) -> bool {
        path.ends_with(".txt") || path.ends_with(".md")
    }

    /// Lấy tên file từ đường dẫn
    pub fn get_file_name(path: &str) -> String {
        path.split('/').last()
            .or_else(|| path.split('\\').last())
            .unwrap_or(path)
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_text_file() {
        assert!(FileService::is_valid_text_file("test.txt"));
        assert!(FileService::is_valid_text_file("test.md"));
        assert!(!FileService::is_valid_text_file("test.pdf"));
    }

    #[test]
    fn test_get_file_name() {
        assert_eq!(FileService::get_file_name("/path/to/file.txt"), "file.txt");
        assert_eq!(FileService::get_file_name("C:\\path\\to\\file.txt"), "file.txt");
    }
}

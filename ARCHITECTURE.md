# Kiến trúc dự án Student English

## Tổng quan

Dự án đã được tái cấu trúc để tách biệt các chức năng thành các service độc lập, giúp code dễ bảo trì và mở rộng hơn.

## Cấu trúc thư mục

```
src-tauri/src/
├── main.rs                          # Entry point, định nghĩa các Tauri commands
└── services/
    ├── mod.rs                       # Module declaration
    ├── file_service.rs              # Xử lý file operations
    ├── parser_service.rs            # Tách text thành từ và câu
    ├── database_service.rs          # Quản lý SQLite database
    └── translation_service.rs       # Xử lý dịch thuật
```

## Các Service

### 1. FileService ([file_service.rs](src-tauri/src/services/file_service.rs))

Xử lý các thao tác với file.

**Chức năng:**
- `read_text_file(path)` - Đọc nội dung file text
- `is_valid_text_file(path)` - Kiểm tra file có hợp lệ không
- `get_file_name(path)` - Lấy tên file từ đường dẫn

**Sử dụng:**
```rust
let content = FileService::read_text_file("/path/to/file.txt")?;
```

### 2. ParserService ([parser_service.rs](src-tauri/src/services/parser_service.rs))

Xử lý việc phân tích và tách text thành từ và câu.

**Chức năng:**
- `split_into_sentences(text)` - Tách text thành các câu
- `split_into_words(text)` - Tách text thành các từ tiếng Anh duy nhất
- `filter_words_by_length(words, min_length)` - Lọc từ theo độ dài
- `find_sentences_with_word(word, sentences)` - Tìm câu chứa từ cụ thể
- `clean_word(word)` - Làm sạch từ (loại bỏ ký tự đặc biệt)

**Sử dụng:**
```rust
let sentences = ParserService::split_into_sentences(&text);
let words = ParserService::split_into_words(&text);
let filtered = ParserService::filter_words_by_length(words, 3);
```

### 3. DatabaseService ([database_service.rs](src-tauri/src/services/database_service.rs))

Quản lý SQLite database cho việc lưu trữ từ vựng.

**Chức năng:**
- `new(db_path)` - Tạo instance mới
- `init()` - Khởi tạo database và tạo bảng
- `save_word(word, translation, sentence)` - Lưu một từ
- `save_words_batch(words)` - Lưu nhiều từ cùng lúc
- `get_all_words()` - Lấy tất cả từ đã lưu
- `find_word_by_text(word)` - Tìm từ trong database
- `mark_as_learned(word_id)` - Đánh dấu từ đã học
- `delete_word(word_id)` - Xóa từ
- `count_learned_words()` - Đếm số từ đã học
- `count_total_words()` - Đếm tổng số từ

**Sử dụng:**
```rust
let db = DatabaseService::new("vocabulary.db");
db.init()?;
db.save_word("hello", "xin chào", "Hello world!")?;
let words = db.get_all_words()?;
```

### 4. TranslationService ([translation_service.rs](src-tauri/src/services/translation_service.rs))

Xử lý dịch thuật sử dụng MyMemory API.

**Chức năng:**
- `new()` - Tạo instance mới
- `translate_en_to_vi(text)` - Dịch từ tiếng Anh sang tiếng Việt
- `translate(text, from_lang, to_lang)` - Dịch giữa các ngôn ngữ
- `translate_batch(texts)` - Dịch nhiều text cùng lúc

**Sử dụng:**
```rust
let translator = TranslationService::new();
let translation = translator.translate_en_to_vi("hello").await?;
```

## Tauri Commands

Các command được expose cho frontend:

### File Operations
- `read_text_file(path)` - Đọc file text
- `process_text_file(path)` - Đọc và xử lý file (tách câu + từ)

### Text Processing
- `split_into_sentences(text)` - Tách text thành câu
- `split_into_words(text)` - Tách text thành từ
- `clean_word(word)` - Làm sạch từ

### Translation
- `translate_text(text)` - Dịch text

### Database
- `save_word(word, translation, sentence)` - Lưu một từ
- `save_words_from_file(words, sentences)` - Lưu nhiều từ từ file
- `get_all_words()` - Lấy tất cả từ
- `find_word(word)` - Tìm từ
- `mark_as_learned(word_id)` - Đánh dấu đã học
- `delete_word(word_id)` - Xóa từ
- `get_vocabulary_stats()` - Lấy thống kê (tổng số từ, số từ đã học)

## Workflow

### 1. Tải file lên và xử lý

```typescript
// Frontend (React)
import { invoke } from '@tauri-apps/api/tauri';

// Bước 1: Đọc và xử lý file
const [sentences, words] = await invoke('process_text_file', {
  path: filePath
});

// sentences: Array<string> - Danh sách các câu
// words: Array<string> - Danh sách các từ (đã lọc, độ dài >= 3)
```

### 2. Lưu từ vào database

**Cách 1: Lưu từng từ một (khi click vào từ)**
```typescript
await invoke('translate_text', { text: word });
await invoke('save_word', {
  word,
  translation,
  sentence
});
```

**Cách 2: Lưu hàng loạt từ file**
```typescript
// Tự động dịch và lưu tất cả từ từ file
const savedCount = await invoke('save_words_from_file', {
  words: words,      // Danh sách từ cần lưu
  sentences: sentences  // Danh sách câu để tìm context
});
```

### 3. Quản lý từ vựng

```typescript
// Lấy tất cả từ đã lưu
const words = await invoke('get_all_words');

// Đánh dấu đã học
await invoke('mark_as_learned', { wordId: id });

// Xóa từ
await invoke('delete_word', { wordId: id });

// Lấy thống kê
const [total, learned] = await invoke('get_vocabulary_stats');
```

## Database Schema

```sql
CREATE TABLE words (
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL UNIQUE,
    translation TEXT,
    sentence TEXT,
    learned INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_word ON words(word);
```

## Dependencies

### Rust
- `tauri` - Framework chính
- `serde` - Serialization/Deserialization
- `rusqlite` - SQLite driver
- `regex` - Regular expressions
- `reqwest` - HTTP client cho API dịch
- `urlencoding` - URL encoding
- `tokio` - Async runtime

### Frontend
- React
- TypeScript
- Tauri APIs

## Best Practices

1. **Separation of Concerns**: Mỗi service chỉ xử lý một nhóm chức năng cụ thể
2. **Error Handling**: Tất cả các function đều trả về `Result` để xử lý lỗi
3. **Type Safety**: Sử dụng Rust's type system để đảm bảo an toàn
4. **Testability**: Mỗi service có thể test độc lập
5. **Documentation**: Tất cả public APIs đều có doc comments

## Testing

```bash
# Test tất cả
cargo test --manifest-path src-tauri/Cargo.toml

# Test một service cụ thể
cargo test --manifest-path src-tauri/Cargo.toml parser_service

# Build project
cargo build --manifest-path src-tauri/Cargo.toml
```

## Future Improvements

1. Thêm cache cho translation để giảm API calls
2. Support thêm nhiều loại file (PDF, DOCX)
3. Thêm export/import vocabulary
4. Thêm spaced repetition algorithm
5. Support offline translation

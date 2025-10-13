# Student English - Ứng dụng học tiếng Anh

Ứng dụng desktop giúp học tiếng Anh thông qua việc đọc văn bản, tra từ và lưu từ vựng.

## ✨ Tính năng

### 📖 Đọc văn bản
- Tải file text (`.txt`, `.md`)
- Hiển thị văn bản với từ có thể click
- Click vào từ để xem nghĩa tiếng Việt

### 📚 Quản lý từ vựng
- Lưu từ vào database SQLite
- Xem danh sách từ đã lưu
- Đánh dấu từ đã học
- Xóa từ
- Thống kê tiến độ học

### 🚀 Xử lý file tự động
- Tách file thành câu và từ
- Tự động dịch và lưu hàng loạt từ
- Tìm câu chứa từ để làm context

### 🌐 Dịch thuật
- Dịch từ tiếng Anh sang tiếng Việt
- Sử dụng MyMemory Translation API
- Tự động xử lý rate limiting

## 🏗️ Kiến trúc

### Backend (Rust)
- **FileService**: Xử lý file operations
- **ParserService**: Tách text thành từ và câu
- **DatabaseService**: Quản lý SQLite database
- **TranslationService**: Xử lý dịch thuật

### Frontend (React + TypeScript)
- React 19 với TypeScript
- Vite để build
- Tauri API để giao tiếp với backend

Xem chi tiết: [ARCHITECTURE.md](./ARCHITECTURE.md)

## 🚀 Cài đặt và chạy

### Yêu cầu hệ thống
- Node.js 18+ và npm/yarn
- Rust 1.70+
- Các dependencies của Tauri: [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Cài đặt dependencies

```bash
# Frontend
npm install
# hoặc
yarn install

# Backend (Rust) sẽ tự động download khi build
```

### Chạy ứng dụng

```bash
# Development mode
npm run tauri dev

# Build production
npm run tauri build
```

⚠️ **LƯU Ý:** Phải chạy `npm run tauri dev`, KHÔNG chạy `npm run dev` (sẽ lỗi)

Xem chi tiết: [RUNNING.md](./RUNNING.md)

## 📖 Hướng dẫn sử dụng

### 1. Đọc và lưu từ thủ công

1. Click "📁 Chọn file văn bản"
2. Chọn file `.txt` hoặc `.md`
3. Click vào từ bạn muốn học
4. Xem nghĩa tiếng Việt
5. Click "💾 Lưu từ này"

### 2. Tải file và lưu tự động

```typescript
// Ví dụ code - có thể tích hợp vào UI
const [sentences, words] = await invoke('process_text_file', { path });
const savedCount = await invoke('save_words_from_file', { words, sentences });
```

### 3. Quản lý từ vựng

1. Chuyển sang tab "Từ vựng"
2. Xem danh sách từ đã lưu
3. Đánh dấu từ đã học
4. Xem thống kê tiến độ

Xem thêm ví dụ: [USAGE_EXAMPLES.md](./USAGE_EXAMPLES.md)

## 🛠️ Phát triển

### Cấu trúc thư mục

```
student-english/
├── src/                          # Frontend React
│   ├── components/
│   │   ├── TextReader.tsx       # Component đọc văn bản
│   │   ├── VocabularyList.tsx   # Component quản lý từ vựng
│   │   └── ...
│   ├── types.ts                 # TypeScript types
│   └── App.tsx                  # Root component
├── src-tauri/                   # Backend Rust
│   └── src/
│       ├── main.rs              # Entry point, Tauri commands
│       └── services/            # Business logic services
│           ├── file_service.rs
│           ├── parser_service.rs
│           ├── database_service.rs
│           └── translation_service.rs
├── ARCHITECTURE.md              # Tài liệu kiến trúc
├── USAGE_EXAMPLES.md            # Ví dụ code
└── RUNNING.md                   # Hướng dẫn chạy app
```

### Test

```bash
# Test Rust
cargo test --manifest-path src-tauri/Cargo.toml

# Check Rust code
cargo check --manifest-path src-tauri/Cargo.toml

# Build TypeScript
npm run build
```

### Thêm chức năng mới

1. **Thêm Rust command:**
   - Thêm function vào service tương ứng
   - Thêm `#[tauri::command]` function vào `main.rs`
   - Đăng ký trong `tauri::generate_handler![]`

2. **Gọi từ Frontend:**
   ```typescript
   import { invoke } from '@tauri-apps/api/core';
   const result = await invoke('your_command', { params });
   ```

## 🗄️ Database

SQLite database: `vocabulary.db`

### Schema

```sql
CREATE TABLE words (
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL UNIQUE,
    translation TEXT,
    sentence TEXT,
    learned INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## 🔧 API Reference

### Tauri Commands

#### File Operations
- `read_text_file(path: string)` → `string`
- `process_text_file(path: string)` → `[string[], string[]]`

#### Text Processing
- `split_into_sentences(text: string)` → `string[]`
- `split_into_words(text: string)` → `string[]`
- `clean_word(word: string)` → `string`

#### Translation
- `translate_text(text: string)` → `string`

#### Database
- `save_word(word, translation, sentence)` → `void`
- `save_words_from_file(words, sentences)` → `number`
- `get_all_words()` → `Word[]`
- `find_word(word: string)` → `Word | null`
- `mark_as_learned(wordId: number)` → `void`
- `delete_word(wordId: number)` → `void`
- `get_vocabulary_stats()` → `[number, number]`

## 🐛 Xử lý lỗi thường gặp

### Dialog.open not allowed

**Nguyên nhân:** Plugin dialog chưa được cấu hình đúng permissions

**Giải pháp:** Đã được sửa - rebuild app với `npm run tauri dev`

### TypeError: Cannot read properties of undefined (reading 'invoke')

**Nguyên nhân:** Chạy trong browser thay vì Tauri app

**Giải pháp:** Chạy `npm run tauri dev` thay vì `npm run dev`

### Translation API rate limit

**Nguyên nhân:** Gọi API quá nhiều

**Giải pháp:** Service đã tự động thêm delay. Đợi vài phút rồi thử lại.

### Database permission error

**Nguyên nhân:** Không có quyền ghi file

**Giải pháp:** Kiểm tra quyền thư mục, chạy app với quyền phù hợp

**Xem thêm:** [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) - Hướng dẫn chi tiết xử lý tất cả các lỗi

## 📚 Tài liệu

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Kiến trúc chi tiết
- [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) - Xử lý lỗi chi tiết
- [USAGE_EXAMPLES.md](./USAGE_EXAMPLES.md) - Ví dụ code
- [RUNNING.md](./RUNNING.md) - Hướng dẫn chạy app

## 🚧 Roadmap

- [ ] Thêm cache cho translation
- [ ] Support PDF, DOCX files
- [ ] Export/Import vocabulary
- [ ] Spaced repetition algorithm
- [ ] Offline translation
- [ ] Dark mode
- [ ] Multiple language support

## 📄 License

MIT License

## 🤝 Contributing

Pull requests are welcome! Vui lòng đọc [ARCHITECTURE.md](./ARCHITECTURE.md) trước khi contribute.

## 💡 Tips

1. Luôn chạy `npm run tauri dev` để test
2. Xem console output để debug
3. Database file `vocabulary.db` ở thư mục gốc của app
4. API dịch có rate limit - không dịch quá nhiều từ cùng lúc
5. Đọc [USAGE_EXAMPLES.md](./USAGE_EXAMPLES.md) để xem cách sử dụng các chức năng

# Hướng dẫn chạy ứng dụng

## ⚠️ LƯU Ý QUAN TRỌNG

**KHÔNG** chạy `npm run dev` hoặc `yarn dev` - điều này sẽ chạy app trong browser và gây ra lỗi:
```
TypeError: Cannot read properties of undefined (reading 'invoke')
```

## ✅ Cách chạy đúng

### Development Mode

```bash
npm run tauri dev
# hoặc
yarn tauri dev
```

Lệnh này sẽ:
1. Build frontend (React + Vite)
2. Compile Rust backend
3. Mở Tauri desktop application

### Production Build

```bash
npm run tauri build
# hoặc
yarn tauri build
```

File thực thi sẽ được tạo trong:
- Linux: `src-tauri/target/release/student-english`
- Windows: `src-tauri/target/release/student-english.exe`
- macOS: `src-tauri/target/release/bundle/macos/`

## 🐛 Xử lý lỗi

### Lỗi: "Cannot read properties of undefined (reading 'invoke')"

**Nguyên nhân:** Bạn đang chạy app trong browser thay vì Tauri desktop app.

**Giải pháp:**
1. Tắt browser
2. Chạy `npm run tauri dev` thay vì `npm run dev`

### Lỗi: "Failed to initialize database"

**Nguyên nhân:** SQLite database không thể được tạo.

**Giải pháp:**
1. Kiểm tra quyền ghi file trong thư mục hiện tại
2. File `vocabulary.db` sẽ được tạo tự động ở thư mục gốc của app

### Lỗi: "Translation error" hoặc "Rate limit"

**Nguyên nhân:** API dịch MyMemory có giới hạn số request.

**Giải pháp:**
1. Đợi vài phút rồi thử lại
2. Không dịch quá nhiều từ cùng lúc
3. Service đã tự động thêm delay 200ms giữa các request

## 📝 Development Workflow

### Làm việc với Frontend

```bash
# Chạy dev mode
npm run tauri dev

# Khi sửa code TypeScript/React, Vite sẽ tự động hot-reload
```

### Làm việc với Backend (Rust)

```bash
# Kiểm tra code Rust
cargo check --manifest-path src-tauri/Cargo.toml

# Build Rust
cargo build --manifest-path src-tauri/Cargo.toml

# Chạy tests
cargo test --manifest-path src-tauri/Cargo.toml

# Sau khi sửa code Rust, restart app để áp dụng thay đổi
# (Không có hot-reload cho Rust code)
```

### Cài đặt dependencies mới

#### Frontend
```bash
npm install <package-name>
```

#### Backend (Rust)
Thêm vào `src-tauri/Cargo.toml`:
```toml
[dependencies]
package-name = "version"
```

Sau đó chạy:
```bash
cargo build --manifest-path src-tauri/Cargo.toml
```

## 🗂️ File database

Database SQLite được lưu tại: `vocabulary.db` (trong thư mục app)

Để xem nội dung database:
```bash
sqlite3 vocabulary.db

# SQL commands
.tables              # Liệt kê các bảng
SELECT * FROM words; # Xem tất cả từ
.exit                # Thoát
```

## 🧪 Testing

### Test Rust code
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### Test TypeScript build
```bash
npm run build
```

## 📦 Distribution

Sau khi build production (`npm run tauri build`), file installer sẽ ở:
- **Linux**:
  - AppImage: `src-tauri/target/release/bundle/appimage/`
  - deb: `src-tauri/target/release/bundle/deb/`
- **Windows**:
  - MSI: `src-tauri/target/release/bundle/msi/`
  - NSIS: `src-tauri/target/release/bundle/nsis/`
- **macOS**:
  - DMG: `src-tauri/target/release/bundle/dmg/`
  - App: `src-tauri/target/release/bundle/macos/`

## 🔧 Troubleshooting

### App không mở

1. Kiểm tra log:
```bash
npm run tauri dev
# Xem console output
```

2. Rebuild from scratch:
```bash
# Xóa cache
rm -rf node_modules dist src-tauri/target

# Cài lại
npm install

# Build lại
npm run tauri dev
```

### Lỗi compilation Rust

```bash
# Xóa build artifacts
cargo clean --manifest-path src-tauri/Cargo.toml

# Build lại
cargo build --manifest-path src-tauri/Cargo.toml
```

## 📚 Tài liệu tham khảo

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Vite Documentation](https://vitejs.dev/)
- [React Documentation](https://react.dev/)
- [Architecture Guide](./ARCHITECTURE.md)
- [Usage Examples](./USAGE_EXAMPLES.md)

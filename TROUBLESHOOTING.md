# Troubleshooting - Xử lý các lỗi thường gặp

## 🚫 Lỗi Dialog

### Lỗi: "dialog.open not allowed" hoặc "Dialog permission denied"

**Biểu hiện:**
```
Error: dialog.open not allowed
```

**Nguyên nhân:**
Tauri plugin dialog chưa được cấu hình đúng permissions.

**Giải pháp:** (Đã được sửa trong commit này)

1. **Thêm plugin dialog vào Cargo.toml:**
```toml
[dependencies]
tauri-plugin-dialog = "2"
```

2. **Đăng ký plugin trong main.rs:**
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    // ... other configs
```

3. **Thêm permission vào capabilities/default.json:**
```json
{
  "permissions": [
    "core:default",
    "opener:default",
    "dialog:default"
  ]
}
```

4. **Rebuild app:**
```bash
cargo build --manifest-path src-tauri/Cargo.toml
npm run tauri dev
```

---

## 🌐 Lỗi Invoke

### Lỗi: "Cannot read properties of undefined (reading 'invoke')"

**Biểu hiện:**
```javascript
TypeError: Cannot read properties of undefined (reading 'invoke')
```

**Nguyên nhân:**
Đang chạy app trong browser thay vì Tauri desktop app.

**Giải pháp:**

❌ **SAI:**
```bash
npm run dev        # Chạy trong browser
yarn dev           # Chạy trong browser
```

✅ **ĐÚNG:**
```bash
npm run tauri dev  # Chạy Tauri desktop app
yarn tauri dev     # Chạy Tauri desktop app
```

**Kiểm tra:**
- Nếu thấy app mở trong Chrome/Firefox/Safari → SAI
- Nếu thấy app mở như một ứng dụng desktop độc lập → ĐÚNG

---

## 🗄️ Lỗi Database

### Lỗi: "Failed to initialize database" hoặc "Permission denied"

**Nguyên nhân:**
- Không có quyền ghi file trong thư mục hiện tại
- Thư mục không tồn tại

**Giải pháp:**

1. **Kiểm tra quyền:**
```bash
ls -la vocabulary.db
```

2. **Xóa database cũ và tạo lại:**
```bash
rm vocabulary.db
npm run tauri dev
```

3. **Chạy với quyền phù hợp:**
```bash
# Linux/Mac
chmod 644 vocabulary.db

# Hoặc chạy app với quyền user bình thường (không dùng sudo)
```

### Lỗi: "Database is locked"

**Nguyên nhân:**
Có nhiều process đang truy cập database cùng lúc.

**Giải pháp:**
1. Đóng tất cả các instance của app
2. Xóa file lock (nếu có):
```bash
rm vocabulary.db-shm vocabulary.db-wal
```
3. Chạy lại app

---

## 🌍 Lỗi Translation API

### Lỗi: "Translation error" hoặc "Rate limit exceeded"

**Nguyên nhân:**
MyMemory Translation API có giới hạn:
- 1000 requests/ngày (free tier)
- Rate limiting nếu request quá nhanh

**Giải pháp:**

1. **Đợi một lúc rồi thử lại** (service đã tự động thêm 200ms delay giữa mỗi request)

2. **Kiểm tra kết nối internet:**
```bash
ping api.mymemory.translated.net
```

3. **Giảm số từ cần dịch:**
   - Không dịch quá nhiều từ cùng lúc
   - Sử dụng chức năng lưu từng từ thay vì batch

4. **Xem log để debug:**
```bash
npm run tauri dev
# Xem console output
```

### Lỗi: "Failed to fetch" hoặc "Network error"

**Nguyên nhân:**
Không có kết nối internet hoặc API bị chặn.

**Giải pháp:**
1. Kiểm tra kết nối internet
2. Kiểm tra firewall/proxy
3. Thử API khác (cần sửa code trong TranslationService)

---

## 🔧 Lỗi Build

### Lỗi: "cargo: command not found"

**Nguyên nhân:**
Rust chưa được cài đặt.

**Giải pháp:**
```bash
# Cài đặt Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env

# Kiểm tra
cargo --version
```

### Lỗi: "failed to compile ... missing dependencies"

**Nguyên nhân:**
Thiếu system dependencies cho Tauri.

**Giải pháp:**

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

**Linux (Arch):**
```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
  appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips
```

**macOS:**
```bash
# Cài Xcode Command Line Tools
xcode-select --install
```

**Windows:**
```bash
# Cài Visual Studio C++ Build Tools
# Download từ: https://visualstudio.microsoft.com/downloads/
```

### Lỗi: "thread ... panicked" trong Rust code

**Giải pháp:**
1. Xem full error message trong console
2. Check lại code Rust có unwrap() không an toàn
3. Rebuild from scratch:
```bash
cargo clean --manifest-path src-tauri/Cargo.toml
cargo build --manifest-path src-tauri/Cargo.toml
```

---

## ⚛️ Lỗi Frontend

### Lỗi: TypeScript compilation errors

**Giải pháp:**
```bash
# Xóa cache và rebuild
rm -rf node_modules dist
npm install
npm run build
```

### Lỗi: "Module not found"

**Giải pháp:**
```bash
npm install
# hoặc
yarn install
```

---

## 🔄 Lỗi Hot Reload

### App không tự động reload khi sửa code

**Nguyên nhân:**
- Rust code không có hot reload (phải restart)
- Frontend code nên có hot reload

**Giải pháp:**

**Khi sửa Frontend (React/TypeScript):**
- Không cần làm gì, Vite sẽ tự động reload

**Khi sửa Backend (Rust):**
- Phải restart app:
  - Ctrl+C để tắt
  - `npm run tauri dev` để chạy lại

---

## 📝 Lỗi File Path

### Lỗi: "File not found" khi đọc file

**Nguyên nhân:**
- Đường dẫn sai
- File không có quyền đọc

**Giải pháp:**

1. **Dùng absolute path:**
```typescript
// ✅ ĐÚNG - Sử dụng dialog để chọn file
const path = await open({...});

// ❌ SAI - Hardcode path
const path = "/home/user/file.txt";
```

2. **Kiểm tra quyền file:**
```bash
ls -la /path/to/file.txt
chmod 644 /path/to/file.txt
```

---

## 🧪 Debug Tips

### Xem Console Output

**Frontend logs:**
- Mở Developer Tools trong app: `Ctrl+Shift+I` (Linux/Windows) hoặc `Cmd+Option+I` (Mac)
- Hoặc xem terminal output khi chạy `npm run tauri dev`

**Backend logs:**
```rust
// Thêm vào code Rust
println!("Debug: {:?}", variable);
eprintln!("Error: {:?}", error);
```

### Test từng phần

**Test Rust functions:**
```bash
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture
```

**Test Rust commands manually:**
Sửa main.rs tạm thời để test:
```rust
fn main() {
    // Test code here
    let result = FileService::read_text_file("test.txt");
    println!("{:?}", result);
}
```

**Test Frontend:**
```typescript
// Thêm console.log
console.log('Before invoke');
const result = await invoke('command');
console.log('Result:', result);
```

---

## 📞 Cần Trợ Giúp?

Nếu vẫn gặp lỗi:

1. **Check documentation:**
   - [ARCHITECTURE.md](./ARCHITECTURE.md)
   - [USAGE_EXAMPLES.md](./USAGE_EXAMPLES.md)
   - [RUNNING.md](./RUNNING.md)

2. **Xem Tauri docs:**
   - https://tauri.app/v1/guides/

3. **Search lỗi:**
   - Google: "tauri [error message]"
   - GitHub Issues: https://github.com/tauri-apps/tauri/issues

4. **Rebuild from scratch:**
```bash
# Clean everything
rm -rf node_modules dist src-tauri/target vocabulary.db

# Reinstall
npm install

# Rebuild
npm run tauri dev
```

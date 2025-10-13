# Ví dụ sử dụng các chức năng

## 1. Upload và xử lý file

### Cách cũ (đọc từng bước)
```typescript
// Đọc file
const content = await invoke('read_text_file', { path: filePath });

// Tách câu
const sentences = await invoke('split_into_sentences', { text: content });

// Tách từ
const words = await invoke('split_into_words', { text: content });
```

### Cách mới (một lần xử lý)
```typescript
// Đọc và xử lý file một lần
const [sentences, words] = await invoke<[string[], string[]]>('process_text_file', {
  path: filePath
});

console.log('Số câu:', sentences.length);
console.log('Số từ:', words.length);
```

## 2. Tự động lưu tất cả từ từ file vào database

### Component example: FileUploader.tsx

```typescript
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/plugin-dialog';

function FileUploader() {
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState('');

  const handleUploadAndSave = async () => {
    try {
      setLoading(true);

      // Bước 1: Chọn file
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Text Files',
          extensions: ['txt', 'md']
        }]
      });

      if (!selected || typeof selected !== 'string') return;

      setProgress('Đang đọc file...');

      // Bước 2: Đọc và tách file thành từ + câu
      const [sentences, words] = await invoke<[string[], string[]]>(
        'process_text_file',
        { path: selected }
      );

      setProgress(`Tìm thấy ${words.length} từ. Đang dịch và lưu...`);

      // Bước 3: Tự động dịch và lưu tất cả từ
      const savedCount = await invoke<number>(
        'save_words_from_file',
        {
          words: words,
          sentences: sentences
        }
      );

      setProgress(`Đã lưu ${savedCount} từ vào database!`);
      alert(`✅ Hoàn tất! Đã lưu ${savedCount} từ mới.`);

    } catch (error) {
      console.error(error);
      alert('Lỗi: ' + error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <button
        onClick={handleUploadAndSave}
        disabled={loading}
      >
        {loading ? 'Đang xử lý...' : 'Tải file và lưu từ vựng'}
      </button>
      {progress && <p>{progress}</p>}
    </div>
  );
}
```

## 3. Hiển thị danh sách từ vựng với thống kê

### Component example: VocabularyStats.tsx

```typescript
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import type { Word } from '../types';

function VocabularyStats() {
  const [words, setWords] = useState<Word[]>([]);
  const [stats, setStats] = useState({ total: 0, learned: 0 });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      // Lấy danh sách từ
      const allWords = await invoke<Word[]>('get_all_words');
      setWords(allWords);

      // Lấy thống kê
      const [total, learned] = await invoke<[number, number]>('get_vocabulary_stats');
      setStats({ total, learned });
    } catch (error) {
      console.error('Lỗi:', error);
    }
  };

  const handleMarkLearned = async (wordId: number) => {
    try {
      await invoke('mark_as_learned', { wordId });
      loadData(); // Reload data
    } catch (error) {
      console.error('Lỗi:', error);
    }
  };

  const handleDelete = async (wordId: number) => {
    if (!confirm('Bạn có chắc muốn xóa từ này?')) return;

    try {
      await invoke('delete_word', { wordId });
      loadData(); // Reload data
    } catch (error) {
      console.error('Lỗi:', error);
    }
  };

  return (
    <div>
      <div className="stats">
        <h3>Thống kê</h3>
        <p>Tổng số từ: {stats.total}</p>
        <p>Đã học: {stats.learned}</p>
        <p>Chưa học: {stats.total - stats.learned}</p>
        <p>Tiến độ: {stats.total > 0 ? Math.round(stats.learned / stats.total * 100) : 0}%</p>
      </div>

      <div className="word-list">
        <h3>Danh sách từ vựng</h3>
        {words.map(word => (
          <div key={word.id} className={word.learned ? 'word learned' : 'word'}>
            <h4>{word.word}</h4>
            <p>{word.translation}</p>
            <p className="context">"{word.sentence}"</p>
            <div className="actions">
              {!word.learned && (
                <button onClick={() => handleMarkLearned(word.id!)}>
                  ✓ Đánh dấu đã học
                </button>
              )}
              <button onClick={() => handleDelete(word.id!)}>
                🗑 Xóa
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
```

## 4. Tìm kiếm từ trong database

### Component example: WordSearch.tsx

```typescript
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import type { Word } from '../types';

function WordSearch() {
  const [searchTerm, setSearchTerm] = useState('');
  const [result, setResult] = useState<Word | null>(null);
  const [notFound, setNotFound] = useState(false);

  const handleSearch = async () => {
    try {
      setNotFound(false);

      // Làm sạch từ trước khi search
      const cleanedWord = await invoke<string>('clean_word', { word: searchTerm });

      // Tìm trong database
      const found = await invoke<Word | null>('find_word', { word: cleanedWord.toLowerCase() });

      if (found) {
        setResult(found);
      } else {
        setNotFound(true);
        setResult(null);
      }
    } catch (error) {
      console.error('Lỗi:', error);
    }
  };

  return (
    <div>
      <h3>Tìm kiếm từ vựng</h3>
      <input
        type="text"
        value={searchTerm}
        onChange={(e) => setSearchTerm(e.target.value)}
        placeholder="Nhập từ cần tìm..."
        onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
      />
      <button onClick={handleSearch}>Tìm</button>

      {result && (
        <div className="search-result">
          <h4>{result.word}</h4>
          <p><strong>Nghĩa:</strong> {result.translation}</p>
          <p><strong>Ví dụ:</strong> "{result.sentence}"</p>
          <p><strong>Trạng thái:</strong> {result.learned ? '✅ Đã học' : '⏳ Chưa học'}</p>
        </div>
      )}

      {notFound && (
        <p className="not-found">Không tìm thấy từ "{searchTerm}" trong danh sách.</p>
      )}
    </div>
  );
}
```

## 5. Cập nhật component TextReader để tương thích

### TextReader.tsx (đã cập nhật)

```typescript
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/plugin-dialog';
import type { Word } from '../types';

function TextReader() {
  const [fileName, setFileName] = useState('');
  const [textContent, setTextContent] = useState('');

  // Upload file và hiển thị
  const handleUpload = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Text Files', extensions: ['txt', 'md'] }]
      });

      if (selected && typeof selected === 'string') {
        const name = selected.split('/').pop() || selected.split('\\').pop() || '';
        setFileName(name);

        const content = await invoke<string>('read_text_file', { path: selected });
        setTextContent(content);
      }
    } catch (error) {
      alert('Lỗi: ' + error);
    }
  };

  // Click vào từ để dịch và lưu
  const handleWordClick = async (word: string, sentence: string) => {
    try {
      // Làm sạch từ
      const cleanWord = await invoke<string>('clean_word', { word });

      // Kiểm tra từ đã tồn tại chưa
      const existing = await invoke<Word | null>('find_word', {
        word: cleanWord.toLowerCase()
      });

      if (existing) {
        alert(`Từ "${cleanWord}" đã có trong danh sách!\nNghĩa: ${existing.translation}`);
        return;
      }

      // Dịch từ
      const translation = await invoke<string>('translate_text', {
        text: cleanWord
      });

      // Lưu vào database
      await invoke('save_word', {
        word: cleanWord.toLowerCase(),
        translation,
        sentence
      });

      alert(`✅ Đã lưu từ "${cleanWord}"!\nNghĩa: ${translation}`);
    } catch (error) {
      alert('Lỗi: ' + error);
    }
  };

  // Render text với từ có thể click
  const renderText = () => {
    if (!textContent) return null;

    const sentences = textContent.split(/[.!?]+/).filter(s => s.trim());

    return sentences.map((sentence, idx) => {
      const words = sentence.split(/\s+/);

      return (
        <span key={idx}>
          {words.map((word, wordIdx) => {
            const cleanWord = word.replace(/[^a-zA-Z]/g, '');

            if (cleanWord.length > 2) {
              return (
                <span
                  key={wordIdx}
                  className="word-highlight"
                  onClick={() => handleWordClick(cleanWord, sentence.trim())}
                >
                  {word}{' '}
                </span>
              );
            }

            return <span key={wordIdx}>{word} </span>;
          })}
          .
        </span>
      );
    });
  };

  return (
    <div className="text-reader">
      <div className="upload-section">
        <button onClick={handleUpload}>📁 Chọn file</button>
        {fileName && <span>📄 {fileName}</span>}
      </div>

      <div className="text-display">
        {renderText()}
      </div>
    </div>
  );
}

export default TextReader;
```

## 6. API Reference

### Available Commands

```typescript
// File Operations
invoke<string>('read_text_file', { path: string })
invoke<[string[], string[]]>('process_text_file', { path: string })

// Text Processing
invoke<string[]>('split_into_sentences', { text: string })
invoke<string[]>('split_into_words', { text: string })
invoke<string>('clean_word', { word: string })

// Translation
invoke<string>('translate_text', { text: string })

// Database Operations
invoke<void>('save_word', { word: string, translation: string, sentence: string })
invoke<number>('save_words_from_file', { words: string[], sentences: string[] })
invoke<Word[]>('get_all_words')
invoke<Word | null>('find_word', { word: string })
invoke<void>('mark_as_learned', { wordId: number })
invoke<void>('delete_word', { wordId: number })
invoke<[number, number]>('get_vocabulary_stats')
```

## Tips

1. **Rate Limiting**: Khi dịch nhiều từ, API có thể rate limit. Command `save_words_from_file` đã tự động xử lý delay giữa các request.

2. **Error Handling**: Luôn wrap invoke calls trong try-catch để xử lý lỗi.

3. **Loading States**: Hiển thị loading indicator khi gọi async operations.

4. **Caching**: Frontend nên cache danh sách từ vựng để tránh gọi API nhiều lần.

5. **Optimization**: Sử dụng `process_text_file` thay vì gọi riêng lẻ `read_text_file`, `split_into_sentences`, `split_into_words`.

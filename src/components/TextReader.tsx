import { useState, useMemo, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import TranslationPopup from './TranslationPopup';
import WordPreviewList from './WordPreviewList';
import type { TranslationData, Word } from '../types';

interface ProgressData {
    current: number;
    total: number;
    percentage: number;
}

function TextReader() {
    const [fileName, setFileName] = useState<string>('');
    const [textContent, setTextContent] = useState<string>('');
    const [showPopup, setShowPopup] = useState<boolean>(false);
    const [translationData, setTranslationData] = useState<TranslationData | null>(null);
    const [previewWords, setPreviewWords] = useState<Word[]>([]);
    const [isProcessing, setIsProcessing] = useState<boolean>(false);
    const [progress, setProgress] = useState<ProgressData | null>(null);

    // Lắng nghe progress events từ backend
    useEffect(() => {
        const unlisten = listen<ProgressData>('translation-progress', (event) => {
            setProgress(event.payload);
        });

        return () => {
            unlisten.then(fn => fn());
        };
    }, []);

    // Upload file
    const handleUpload = async () => {
        try {
            const selected = await open({
                multiple: false,
                filters: [{
                    name: 'Text Files',
                    extensions: ['txt', 'md']
                }]
            });

            if (selected && typeof selected === 'string') {
                const name = selected.split('/').pop() || selected.split('\\').pop() || '';
                setFileName(name);

                const content = await invoke<string>('read_text_file', { path: selected });
                setTextContent(content);

                // Tự động parse file và lấy danh sách từ chưa lưu
                setIsProcessing(true);
                setPreviewWords([]);
                setProgress(null);

                try {
                    const words = await invoke<Word[]>('parse_file_and_get_unsaved_words', { path: selected });
                    setPreviewWords(words);

                    if (words.length === 0) {
                        alert('ℹ️ Tất cả các từ trong file đã được lưu trước đó!');
                    } else {
                        alert(`✅ Tìm thấy ${words.length} từ mới chưa được lưu!`);
                    }
                } catch (error) {
                    console.error('Parse error:', error);
                    alert('Lỗi khi phân tích file: ' + error);
                } finally {
                    setIsProcessing(false);
                    setProgress(null);
                }
            }
        } catch (error) {
            console.error('Upload error:', error);
            alert('Lỗi: ' + error);
        }
    };

    // Click vào từ
    const handleWordClick = useCallback(async (word: string, sentence: string) => {
        setShowPopup(true);
        setTranslationData({
            word,
            translation: 'Đang dịch...',
            sentence
        });

        try {
            const translation = await invoke<string>('translate_text', { text: word });
            setTranslationData({
                word,
                translation,
                sentence
            });
        } catch (error) {
            setTranslationData({
                word,
                translation: 'Lỗi dịch: ' + error,
                sentence
            });
        }
    }, []);

    // Lưu từ
    const handleSaveWord = useCallback(async () => {
        if (!translationData) return;

        try {
            await invoke('save_word', {
                word: translationData.word,
                translation: translationData.translation,
                sentence: translationData.sentence
            });
            alert('✅ Đã lưu từ vào danh sách!');
            setShowPopup(false);
        } catch (error) {
            alert('Lỗi: ' + error);
        }
    }, [translationData]);

    // Render text với highlight - Memoize để tránh re-render
    const renderedText = useMemo(() => {
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
                                    onClick={() => handleWordClick(cleanWord.toLowerCase(), sentence.trim())}
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
    }, [textContent, handleWordClick]);

    const handleWordsSaved = () => {
        // Callback khi từ được lưu thành công
        // Có thể refresh danh sách hoặc hiển thị thông báo
    };

    return (
        <div className="tab-content active">
            <div className="upload-section">
                <button onClick={handleUpload} className="btn-primary" disabled={isProcessing}>
                    {isProcessing ? '⏳ Đang xử lý...' : '📁 Chọn file văn bản'}
                </button>
                {fileName && <span id="file-name">📄 {fileName}</span>}
            </div>

            {/* Progress bar */}
            {isProcessing && progress && progress.total > 0 && (
                <div className="progress-container">
                    <div className="progress-info">
                        <span>Đang dịch từ: {progress.current} / {progress.total}</span>
                        <span>{progress.percentage}%</span>
                    </div>
                    <div className="progress-bar-wrapper">
                        <div
                            className="progress-bar-fill"
                            style={{ width: `${progress.percentage}%` }}
                        />
                    </div>
                </div>
            )}

            {/* Hiển thị danh sách từ preview */}
            {previewWords.length > 0 && (
                <WordPreviewList
                    words={previewWords}
                    onWordsSaved={handleWordsSaved}
                />
            )}

            <div className="text-display">
                <div id="text-content">
                    {renderedText}
                </div>
            </div>

            {showPopup && translationData && (
                <TranslationPopup
                    data={translationData}
                    onSave={handleSaveWord}
                    onClose={() => setShowPopup(false)}
                />
            )}
        </div>
    );
}

export default TextReader;
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import TranslationPopup from './TranslationPopup';
import type { TranslationData } from '../types';

function TextReader() {
    const [fileName, setFileName] = useState<string>('');
    const [textContent, setTextContent] = useState<string>('');
    const [showPopup, setShowPopup] = useState<boolean>(false);
    const [translationData, setTranslationData] = useState<TranslationData | null>(null);

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
            }
        } catch (error) {
            alert('Lỗi: ' + error);
        }
    };

    // Click vào từ
    const handleWordClick = async (word: string, sentence: string) => {
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
    };

    // Lưu từ
    const handleSaveWord = async () => {
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
    };

    // Render text với highlight
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
    };

    return (
        <div className="tab-content active">
            <div className="upload-section">
                <button onClick={handleUpload} className="btn-primary">
                    📁 Chọn file văn bản
                </button>
                {fileName && <span id="file-name">📄 {fileName}</span>}
            </div>

            <div className="text-display">
                <div id="text-content">
                    {renderText()}
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
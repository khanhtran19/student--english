import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Word } from '../types';

interface WordPreviewListProps {
    words: Word[];
    onWordsSaved: () => void;
}

function WordPreviewList({ words, onWordsSaved }: WordPreviewListProps) {
    const [wordList, setWordList] = useState<Word[]>(words);
    const [savingWords, setSavingWords] = useState<Set<string>>(new Set());
    const [isSavingAll, setIsSavingAll] = useState(false);
    const [saveProgress, setSaveProgress] = useState({ current: 0, total: 0 });

    const handleSaveToWords = async (word: Word, event?: React.MouseEvent) => {
        if (event) {
            event.preventDefault();
            event.stopPropagation();
        }

        setSavingWords(prev => new Set(prev).add(word.word));

        try {
            await invoke('save_word_to_words', {
                word: word.word,
                translation: word.translation,
                sentence: word.sentence
            });

            // Remove từ khỏi danh sách sau khi lưu thành công
            setWordList(prev => prev.filter(w => w.word !== word.word));

        } catch (error) {
            alert('Lỗi khi lưu vào Words: ' + error);
        } finally {
            setSavingWords(prev => {
                const newSet = new Set(prev);
                newSet.delete(word.word);
                return newSet;
            });
        }
    };

    const handleSaveToCommon = async (word: Word, event?: React.MouseEvent) => {
        if (event) {
            event.preventDefault();
            event.stopPropagation();
        }

        setSavingWords(prev => new Set(prev).add(word.word));

        try {
            await invoke('save_word_to_common', {
                word: word.word,
                translation: word.translation,
                sentence: word.sentence
            });

            // Remove từ khỏi danh sách sau khi lưu thành công
            setWordList(prev => prev.filter(w => w.word !== word.word));

        } catch (error) {
            alert('Lỗi khi lưu vào Common: ' + error);
        } finally {
            setSavingWords(prev => {
                const newSet = new Set(prev);
                newSet.delete(word.word);
                return newSet;
            });
        }
    };

    const handleSaveAll = async (table: 'words' | 'common', event?: React.MouseEvent) => {
        if (event) {
            event.preventDefault();
            event.stopPropagation();
        }

        const saveFunction = table === 'words' ? 'save_word_to_words' : 'save_word_to_common';
        const totalWords = wordList.length;

        setIsSavingAll(true);
        setSaveProgress({ current: 0, total: totalWords });

        let savedCount = 0;

        for (let i = 0; i < wordList.length; i++) {
            const word = wordList[i];
            try {
                await invoke(saveFunction, {
                    word: word.word,
                    translation: word.translation,
                    sentence: word.sentence
                });
                savedCount++;
                setSaveProgress({ current: savedCount, total: totalWords });
            } catch (error) {
                console.error(`Lỗi khi lưu từ ${word.word}:`, error);
            }
        }

        setWordList([]);
        setIsSavingAll(false);
        setSaveProgress({ current: 0, total: 0 });
        onWordsSaved();
        alert(`✅ Đã lưu ${savedCount}/${totalWords} từ vào ${table === 'words' ? 'Words' : 'Common'}!`);
    };

    if (wordList.length === 0) {
        return (
            <div style={{ padding: '20px', textAlign: 'center', color: '#666' }}>
                Không có từ mới nào để hiển thị (tất cả từ đã được lưu trước đó)
            </div>
        );
    }

    return (
        <div style={{ padding: '20px' }}>
            <div style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                marginBottom: '15px',
                paddingBottom: '10px',
                borderBottom: '2px solid #ddd'
            }}>
                <h3 style={{ margin: 0 }}>📋 Danh sách từ mới ({wordList.length} từ)</h3>
                <div style={{ display: 'flex', gap: '10px' }}>
                    <button
                        onClick={(e) => handleSaveAll('words', e)}
                        className="btn-primary"
                        style={{ fontSize: '14px', padding: '8px 16px' }}
                    >
                        💾 Lưu tất cả vào Words
                    </button>
                    <button
                        onClick={(e) => handleSaveAll('common', e)}
                        className="btn-primary"
                        style={{ fontSize: '14px', padding: '8px 16px' }}
                    >
                        ⭐ Lưu tất cả vào Common
                    </button>
                </div>
            </div>

            <div style={{
                maxHeight: '400px',
                overflowY: 'auto',
                border: '1px solid #ddd',
                borderRadius: '8px'
            }}>
                <table style={{
                    width: '100%',
                    borderCollapse: 'collapse',
                    backgroundColor: 'white'
                }}>
                    <thead style={{
                        position: 'sticky',
                        top: 0,
                        backgroundColor: '#f5f5f5',
                        borderBottom: '2px solid #ddd'
                    }}>
                        <tr>
                            <th style={{ padding: '12px', textAlign: 'left', width: '20%' }}>Từ tiếng Anh</th>
                            <th style={{ padding: '12px', textAlign: 'left', width: '25%' }}>Nghĩa tiếng Việt</th>
                            <th style={{ padding: '12px', textAlign: 'left', width: '35%' }}>Câu ví dụ</th>
                            <th style={{ padding: '12px', textAlign: 'center', width: '20%' }}>Hành động</th>
                        </tr>
                    </thead>
                    <tbody>
                        {wordList.map((word, index) => {
                            const isSaving = savingWords.has(word.word);

                            return (
                                <tr
                                    key={index}
                                    style={{
                                        borderBottom: '1px solid #eee',
                                        backgroundColor: index % 2 === 0 ? 'white' : '#f9f9f9'
                                    }}
                                >
                                    <td style={{ padding: '12px', fontWeight: 'bold', color: '#2c3e50' }}>
                                        {word.word}
                                    </td>
                                    <td style={{ padding: '12px', color: '#e74c3c' }}>
                                        {word.translation}
                                    </td>
                                    <td style={{
                                        padding: '12px',
                                        fontSize: '13px',
                                        color: '#555',
                                        fontStyle: 'italic'
                                    }}>
                                        {word.sentence}
                                    </td>
                                    <td style={{ padding: '12px', textAlign: 'center' }}>
                                        <div style={{ display: 'flex', gap: '5px', justifyContent: 'center' }}>
                                            <button
                                                onClick={() => handleSaveToWords(word)}
                                                disabled={isSaving}
                                                className="btn-primary"
                                                style={{
                                                    fontSize: '12px',
                                                    padding: '6px 12px',
                                                    opacity: isSaving ? 0.5 : 1
                                                }}
                                            >
                                                {isSaving ? '...' : '💾 Words'}
                                            </button>
                                            <button
                                                onClick={() => handleSaveToCommon(word)}
                                                disabled={isSaving}
                                                className="btn-primary"
                                                style={{
                                                    fontSize: '12px',
                                                    padding: '6px 12px',
                                                    backgroundColor: '#f39c12',
                                                    opacity: isSaving ? 0.5 : 1
                                                }}
                                            >
                                                {isSaving ? '...' : '⭐ Common'}
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>

            <div style={{
                marginTop: '10px',
                fontSize: '13px',
                color: '#666',
                fontStyle: 'italic'
            }}>
                💡 Mẹo: Nhấn vào "Words" để lưu từ cần học, "Common" để lưu từ thông dụng
            </div>
            {/* read unused vars to avoid TS unused variable errors */}
            <div style={{ display: 'none' }} aria-hidden>
                {isSavingAll ? 'saving' : null}
                {saveProgress.total}
            </div>
        </div>
    );
}

export default WordPreviewList;

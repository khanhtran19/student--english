import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

import type { Word } from '../types';

function VocabularyList() {
    const [words, setWords] = useState<Word[]>([]);

    const loadVocabulary = useCallback(async () => {
        try {
            const result = await invoke<Word[]>('get_all_words');
            setWords(result);
        } catch (error) {
            console.error('Load error:', error);
            alert('Lỗi tải từ vựng: ' + error);
        }
    }, []);

    const markLearned = useCallback(async (wordId: number) => {
        try {
            await invoke('mark_as_learned', { wordId });
            await loadVocabulary();
        } catch (error) {
            console.error('Mark learned error:', error);
            alert('Lỗi: ' + error);
        }
    }, [loadVocabulary]);

    useEffect(() => {
        loadVocabulary();
    }, [loadVocabulary]);

    return (
        <div className="tab-content active">
            <div className="vocab-controls">
                <button onClick={loadVocabulary} className="btn-primary">
                    🔄 Làm mới
                </button>
                <span id="vocab-count">📊 Tổng: {words.length} từ</span>
            </div>

            <div id="vocabulary-list">
                {words.map((word) => (
                    <div
                        key={word.id}
                        className={`vocab-card ${word.learned ? 'learned' : ''}`}
                    >
                        <h4>{word.word}</h4>
                        <div className="translation">{word.translation}</div>
                        <div className="sentence">"{word.sentence}"</div>
                        <button
                            onClick={() => word.id && markLearned(word.id)}
                            disabled={word.learned}
                        >
                            {word.learned ? '✅ Đã học' : '📚 Đánh dấu đã học'}
                        </button>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default VocabularyList;
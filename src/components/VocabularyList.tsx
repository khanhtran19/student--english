import { useState, useEffect, useCallback, memo } from 'react';
import { invoke } from '@tauri-apps/api/core';

import type { Word } from '../types';

// Tách WordCard ra thành component riêng để tối ưu re-render
interface WordCardProps {
    word: Word;
    onMarkLearned: (wordId: number) => void;
}

const WordCard = memo(({ word, onMarkLearned }: WordCardProps) => {
    const handleClick = useCallback(() => {
        if (word.id) {
            onMarkLearned(word.id);
        }
    }, [word.id, onMarkLearned]);

    return (
        <div className={`vocab-card ${word.learned ? 'learned' : ''}`}>
            <h4>{word.word}</h4>
            <div className="translation">{word.translation}</div>
            <div className="sentence">"{word.sentence}"</div>
            <button
                onClick={handleClick}
                disabled={word.learned}
            >
                {word.learned ? '✅ Đã học' : '📚 Đánh dấu đã học'}
            </button>
        </div>
    );
});

WordCard.displayName = 'WordCard';

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

    // Tối ưu: Cập nhật local state thay vì reload toàn bộ
    const markLearned = useCallback(async (wordId: number) => {
        try {
            // Cập nhật UI ngay lập tức (optimistic update)
            setWords(prevWords =>
                prevWords.map(word =>
                    word.id === wordId
                        ? { ...word, learned: true }
                        : word
                )
            );

            // Gọi API
            await invoke('mark_as_learned', { wordId });
        } catch (error) {
            console.error('Mark learned error:', error);
            alert('Lỗi: ' + error);
            // Nếu lỗi, reload lại để đồng bộ
            loadVocabulary();
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
                    <WordCard
                        key={word.id}
                        word={word}
                        onMarkLearned={markLearned}
                    />
                ))}
            </div>
        </div>
    );
}

export default VocabularyList;
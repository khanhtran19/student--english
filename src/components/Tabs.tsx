import { memo, useCallback } from 'react';

interface TabsProps {
    activeTab: 'reader' | 'vocabulary';
    onTabChange: (tab: 'reader' | 'vocabulary') => void;
}

function Tabs({ activeTab, onTabChange }: TabsProps) {
    // Memoize handlers để tránh tạo function mới mỗi lần render
    const handleReaderClick = useCallback(() => {
        onTabChange('reader');
    }, [onTabChange]);

    const handleVocabularyClick = useCallback(() => {
        onTabChange('vocabulary');
    }, [onTabChange]);

    return (
        <div className="tabs">
            <button
                className={`tab-btn ${activeTab === 'reader' ? 'active' : ''}`}
                onClick={handleReaderClick}
            >
                Đọc văn bản
            </button>
            <button
                className={`tab-btn ${activeTab === 'vocabulary' ? 'active' : ''}`}
                onClick={handleVocabularyClick}
            >
                Từ vựng đã học
            </button>
        </div>
    );
}

// Memo component để tránh re-render khi parent re-render
export default memo(Tabs);
interface TabsProps {
    activeTab: 'reader' | 'vocabulary';
    onTabChange: (tab: 'reader' | 'vocabulary') => void;
}

function Tabs({ activeTab, onTabChange }: TabsProps) {
    return (
        <div className="tabs">
            <button
                className={`tab-btn ${activeTab === 'reader' ? 'active' : ''}`}
                onClick={() => onTabChange('reader')}
            >
                Đọc văn bản
            </button>
            <button
                className={`tab-btn ${activeTab === 'vocabulary' ? 'active' : ''}`}
                onClick={() => onTabChange('vocabulary')}
            >
                Từ vựng đã học
            </button>
        </div>
    );
}

export default Tabs;
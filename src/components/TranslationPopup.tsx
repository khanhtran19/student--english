import { memo } from 'react';
import type { TranslationData } from '../types';

interface TranslationPopupProps {
    data: TranslationData;
    onSave: () => void;
    onClose: () => void;
}

function TranslationPopup({ data, onSave, onClose }: TranslationPopupProps) {
    return (
        <div className="popup">
            <div className="popup-content">
                <h3>{data.word}</h3>
                <p>{data.translation}</p>
                <p className="sentence-context">"{data.sentence}"</p>
                <button onClick={onSave} className="btn-primary">
                    💾 Lưu từ này
                </button>
                <button onClick={onClose} className="btn-secondary">
                    Đóng
                </button>
            </div>
        </div>
    );
}

// Memo component để tránh re-render không cần thiết
// Popup chỉ re-render khi data, onSave, hoặc onClose thay đổi
export default memo(TranslationPopup);
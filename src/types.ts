export interface Word {
    id?: number;
    word: string;
    translation: string;
    sentence: string;
    learned: boolean;
}

export interface TranslationData {
    word: string;
    translation: string;
    sentence: string;
}
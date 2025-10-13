import { useState } from 'react';
import Header from './components/Header';
import Tabs from './components/Tabs';
import TextReader from './components/TextReader';
import VocabularyList from './components/VocabularyList';
import './styles.css';

function App() {
  const [activeTab, setActiveTab] = useState<'reader' | 'vocabulary'>('reader');

  return (
    <div className="container">
      <Header />

      <Tabs activeTab={activeTab} onTabChange={setActiveTab} />

      {activeTab === 'reader' ? (
        <TextReader />
      ) : (
        <VocabularyList />
      )}
    </div>
  );
}

export default App;
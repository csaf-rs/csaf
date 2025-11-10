import { useState, useEffect } from 'react';
import Header from './components/Header';
import Controls from './components/Controls';
import DropZone from './components/DropZone';
import DocumentInfo from './components/DocumentInfo';
import LoadingSpinner from './components/LoadingSpinner';
import ValidationResults from './components/ValidationResults';
import Alert from './components/Alert';
import Footer from './components/Footer';
import { initWasm, validateDocument } from './utils/validation';
import type { ValidationResult, ValidationPreset } from './types';

function App() {
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const [currentDocument, setCurrentDocument] = useState<unknown | null>(null);
  const [currentFilename, setCurrentFilename] = useState<string | null>(null);
  const [fileSize, setFileSize] = useState<number | null>(null);
  const [preset, setPreset] = useState<ValidationPreset>('basic');
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<ValidationResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    initWasm()
      .then(() => {
        setWasmLoaded(true);
        console.log('✅ WASM module loaded successfully');
      })
      .catch((err: unknown) => {
        console.error('Failed to load WASM module:', err);
        setError('Failed to initialize WebAssembly module. Please refresh the page.');
      });
  }, []);

  const handleFileSelect = (file: File) => {
    if (!file) return;

    if (!file.name.endsWith('.json')) {
      setError('Please select a JSON file');
      return;
    }

    const reader = new FileReader();

    reader.onload = (e) => {
      try {
        const result = e.target?.result;
        if (typeof result === 'string') {
          const doc = JSON.parse(result);
          setCurrentDocument(doc);
          setCurrentFilename(file.name);
          setFileSize(file.size);
          setResults(null);
          setError(null);
        }
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Unknown error';
        setError('Invalid JSON file: ' + message);
      }
    };

    reader.onerror = () => {
      setError('Failed to read file');
    };

    reader.readAsText(file);
  };

  const handleValidate = async () => {
    if (!wasmLoaded) {
      setError('WASM module not loaded yet. Please wait...');
      return;
    }

    if (!currentDocument) {
      setError('No document loaded');
      return;
    }

    setLoading(true);
    setResults(null);
    setError(null);

    try {
      const result = await validateDocument(currentDocument, preset);
      setResults(result);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : (typeof err === 'string' ? err : 'Unknown error');
      setError('Validation error: ' + errorMsg);
      console.error('Validation error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleClear = () => {
    setCurrentDocument(null);
    setCurrentFilename(null);
    setFileSize(null);
    setResults(null);
    setError(null);
  };

  return (
    <div className="bg-gray-50 min-h-screen">
      <div className="container mx-auto px-4 py-8 max-w-6xl">
        <Header />

        <Controls
          preset={preset}
          onPresetChange={setPreset}
          onValidate={handleValidate}
          onClear={handleClear}
          validateDisabled={!currentDocument}
        />

        {!currentDocument && <DropZone onFileSelect={handleFileSelect} />}

        {currentFilename && (
          <DocumentInfo
            filename={currentFilename}
            size={fileSize}
          />
        )}

        {loading && <LoadingSpinner />}

        {results && <ValidationResults results={results} />}

        {error && (
          <Alert
            type="error"
            title="Error"
            message={error}
            onClose={() => setError(null)}
          />
        )}

        <Footer />
      </div>
    </div>
  );
}

export default App;

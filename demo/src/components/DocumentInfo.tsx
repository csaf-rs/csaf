interface DocumentInfoProps {
  filename: string;
  size: number;
}

function DocumentInfo({ filename, size }: DocumentInfoProps) {
  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  };

  return (
    <div className="bg-white rounded-lg shadow-sm p-6 mb-6">
      <h3 className="text-lg font-semibold text-gray-900 mb-4">Document Information</h3>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <span className="text-sm text-gray-600">Filename:</span>
          <p className="font-medium text-gray-900">{filename}</p>
        </div>
        <div>
          <span className="text-sm text-gray-600">Size:</span>
          <p className="font-medium text-gray-900">{formatBytes(size)}</p>
        </div>
      </div>
    </div>
  );
}

export default DocumentInfo;

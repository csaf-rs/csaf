function LoadingSpinner() {
  return (
    <div className="bg-white rounded-lg shadow-sm p-8 text-center mb-6">
      <div className="mx-auto mb-4 w-10 h-10 border-4 border-gray-200 border-t-blue-500 rounded-full animate-spin"></div>
      <p className="text-gray-600">Validating document...</p>
    </div>
  );
}

export default LoadingSpinner;

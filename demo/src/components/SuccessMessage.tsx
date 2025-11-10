function SuccessMessage() {
  return (
    <div className="flex items-start p-4 bg-green-50 border-l-4 border-green-500 rounded">
      <svg
        className="h-5 w-5 text-green-500 mr-3 mt-0.5"
        fill="currentColor"
        viewBox="0 0 20 20"
      >
        <path
          fillRule="evenodd"
          d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
          clipRule="evenodd"
        />
      </svg>
      <div>
        <h4 className="font-semibold text-green-800 mb-1">Validation Passed!</h4>
        <p className="text-green-700 text-sm">
          The document is valid according to the selected preset.
        </p>
      </div>
    </div>
  );
}

export default SuccessMessage;

import ValidationStats from './ValidationStats';
import ErrorList from './ErrorList';
import SuccessMessage from './SuccessMessage';
import type { ValidationResult } from '../types';

interface ValidationResultsProps {
  results: ValidationResult;
}

function ValidationResults({ results }: ValidationResultsProps) {
  const badge = results.success ? (
    <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800">
      ✓ Valid
    </span>
  ) : (
    <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-red-100 text-red-800">
      ✗ Invalid
    </span>
  );

  return (
    <div className="bg-white rounded-lg shadow-sm p-6 mb-6">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">Validation Results</h3>
        {badge}
      </div>

      <ValidationStats
        version={results.version}
        preset={results.preset}
        errorCount={results.errors.length}
      />

      {results.errors.length > 0 ? (
        <ErrorList errors={results.errors} />
      ) : (
        <SuccessMessage />
      )}
    </div>
  );
}

export default ValidationResults;

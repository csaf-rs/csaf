import InfoCard from './InfoCard';
import ErrorList from './ErrorList';
import Alert from './Alert';
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

      <InfoCard
        items={[
          { label: 'CSAF Version', value: results.version },
          { label: 'Preset Used', value: results.preset },
          { label: 'Errors Found', value: results.errors.length },
        ]}
        columns={3}
      />

      {results.errors.length > 0 ? (
        <ErrorList errors={results.errors} />
      ) : (
        <Alert
          type="success"
          title="Validation Passed!"
          message="The document is valid according to the selected preset."
        />
      )}
    </div>
  );
}

export default ValidationResults;

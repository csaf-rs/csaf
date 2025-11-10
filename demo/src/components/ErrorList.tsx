import type { ValidationError } from '../types';

interface ErrorListProps {
  errors: ValidationError[];
}

function ErrorList({ errors }: ErrorListProps) {
  return (
    <div className="space-y-2">
      {errors.map((error, index) => (
        <div key={index} className="border-l-4 border-red-500 bg-red-50 p-4 rounded">
          <div className="flex items-start">
            <div className="flex-shrink-0">
              <span className="inline-flex items-center justify-center h-6 w-6 rounded-full bg-red-100 text-red-800 text-xs font-bold">
                {index + 1}
              </span>
            </div>
            <div className="ml-3 flex-1">
              <p className="text-sm font-medium text-red-800">
                {error.message}
              </p>
              {error.instancePath && (
                <p className="mt-1 text-xs text-red-600">
                  Path: <code className="bg-red-100 px-1 py-0.5 rounded">{error.instancePath}</code>
                </p>
              )}
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}

export default ErrorList;

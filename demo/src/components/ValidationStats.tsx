import type { ValidationPreset } from '../types';

interface ValidationStatsProps {
  version: string;
  preset: ValidationPreset;
  errorCount: number;
}

function ValidationStats({ version, preset, errorCount }: ValidationStatsProps) {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-6">
      <div className="bg-gray-50 rounded-lg p-4">
        <span className="text-sm text-gray-600">CSAF Version</span>
        <p className="text-xl font-bold text-gray-900">{version}</p>
      </div>
      <div className="bg-gray-50 rounded-lg p-4">
        <span className="text-sm text-gray-600">Preset Used</span>
        <p className="text-xl font-bold text-gray-900 capitalize">{preset}</p>
      </div>
      <div className="bg-gray-50 rounded-lg p-4">
        <span className="text-sm text-gray-600">Errors Found</span>
        <p className="text-xl font-bold text-gray-900">{errorCount}</p>
      </div>
    </div>
  );
}

export default ValidationStats;

import type { ValidationPreset } from '../types';

interface ControlsProps {
  preset: ValidationPreset;
  onPresetChange: (preset: ValidationPreset) => void;
  onValidate: () => void;
  onClear: () => void;
  validateDisabled: boolean;
}

function Controls({ preset, onPresetChange, onValidate, onClear, validateDisabled }: ControlsProps) {
  return (
    <div className="bg-white rounded-lg shadow-sm p-6 mb-6">
      <div className="flex flex-col sm:flex-row gap-4 items-start sm:items-center">
        <div className="flex-1">
          <label htmlFor="preset" className="block text-sm font-medium text-gray-700 mb-2">
            Validation Preset
          </label>
          <select
            id="preset"
            value={preset}
            onChange={(e) => onPresetChange(e.target.value as ValidationPreset)}
            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="basic">Basic</option>
            <option value="extended">Extended</option>
            <option value="full">Full</option>
          </select>
        </div>
        <div className="flex gap-2 sm:mt-7">
          <button
            onClick={onValidate}
            disabled={validateDisabled}
            className="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors font-medium"
          >
            Validate
          </button>
          <button
            onClick={onClear}
            className="px-6 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors font-medium"
          >
            Clear
          </button>
        </div>
      </div>
    </div>
  );
}

export default Controls;

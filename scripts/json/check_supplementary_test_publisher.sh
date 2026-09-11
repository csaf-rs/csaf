#!/bin/bash
# Checks that all supplementary test files have the expected publisher field.
# Usage: ./check_supplementary_test_publisher.sh [--fix]
#   --fix        Corrects publisher fields in-place
set -euo pipefail

# Always run from the repository root so results are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

# Resolve to this script's directory
SCRIPT_PATH="$REPO_ROOT/scripts/json/validate_testcases_json_publisher.py"
TESTS_DIR="$REPO_ROOT/type-generator/assets/tests"
EXPECTED_PUBLISHER='{
  "category": "other",
  "name": "CSAF-RS Test Files",
  "namespace": "https://github.com/csaf-rs/csaf/tree/main/type-generator/assets/tests"
}'


# Call Python script with all arguments
exec python3 "$SCRIPT_PATH" "$TESTS_DIR" --publisher "$EXPECTED_PUBLISHER" "$@"

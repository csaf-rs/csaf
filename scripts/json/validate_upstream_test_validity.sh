#!/usr/bin/env bash
set -uo pipefail

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

echo "Building csaf-validator with release flag..."
cargo build -p csaf-validator --release

# Paths
VALIDATOR_PATH="$REPO_ROOT/target/release/csaf-validator"
CSAF_2_0_TESTCASES_JSON="$REPO_ROOT/csaf/csaf_2.0/test/validator/data/testcases.json"
CSAF_2_1_TESTCASES_JSON="$REPO_ROOT/csaf/csaf_2.1/test/validator/data/testcases.json"

ANY_ERRORS_FOUND=0

echo "Checking upstream tests for CSAF 2.0..."
python3 "$REPO_ROOT/scripts/json/validate_testcases_json_validity.py" "$VALIDATOR_PATH" "$CSAF_2_0_TESTCASES_JSON" "2.0" || ANY_ERRORS_FOUND=1

echo "Checking upstream tests for CSAF 2.1..."
python3 "$REPO_ROOT/scripts/json/validate_testcases_json_validity.py" "$VALIDATOR_PATH" "$CSAF_2_1_TESTCASES_JSON" "2.1" || ANY_ERRORS_FOUND=1

exit $ANY_ERRORS_FOUND

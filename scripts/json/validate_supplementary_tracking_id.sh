#!/usr/bin/env bash
# Validate supplementary test files adhere to tracking id / filename rules
#
# Usage: ./validate_supplementary_tracking_id.sh

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}

TEST_ROOT="$REPO_ROOT/type-generator/assets/tests"
SCRIPT_PATH="$REPO_ROOT/scripts/json/validate_testcases_json_tracking_id.py"
PREFIX_2_0="CSAF-RS_CSAF-CSAF_2_0-"
PREFIX_2_1="CSAF-RS_CSAF-CSAF_2_1-"

# Validate test files tracking ID prefixes
python3 "$SCRIPT_PATH" \
  "$TEST_ROOT" \
  --prefix-2-0 "$PREFIX_2_0" \
  --prefix-2-1 "$PREFIX_2_1" \

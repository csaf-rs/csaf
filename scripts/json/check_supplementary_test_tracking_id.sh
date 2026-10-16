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

# *-schema-s07.json tests that schema errors at the document root are correctly reported on by
# omitting the document property, so they can't have a valid tracking ID, and are therefore ignored in this check
IGNORE_ARGS=(
  --ignore-filename "csaf-rs_csaf-csaf_2_0-schema-s07.json"
  --ignore-filename "csaf-rs_csaf-csaf_2_1-schema-s07.json"
)

# Validate test files tracking ID prefixes
python3 "$SCRIPT_PATH" \
  "$TEST_ROOT" \
  --prefix-2-0 "$PREFIX_2_0" \
  --prefix-2-1 "$PREFIX_2_1" \
  "${IGNORE_ARGS[@]}"

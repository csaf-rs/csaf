#!/usr/bin/env bash
# Validates that all JSON files in the repository are syntactically correct.
# Uses jq to parse each file and reports any that fail to parse.
#
# Usage: ./check_json_syntax.sh [-q|--quiet]
#   -q, --quiet  Only print filenames, suppress error details

set -euo pipefail

# Parse command-line flags
QUIET=0
for arg in "$@"; do
  case "$arg" in
    --quiet|-q) QUIET=1 ;;
  esac
done

echo "Validating JSON syntax..."
VALID=0
INVALID=0

# Find all JSON files (excluding ignored directories) and validate each one
while IFS= read -r file; do
  if ! jq_err=$(jq empty "$file" 2>&1); then
    echo "Invalid JSON found: $file"
    if [ "$QUIET" -eq 0 ]; then
      echo "   $jq_err"
    fi
    ((INVALID++))
  else
    ((VALID++))
  fi
done < <(find . -name '*.json' -not -path './target/*' -not -path './.git/*' -not -path './csaf/*' -not -path './ssvc/*' -not -path './csaf-rs/assets/*' -not -path './.vscode/*' -not -path './scripts/*')

echo ""
echo "Results: $VALID valid, $INVALID invalid ($(( VALID + INVALID )) total)"

if [ "$INVALID" -gt 0 ]; then
  echo "JSON validation failed."
  exit 1
fi
echo "All JSON files are valid."

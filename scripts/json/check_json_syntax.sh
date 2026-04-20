#!/usr/bin/env bash
# Validates that all JSON files in the repository are syntactically correct.
# Uses jq to parse each file and reports any that fail to parse.
#
# Usage: ./check_json_syntax.sh [-q|--quiet]
#   -q, --quiet  Only print filenames, suppress error details

set -euo pipefail

# Always run from the repository root so results are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

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
    ((++INVALID))
  else
    ((++VALID))
  fi
done < <(find . \( -name 'target' -o -path './.git' -o -path './csaf' -o -path './ssvc' -o -path './csaf-rs/assets' -o -path './.vscode' -o -path './scripts' \) -prune -o -name '*.json' -not -name '*testcases.json' -not -name '*testcases_json_schema.json' -print)

echo ""
echo "Results: $VALID valid, $INVALID invalid ($(( VALID + INVALID )) total)"

if [ "$INVALID" -gt 0 ]; then
  echo "JSON validation failed."
  exit 1
fi
echo "All JSON files are valid."

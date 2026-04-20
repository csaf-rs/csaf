#!/usr/bin/env bash
# Checks that all JSON files have their keys sorted alphabetically.
# Compares each file against a version with sorted keys (jq --sort-keys).
#
# Usage: ./check_json_sorting.sh [-q|--quiet] [--fix]
#   -q, --quiet  Only print filenames, suppress diff details
#   --fix        Automatically sort keys in-place
#
# Only considers supplemental test cases in the type-generator (type-generator/assets/tests/).
# Excludes test files for test 6.2.13 (sorting), as they intentionally contain unsorted keys.

set -euo pipefail

FIX=0
QUIET=0
for arg in "$@"; do
  case "$arg" in
    --fix) FIX=1 ;;
    --quiet|-q) QUIET=1 ;;
  esac
done

echo "Checking JSON key sorting..."
SORTED=0
UNSORTED=0
FIXED=0
while IFS= read -r file; do
  if ! jq empty "$file" 2>/dev/null; then
    # skip invalid JSON files, those are caught by validate_json.sh
    continue
  fi
  if ! sort_diff=$(diff <(jq --sort-keys . "$file") <(jq . "$file") 2>&1); then
    if [ "$FIX" -eq 1 ]; then
      tmp=$(mktemp)
      jq --sort-keys . "$file" > "$tmp" && mv "$tmp" "$file"
      echo "Fixed sorting: $file"
      ((FIXED++))
    else
      echo "Unsorted keys in JSON: $file"
      if [ "$QUIET" -eq 0 ]; then
        echo "$sort_diff" | head -50 | sed 's/^/   /'
      fi
      ((UNSORTED++))
    fi
  else
    ((SORTED++))
  fi
done < <(find ./type-generator/assets/tests -name '*.json' -not -name '*6-2-13*' -not -name '*6_2_13*')

echo ""
if [ "$FIX" -eq 1 ]; then
  echo "Results: $SORTED already sorted, $FIXED fixed ($(( SORTED + FIXED )) total)"
  echo "All JSON files now have alphabetically sorted keys."
else
  echo "Results: $SORTED sorted, $UNSORTED unsorted ($(( SORTED + UNSORTED )) total)"
  if [ "$UNSORTED" -gt 0 ]; then
    echo "JSON sorting check failed. Run with --fix to auto-fix."
    exit 1
  fi
  echo "All JSON files have alphabetically sorted keys."
fi

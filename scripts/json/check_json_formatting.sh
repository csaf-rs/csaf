#!/usr/bin/env bash
# Checks that all JSON files are well-formatted.
# Well-formatted means: 2-space indentation, consistent spacing.
# Compares each file against the output of `jq .`.
#
# Usage: ./check_json_formatting.sh [-q|--quiet] [--fix]
#   -q, --quiet  Only print filenames, suppress diff details
#   --fix        Automatically reformat files in-place

set -euo pipefail

# Always run from the repository root so results are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

FIX=0
QUIET=0
for arg in "$@"; do
  case "$arg" in
    --fix) FIX=1 ;;
    --quiet|-q) QUIET=1 ;;
  esac
done

echo "Checking JSON formatting..."
WELL_FORMATTED=0
MAL_FORMATTED=0
FIXED=0
while IFS= read -r file; do
  if ! jq empty "$file" 2>/dev/null; then
    # skip invalid JSON files, those are caught by check_json_syntax.sh
    continue
  fi
  if ! fmt_diff=$(diff <(jq . "$file") "$file" 2>&1); then
    if [ "$FIX" -eq 1 ]; then
      tmp=$(mktemp)
      jq . "$file" > "$tmp" && mv "$tmp" "$file"
      echo "Fixed formatting: $file"
      ((++FIXED))
    else
      echo "⚠Malformatted JSON found: $file"
      if [ "$QUIET" -eq 0 ]; then
        echo "$fmt_diff" | head -50 | sed 's/^/   /'
      fi
      ((++MAL_FORMATTED))
    fi
  else
    ((++WELL_FORMATTED))
  fi
done < <(find . \( -path './target' -o -path './.git' -o -path './csaf' -o -path './ssvc' -o -path './csaf-rs/assets' -o -path './.vscode' -o -path './scripts' \) -prune -o -name '*.json' -print)

echo ""
if [ "$FIX" -eq 1 ]; then
  echo "Results: $WELL_FORMATTED already well-formatted, $FIXED fixed ($(( WELL_FORMATTED + FIXED )) total)"
  echo "All JSON files are now well-formatted."
else
  echo "Results: $WELL_FORMATTED well-formatted, $MAL_FORMATTED malformatted ($(( WELL_FORMATTED + MAL_FORMATTED )) total)"
  if [ "$MAL_FORMATTED" -gt 0 ]; then
    echo "JSON formatting check failed. Run with --fix to auto-fix."
    exit 1
  fi
  echo "All JSON files are well-formatted."
fi

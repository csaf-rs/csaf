#!/bin/bash

set -euo pipefail

# Always run from the repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

# input / output setup

# directory and name of the word frequency download script
SCRIPT_DIR="$REPO_ROOT/scripts/wordfreq"
SCRIPT="download_wordfreq.py"
# output directory for the boiled down frequency dicts
OUTPUT_DIR="$REPO_ROOT/csaf-rs/assets/wordfreq"

# frequency dict sources (to be extended)

# source url for the word frequency files (from pyspellchecker)
# currently, we only have one source, so this is hard-coupled down below
RAW_PY_SPELLCHECKER_GITHUB_URL="https://raw.githubusercontent.com/barrust/pyspellchecker/master/spellchecker/resources"
PY_SPELLCHECKER_LANGUAGES=(ar de en es eu fa fr it lv nl pt ru)

# top N words to boil down to
TOP_N=10000

# ensure the output directory exists
mkdir -p "$OUTPUT_DIR"

echo "Downloading language files to $OUTPUT_DIR..."
for lang in "${PY_SPELLCHECKER_LANGUAGES[@]}"; do
    URL="$RAW_PY_SPELLCHECKER_GITHUB_URL/$lang.json.gz"
    OUTPUT_FILE="$OUTPUT_DIR/${lang}_${TOP_N}.txt"
    echo "Downloading $lang..."
    python3 "$SCRIPT_DIR/$SCRIPT" -u "$URL" -n "$TOP_N" -o "$OUTPUT_FILE"
done

echo "Done!"

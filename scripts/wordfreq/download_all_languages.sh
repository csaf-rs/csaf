#!/bin/bash

set -euox pipefail

# Always run from the repository root
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

SCRIPT_DIR="scripts/wordfreq"
SCRIPT="download_wordfreq.py"
OUTPUT_DIR="$REPO_ROOT/csaf-rs/assets/wordfreq"
RAW_GITHUB_URL="https://raw.githubusercontent.com/barrust/pyspellchecker/master/spellchecker/resources"

LANGUAGES=(ar de en es eu fa fr it lv nl pt ru)
TOP_N=10000

mkdir -p "$OUTPUT_DIR"

echo "Downloading language files to $OUTPUT_DIR..."
cd "$SCRIPT_DIR"
for lang in "${LANGUAGES[@]}"; do
    URL="$RAW_GITHUB_URL/$lang.json.gz"
    echo "  Downloading $lang..."
    python3 "$SCRIPT" -u "$URL" -n "$TOP_N" -c
    rm -f "${lang}_${TOP_N}.txt"
done

echo "Done!"

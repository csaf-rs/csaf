#!/usr/bin/env bash
# Converts cargo-llvm-cov JSON output (stdin) into a per-file Markdown coverage table.
# Usage: cargo llvm-cov --json ... | ./scripts/coverage/coverage_per_file.sh

set -euo pipefail

jq -r '
  "| File | Lines | Functions | Regions | Line Coverage |",
  "| ---- | ----: | --------: | ------: | -----------: |",
  (.data[0].files | sort_by(-.summary.lines.percent) | .[] |
    .filename as $f |
    .summary |
    "| \($f | split("/") | .[-3:] | join("/")) | \(.lines.covered)/\(.lines.count) | \(.functions.covered)/\(.functions.count) | \(.regions.covered)/\(.regions.count) | \(.lines.percent * 100 | round / 100)% |"
  )
'

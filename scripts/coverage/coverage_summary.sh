#!/usr/bin/env bash
# Converts cargo-llvm-cov JSON output (stdin) into a Markdown summary table.
# Usage: cargo llvm-cov --json --summary-only ... | ./scripts/coverage/coverage_summary.sh

set -euo pipefail

jq -r '
  .data[0].totals |
  "| Metric | Covered | Total | Coverage |",
  "| ------ | ------: | ----: | -------: |",
  "| Lines | \(.lines.covered) | \(.lines.count) | \(.lines.percent * 100 | round / 100)% |",
  "| Functions | \(.functions.covered) | \(.functions.count) | \(.functions.percent * 100 | round / 100)% |",
  "| Regions | \(.regions.covered) | \(.regions.count) | \(.regions.percent * 100 | round / 100)% |",
  "| Instantiations | \(.instantiations.covered) | \(.instantiations.count) | \(.instantiations.percent * 100 | round / 100)% |"
'

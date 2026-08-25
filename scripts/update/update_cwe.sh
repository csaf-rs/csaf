#!/usr/bin/env bash
set -euo pipefail

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

# CWE version to fetch, e.g. v4.20 (defaults to "latest")
VERSION=${1:-latest}
echo "Updating $VERSION"

# download the CWE XML archive for the given version
curl -fS "https://cwe.mitre.org/data/xml/cwec_${VERSION}.xml.zip" |
# unzip it
funzip |
# convert it to CSV via XSLT
xsltproc scripts/update/convert-cwe-to-csv.xslt - |
# sort by CWE ID (skipping the header row)
# and write the result to a CSV file named with the CWE version and CWE date from the header
(read -r header; sort -n > "csaf-rs/assets/cwe/cwe_${header}.csv" )

#!/usr/bin/env bash
set -euo pipefail

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

VERSION=${1:-latest}
curl -fS https://cwe.mitre.org/data/xml/cwec_${VERSION}.xml.zip | zcat | xsltproc scripts/update/convert-cwe-to-csv.xslt - | (read -r header; sort -n > csaf-rs/assets/cwe/cwe_${header}.csv )

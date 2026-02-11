#!/usr/bin/env bash
set -euo pipefail
VERSION=${1:-latest}
curl -fS https://cwe.mitre.org/data/xml/cwec_${VERSION}.xml.zip | zcat | xsltproc convert-cwe-to-csv.xslt - | (read -r header; sort -n > csaf-rs/assets/cwe/cwe_${header}.csv )

#!/usr/bin/env bash
set -euo pipefail
VERSION=${1:-latest}
curl -fS https://cwe.mitre.org/data/xml/cwec_${VERSION}.xml.zip | zcat | xsltproc convert-cwe-to-csv.xslt  - | sort -n > csaf-rs/assets/cwe/cwe-${VERSION}.csv

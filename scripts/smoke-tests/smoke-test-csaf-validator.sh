#!/usr/bin/env bash
# Run this script from any directory inside the repository with the path to a
# built csaf-validator binary. It checks that a valid CSAF document is accepted
# and that an invalid CSAF document is rejected.
#
# Usage: ./scripts/smoke-tests/smoke-test-csaf-validator.sh PATH_TO_CSAF_VALIDATOR

set -euo pipefail

# show help if path to csaf validator binary is missing
if [[ $# -ne 1 ]]; then
    echo "Usage: $0 PATH_TO_CSAF_VALIDATOR" >&2
    exit 2
fi

# preserve paths relative to the caller before switching to the repository root.
case "$1" in
    /*) VALIDATOR=$1 ;; # absolute paths
    *) VALIDATOR="$(pwd -P)/$1" ;; # relative paths
esac

# repo root anchor
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
    echo "Error: not inside a git repository." >&2
    exit 1
}
cd "$REPO_ROOT"

# absolute paths to test files
VALID_FILE="csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-01-11.json"
INVALID_FILE="csaf/csaf_2.1/test/validator/data/mandatory/oasis_csaf_tc-csaf_2_1-2024-6-1-01-01.json"

# valid file should pass
"${VALIDATOR}" "${VALID_FILE}"

# invalid file should fail
if "${VALIDATOR}" "${INVALID_FILE}"; then
    echo "csaf-validator accepted an invalid CSAF document!"
    exit 1
fi

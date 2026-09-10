#!/usr/bin/env bash
#
# create-new-testfile.sh
#
# Bootstraps a new supplementary CSAF test fixture under
# type-generator/assets/tests/csaf_<version>/, wires it up in the
# corresponding testcases.json, and regenerates the Rust test definitions.
#
# Usage:
#   ./create-new-testfile.sh --test <id> --case <failure|valid> [--csaf-version <2.0|2.1|both>]
#
# Examples:
#   ./create-new-testfile.sh --test 6.1.29 --validity failure
#   ./create-new-testfile.sh -t 1.29 --validity valid -c 2.1
#   ./create-new-testfile.sh -t 6.1.27.5 --validity failure -c both
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ASSETS_DIR="$SCRIPT_DIR/type-generator/assets/tests"

usage() {
  cat <<'EOF'
Usage: create-new-testfile.sh --test <id> --validity <failure|valid> [--csaf-version <2.0|2.1|both>]

Required:
  -t, --test <id>        Test number from the specification, e.g. "6.1.29"
  --validity <type>       Whether the new file is a "failure" (failing test case) or
                          "valid" case.

Optional:
  -c, --csaf-version <ver>    CSAF version to target: "2.0", "2.1", or "both". Defaults to "2.1".
  -h, --help              Show this help message.
EOF
}

TEST_ID=""
VERSION="2.1"
CASE_TYPE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    -t|--test)
      TEST_ID="${2:-}"
      shift 2
      ;;
    -c|--csaf-version)
      VERSION="${2:-}"
      shift 2
      ;;
    --validity)
      CASE_TYPE="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ -z "$TEST_ID" ]]; then
  echo "Error: --test is required" >&2
  usage >&2
  exit 1
fi

if [[ -z "$CASE_TYPE" ]]; then
  echo "Error: --validity is required" >&2
  usage >&2
  exit 1
fi

command -v jq >/dev/null 2>&1 || { echo "Error: jq is required but not installed" >&2; exit 1; }

# Normalize case type to "failure" or "valid"
case "$CASE_TYPE" in
  failure)
    CASE_TYPE="failure"
    ;;
  valid)
    CASE_TYPE="valid"
    ;;
  *)
    echo "Error: --case must be one of failure/fail/invalid or valid/pass (got '$CASE_TYPE')" >&2
    exit 1
    ;;
esac

if ! [[ "$TEST_ID" =~ ^6(\.[0-9]+)+$ ]]; then
  echo "Error: invalid test id '$TEST_ID' (expected e.g. '6.1.29' or '6.1.27.5')" >&2
  exit 1
fi

case "$VERSION" in
  2.0) VERSIONS=(2.0) ;;
  2.1) VERSIONS=(2.1) ;;
  both) VERSIONS=(2.0 2.1) ;;
  *)
    echo "Error: --csaf-version must be one of 2.0, 2.1, or both (got '$VERSION')" >&2
    exit 1
    ;;
esac

# Split the test id into its dot-separated numeric components, dropping the
# leading "6".
IFS='.' read -r -a ID_PARTS <<< "$TEST_ID"
SECTION="${ID_PARTS[1]}"

process_version() {
  local version="$1"
  local version_with_underlines="${version/./_}"
  local dir="$ASSETS_DIR/csaf_${version}"
  local testcases_json="$dir/testcases.json"
  local template="$dir/csaf-rs_csaf-csaf_${version_with_underlines}-TEMPLATE.json"

  if [[ ! -f "$testcases_json" ]]; then
    echo "Error: testcases.json not found at $testcases_json" >&2
    exit 1
  fi
  if [[ ! -f "$template" ]]; then
    echo "Error: template file not found at $template" >&2
    exit 1
  fi

  local group
  case "$SECTION" in
    1) group="mandatory" ;;
    2) if [[ "$version" == "2.0" ]]; then group="optional"; else group="recommended"; fi ;;
    3) group="informative" ;;
    *)
      echo "Error: unsupported section '6.$SECTION.*' (expected 6.1.*, 6.2.*, or 6.3.*)" >&2
      exit 1
      ;;
  esac

  if [[ ! -d "$dir/$group" ]]; then
    echo "Error: group folder not found at $dir/$group" >&2
    exit 1
  fi

  # Build the filename's numeric segment: the first component after "6" (the
  # section, e.g. "1") is kept as-is, all following components are
  # zero-padded to two digits (e.g. "6.1.27.5" -> "6-1-27-05").
  local id_middle="${ID_PARTS[1]}"
  local i
  for ((i = 2; i < ${#ID_PARTS[@]}; i++)); do
    id_middle="${id_middle}-$(printf '%02d' "${ID_PARTS[$i]}")"
  done

  local file_prefix="csaf-rs_csaf-csaf_${version_with_underlines}-6-${id_middle}"

  # Find the array (failures/valid) key to use in testcases.json
  local array_key
  if [[ "$CASE_TYPE" == "failure" ]]; then
    array_key="failures"
  else
    array_key="valid"
  fi

  local existing_entry
  existing_entry=$(jq --arg id "$TEST_ID" '.tests[] | select(.id == $id)' "$testcases_json")

  # Determine the next free case number. The tens digit of the case number
  # indicates validity (even = failure, odd = valid): failures start at 01,
  # valid cases start at 11. Once a decade (e.g. 01-09 or 11-19) is used up,
  # the next case of the same type jumps to the next same-parity decade,
  # e.g. after s09 the next failure case is s21 and after s19 the next valid case is s31.
  local max_num=-1
  if [[ -n "$existing_entry" ]]; then
    local existing_names
    existing_names=$(jq -r --arg key "$array_key" '.[$key] // [] | .[].name' <<< "$existing_entry")
    while IFS= read -r name; do
      [[ -z "$name" ]] && continue
      if [[ "$name" =~ -s([0-9]{2})\.json$ ]]; then
        local n=$((10#${BASH_REMATCH[1]}))
        if ((n > max_num)); then
          max_num=$n
        fi
      fi
    done <<< "$existing_names"
  fi

  local case_num
  if ((max_num < 0)); then
    if [[ "$CASE_TYPE" == "failure" ]]; then
      case_num=1
    else
      case_num=11
    fi
  else
    local decade=$((max_num / 10))
    local remainder=$((max_num % 10))
    if ((remainder < 9)); then
      case_num=$((max_num + 1))
    else
      local next_decade=$((decade + 2))
      case_num=$((next_decade * 10 + 1))
    fi
  fi
  local case_str
  case_str=$(printf '%02d' "$case_num")

  local new_filename="${file_prefix}-s${case_str}.json"
  local rel_name="${group}/${new_filename}"
  local new_filepath="$dir/$rel_name"

  if [[ -e "$new_filepath" ]]; then
    echo "Error: file already exists at $new_filepath" >&2
    exit 1
  fi

  local tracking_id
  tracking_id=$(echo "${new_filename%.json}" | tr '[:lower:]' '[:upper:]')

  local case_description="failing"
  local valid_bool="false"
  if [[ "$CASE_TYPE" == "valid" ]]; then
    case_description="valid"
    valid_bool="true"
  fi

  # Create the test fixture from the template, updating its tracking id and
  # title with a placeholder that should be edited afterwards.
  jq --arg id "$tracking_id" \
     --arg title "TODO: describe $case_description supplementary example for test $TEST_ID" \
     '.document.tracking.id = $id | .document.title = $title' \
     "$template" > "$new_filepath"

  # Wire the new file into testcases.json, either by adding a new test entry
  # or by appending to the failures/valid array of an existing one.
  local tmp_json
  tmp_json="$(mktemp)"
  if [[ -z "$existing_entry" ]]; then
    jq --arg id "$TEST_ID" \
       --arg group "$group" \
       --arg key "$array_key" \
       --arg name "$rel_name" \
       --argjson valid "$valid_bool" \
       '.tests += [{id: $id, group: $group, ($key): [{name: $name, valid: $valid}]}]' \
       "$testcases_json" > "$tmp_json"
  else
    jq --arg id "$TEST_ID" \
       --arg key "$array_key" \
       --arg name "$rel_name" \
       --argjson valid "$valid_bool" \
       '(.tests[] | select(.id == $id) | .[$key]) |= ((. // []) + [{name: $name, valid: $valid}])' \
       "$testcases_json" > "$tmp_json"
  fi
  mv "$tmp_json" "$testcases_json"

  echo "Created $new_filepath"
  echo "Updated $testcases_json (test $TEST_ID, group '$group', $array_key -> $rel_name)"
}

for v in "${VERSIONS[@]}"; do
  process_version "$v"
done

echo "Running type-generator..."
cargo run -p type-generator

echo "Done."

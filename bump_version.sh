#!/usr/bin/env bash
set -euo pipefail

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

usage() {
  cat >&2 <<EOF
Usage: $(basename "$0") <command>

Commands:
  major             Bump the major version (x.y.z -> x+1.0.0)
  minor             Bump the minor version (x.y.z -> x.y+1.0)
  patch             Bump the patch version (x.y.z -> x.y.z+1)
  set <version>     Set an explicit version (e.g. 1.2.3)

The following files are updated:
  Cargo.toml            (workspace.package.version)
  csaf-validator/Cargo.toml  (path dependency on csaf-rs)
  csaf-converter/Cargo.toml  (path dependency on csaf-rs)
  csaf-result-json/Cargo.toml  (path dependency on csaf-rs)
  wasm/package.json
EOF
  exit 1
}

# --- Parse arguments -------------------------------------------------------

COMMAND=${1:-}
case "$COMMAND" in
  major|minor|patch) ;;
  set)
    NEW_VERSION=${2:-}
    if [[ -z "$NEW_VERSION" ]]; then
      echo "Error: 'set' requires a version argument." >&2
      usage
    fi
    if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
      echo "Error: version must be in the form MAJOR.MINOR.PATCH (got: '$NEW_VERSION')." >&2
      exit 1
    fi
    ;;
  *) usage ;;
esac

# --- Read current version from workspace Cargo.toml ------------------------

CURRENT_VERSION=$(grep -m1 '^version\s*=' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
if [[ -z "$CURRENT_VERSION" ]]; then
  echo "Error: could not read current version from Cargo.toml." >&2
  exit 1
fi

IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

# --- Compute new version ---------------------------------------------------

case "$COMMAND" in
  major) NEW_VERSION="$((MAJOR + 1)).0.0" ;;
  minor) NEW_VERSION="${MAJOR}.$((MINOR + 1)).0" ;;
  patch) NEW_VERSION="${MAJOR}.${MINOR}.$((PATCH + 1))" ;;
  set)   ;; # already set above
esac

echo "Bumping version: $CURRENT_VERSION -> $NEW_VERSION"

# --- Update files ----------------------------------------------------------

# workspace Cargo.toml: [workspace.package] version field
# workspace Cargo.toml: the [workspace.package] version field (first occurrence)
sed -i.bak "s/^version = \"[^\"]*\"/version = \"${NEW_VERSION}\"/" Cargo.toml

# path dependencies on csaf-rs in csaf-validator and csaf-converter
for toml in csaf-validator/Cargo.toml csaf-converter/Cargo.toml csaf-result-json/Cargo.toml; do
  sed -i.bak 's|\(path = "\.\./csaf-rs", version = "\)[^"]*"|\1'"${NEW_VERSION}"'"|' "$toml"
done

# wasm/package.json
sed -i.bak 's/"version": "[^"]*"/"version": "'"${NEW_VERSION}"'"/' wasm/package.json

# Remove sed backup files
find . -maxdepth 3 -name "*.bak" -delete

echo "Done. Version is now $NEW_VERSION."

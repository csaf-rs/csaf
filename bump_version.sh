#!/usr/bin/env bash
set -euo pipefail

# Always run from the repository root so paths are deterministic
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "Error: not inside a git repository." >&2
  exit 1
}
cd "$REPO_ROOT"

# --- Prerequisite check ----------------------------------------------------

if ! command -v cargo-set-version &>/dev/null && ! cargo set-version --version &>/dev/null 2>&1; then
  echo "Error: cargo-edit is not installed. Run: cargo install cargo-edit" >&2
  exit 1
fi

if ! command -v npm &>/dev/null; then
  echo "Error: npm is not installed." >&2
  exit 1
fi

# --- Usage -----------------------------------------------------------------

usage() {
  cat >&2 <<EOF
Usage: $(basename "$0") <command>

Commands:
  major             Bump the major version (x.y.z -> x+1.0.0)
  minor             Bump the minor version (x.y.z -> x.y+1.0)
  patch             Bump the patch version (x.y.z -> x.y.z+1)
  set <version>     Set an explicit version (e.g. 1.2.3)

Tools required: cargo-edit (cargo install cargo-edit), npm
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

# --- Update Cargo versions -------------------------------------------------

case "$COMMAND" in
  major|minor|patch)
    cargo set-version --bump "$COMMAND"
    ;;
  set)
    cargo set-version "$NEW_VERSION"
    ;;
esac

# --- Resolve the new version -----------------------------------------------

NEW_VERSION=$(grep -m1 '^version\s*=' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')

# --- Update wasm/package.json ----------------------------------------------

npm version --no-git-tag-version --prefix wasm "$NEW_VERSION"

echo "Done. Version is now $NEW_VERSION."

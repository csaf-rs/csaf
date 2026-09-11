#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path

TEST_PRESETS: dict[str, tuple[str, ...]] = {
    "csaf_2.0": ("informative", "mandatory", "optional", "schema"),
    "csaf_2.1": ("informative", "mandatory", "recommended", "schema"),
}

def validate_tracking_id_prefix(
    tracking_id: str, filename: str, expected_prefix: str
) -> bool:
    """
    Validate that the tracking ID prefix aligns with CSAF version folder and filename.

    Checks:
    1. Tracking ID has the correct version prefix for the folder
    2. Filename matches the tracking ID (case-insensitive, minus .json extension, this is a simplification of the actual
    ruleset for tracking ID -> filename derivation, but works for our supplemental test files)
    """
    # no tracking id at all
    if not tracking_id:
        print(f"Tracking id is missing in file {filename}")
        return False

    # check if tracking ID starts with correct prefix
    if not tracking_id.startswith(expected_prefix):
        print(f"Tracking ID prefix mismatch. Expected: {expected_prefix}, Found: {tracking_id}, in file: {filename}")
        return False

    # filename should be tracking id in lower case, without .json extension
    filename_without_ext = filename.replace(".json", "")
    tracking_id_lowercase = tracking_id.lower()
    if not filename_without_ext == tracking_id_lowercase:
        print(f"Filename does not match tracking ID. Filename without extension: {filename_without_ext}, Tracking ID: {tracking_id_lowercase}")
        return False

    return True

def main() -> int:
    parser = argparse.ArgumentParser(
        description="Validate and optionally fix publisher properties in CSAF test files"
    )
    parser.add_argument(
        "test_root",
        type=Path,
        help="Path to test root directory",
    )
    parser.add_argument(
        "--prefix-2-0",
        type=str,
        required=True,
        help="Expected tracking ID prefix for CSAF 2.0)",
    )
    parser.add_argument(
        "--prefix-2-1",
        type=str,
        required=True,
        help="Expected tracking ID prefix for CSAF 2.1)",
    )
    args = parser.parse_args()

    tests_root = args.test_root

    # stats
    has_errors = False
    has_exceptions = False

    for csaf_version in sorted(TEST_PRESETS):
        print(f"Validating tracking IDs for CSAF version: {csaf_version}")
        expected_prefix = args.prefix_2_0 if csaf_version == "csaf_2.0" else args.prefix_2_1
        for preset in TEST_PRESETS[csaf_version]:
            directory = tests_root / csaf_version / preset
            # check if directory exists
            if not directory.is_dir():
                raise FileNotFoundError(f"Expected to find test group directory: {directory}")

            # iterate all files
            for file_path in sorted(directory.glob("*.json")):
                try:
                    with open(file_path, "r", encoding="utf-8") as f:
                        data = json.load(f)

                    # validate tracking_id
                    tracking_id = data.get("document", {}).get("tracking", {}).get("id")
                    valid = validate_tracking_id_prefix(tracking_id, file_path.name, expected_prefix)
                    if not valid:
                        has_errors = True

                except Exception as e:
                    print(f"{file_path.name}: {type(e).__name__}: {e}")
                    has_exceptions = True

    return 1 if has_exceptions or has_errors else 0

if __name__ == "__main__":
    exit(main())
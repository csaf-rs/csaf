#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path

TEST_PRESETS: dict[str, tuple[str, ...]] = {
    "csaf_2.0": ("informative", "mandatory", "optional", "schema"),
    "csaf_2.1": ("informative", "mandatory", "recommended", "schema"),
}

EXPECTED_PUBLISHER = {
    "category": "other",
    "name": "CSAF-RS Test Files",
    "namespace": "https://github.com/csaf-rs/csaf/tree/main/type-generator/assets/tests",
}

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
        "--publisher",
        type=json.loads,
        required=True,
        help="Expected publisher as JSON string",
    )
    parser.add_argument(
        "--fix",
        action="store_true",
        help="Fix invalid publisher properties in-place",
    )
    args = parser.parse_args()

    tests_root = args.test_root
    expected_publisher = args.publisher
    
    # stats
    has_errors = False
    has_exceptions = False

    for csaf_version in sorted(TEST_PRESETS):
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

                    # validate publisher
                    publisher = data.get("document", {}).get("publisher", {})
                    if publisher != expected_publisher:
                        has_errors = True
                        if args.fix:
                            data["document"]["publisher"] = expected_publisher
                            with open(file_path, "w", encoding="utf-8") as f:
                                json.dump(data, f, indent=2)
                                f.write("\n")
                            print(f"Fixed wrong publisher in file {file_path.name}")
                        else:
                            print(f"Found wrong publisher in file {file_path.name}")

                except Exception as e:
                    print(f"{file_path.name}: {type(e).__name__}: {e}")
                    has_exceptions = True

    return 1 if has_exceptions or (has_errors and not args.fix) else 0

if __name__ == "__main__":
    exit(main())

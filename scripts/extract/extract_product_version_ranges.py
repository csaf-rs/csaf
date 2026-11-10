#!/usr/bin/env python3
"""List product version range names in JSON files."""

import argparse
import json
from enum import Enum
from pathlib import Path


class VersionSystem(str, Enum):
    VERS = "vers"
    NON_VERS = "non-vers"


def find_ranges(value: object, system: VersionSystem):

    if isinstance(value, dict):
        if (
            value.get("category") == "product_version_range"
            and isinstance(value.get("name"), str)
            and (
                value["name"].startswith("vers:")
                == (system is VersionSystem.VERS)
            )
        ):
            yield value["name"]
        for child in value.values():
            yield from find_ranges(child, system)
    elif isinstance(value, list):
        for child in value:
            yield from find_ranges(child, system)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Find product_version_range names in JSON files."
    )
    parser.add_argument(
        "directory",
        nargs="?",
        type=Path,
        default=Path.cwd(),
        help="Directory to scan (default: current directory)",
    )
    parser.add_argument(
        "--system",
        choices=[system.value for system in VersionSystem],
        default=VersionSystem.NON_VERS.value,
        help="Report vers or non-vers names (either vers or non-vers, default non-vers)",
    )
    args = parser.parse_args()
    root = args.directory.resolve()
    system = VersionSystem(args.system)

    print("version range\tfilename")
    for path in sorted(root.rglob("*.json")):
        for version in find_ranges(
            json.loads(path.read_text(encoding="utf-8")), system
        ):
            print(f"{version}\t{path.relative_to(root)}")


if __name__ == "__main__":
    main()

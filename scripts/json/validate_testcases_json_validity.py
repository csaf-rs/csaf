#!/usr/bin/env python3
"""
Validate testcases.json file valid fields through csaf-validator.

This script runs each test case through the csaf-validator with the basic (schema + mandatory) preset and
compares the validation results to the test cases' valid field.
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple


class TestValidator:
    def __init__(self, validator_path: Path, testcase_json_path: Path, csaf_version: str):
        self.validator_path = validator_path
        self.testcase_json_path = testcase_json_path
        self.csaf_version = csaf_version

    def validate_file(self, test_file: Path) -> bool:
        """
        Validate a single test file.
        Returns True if validation succeeds (no mandatory + schema errors), False if validation fails, timeout is reached
        or the csaf-validator throws an error.
        """
        cmd = [
            str(self.validator_path),
            str(test_file),
            "-C", self.csaf_version,
            "-T", "basic",
        ]
        
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=10
            )
            return result.returncode == 0
        except subprocess.TimeoutExpired:
            print(f"  ⚠ Timeout validating {test_file.name}")
            return False
        except Exception as e:
            print(f"  ⚠ Error validating {test_file.name}: {e}")
            return False

    def process_testcases_json_file(self) -> int:
        """
        Process testcases.json file, validating supplementary tests.
        Returns error count number
        """
        error_count = 0
        # load testcases json file
        with open(self.testcase_json_path, 'r') as f:
            testcases = json.load(f)

        # iterate all tests (6.1.1, 6.1.2, ...) in file
        for test in testcases.get("tests", []):
            test_id = test.get("id")
            # iterate both "failures" and "valid" sections
            for test_group in ["failures", "valid"]:
                if test_group in test:
                    # iterate all test cases per failures / valid
                    for test_case in test[test_group]:
                        # construct test case file path
                        test_case_file_path = self.testcase_json_path.parent / test_case["name"]

                        # test case file not found
                        if not test_case_file_path.exists():
                            print(f"⚠ Test case file not found: {test_case_file_path}")
                            error_count += 1
                            continue

                        # get validity from testcases.json
                        expected = test_case.get("valid")

                        # validate the test case file
                        is_valid = self.validate_file(test_case_file_path)


                        # compare testcases.json validity vs csaf-validator basic (schema + mandatory) run result
                        if is_valid != expected:
                            error_count += 1
                            print(f"⚠ {test_id}: {test_case['name']} (csaf-validator={is_valid}, testcases.json={expected})")


        return error_count


def main():
    parser = argparse.ArgumentParser(
        description="Validate testcases.json file valid fields through csaf-validator."
    )
    parser.add_argument(
        "validator_path",
        type=Path,
        help="Path to the csaf-validator binary"
    )
    parser.add_argument(
        "testcase_json_path",
        type=Path,
        help="Path to the testcases.json file"
    )
    parser.add_argument(
        "csaf_version",
        type=str,
        help="CSAF versions to validate"
    )
    
    args = parser.parse_args()

    # check version
    if not args.csaf_version in ["2.0", "2.1"]:
        print("Error: unknown csaf version. Please provide either 2.0 or 2.1.")
        sys.exit(1)

    # csaf-validator not found
    if not args.validator_path.exists():
        print(f"Error: csaf-validator not found at {args.validator_path}")
        print("Please provide a valid path to the csaf-validator binary")
        sys.exit(1)

    if not args.testcase_json_path.exists():
        print(f"Error: testcases.json not found at {args.testcase_json_path}")
        print("Please provide a valid path to the testcases.json file")
        sys.exit(1)
    
    validator = TestValidator(args.validator_path, args.testcase_json_path, args.csaf_version)
    error_count = validator.process_testcases_json_file()
    sys.exit(1 if error_count else 0)



if __name__ == "__main__":
    main()

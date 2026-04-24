package main

import (
	"fmt"
	"os"

	"github.com/csaf-rs/csaf/go/csaf_ffi"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <csaf-json-file> [preset]\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  preset: basic (default), extended, full\n")
		os.Exit(1)
	}

	path := os.Args[1]
	preset := "basic"
	if len(os.Args) > 2 {
		preset = os.Args[2]
	}

	data, err := os.ReadFile(path)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
		os.Exit(1)
	}

	result, err := csaf_ffi.ValidateCsaf(string(data), preset)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Validation error: %v\n", err)
		os.Exit(1)
	}

	status := "✅ VALID"
	if !result.Success {
		status = "❌ INVALID"
	}

	fmt.Printf("%s (CSAF %s)\n", status, result.Version)
	fmt.Printf("  Errors: %d, Warnings: %d, Infos: %d, Not Found: %d\n",
		result.NumErrors, result.NumWarnings, result.NumInfos, result.NumNotFound)

	for _, tr := range result.TestResults {
		switch tr.Status.(type) {
		case csaf_ffi.TestResultStatusSuccess:
			fmt.Printf("  ✅ %s\n", tr.TestId)
		case csaf_ffi.TestResultStatusFailure:
			f := tr.Status.(csaf_ffi.TestResultStatusFailure)
			fmt.Printf("  ❌ %s (%d errors, %d warnings, %d infos)\n",
				tr.TestId, len(f.Errors), len(f.Warnings), len(f.Infos))
		case csaf_ffi.TestResultStatusNotFound:
			fmt.Printf("  ⚠️  %s (not found)\n", tr.TestId)
		case csaf_ffi.TestResultStatusSkipped:
			fmt.Printf("  ⏭️  %s (skipped)\n", tr.TestId)
		}
	}
}

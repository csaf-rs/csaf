package csaf_ffi_test

import (
	"testing"

	"github.com/csaf-rs/csaf/go/csaf_ffi"
)

// Minimal valid CSAF 2.1 document for testing.
const validCsaf21 = `{
  "$schema": "https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json",
  "document": {
    "category": "csaf_base",
    "csaf_version": "2.1",
    "distribution": {
      "tlp": { "label": "CLEAR" }
    },
    "publisher": {
      "category": "other",
      "name": "Test",
      "namespace": "https://example.com"
    },
    "title": "Test Document",
    "tracking": {
      "current_release_date": "2024-01-01T00:00:00.000Z",
      "id": "TEST-001",
      "initial_release_date": "2024-01-01T00:00:00.000Z",
      "revision_history": [
        {
          "date": "2024-01-01T00:00:00.000Z",
          "number": "1",
          "summary": "Initial."
        }
      ],
      "status": "final",
      "version": "1"
    }
  }
}`

func TestValidateCsafBasic(t *testing.T) {
	result, err := csaf_ffi.ValidateCsaf(validCsaf21, "basic")
	if err != nil {
		t.Fatalf("ValidateCsaf returned error: %v", err)
	}

	if result.Version != "2.1" {
		t.Errorf("expected version 2.1, got %s", result.Version)
	}

	if len(result.TestResults) == 0 {
		t.Error("expected at least one test result")
	}
}

func TestValidateCsaf21(t *testing.T) {
	result, err := csaf_ffi.ValidateCsaf21(validCsaf21, "basic")
	if err != nil {
		t.Fatalf("ValidateCsaf21 returned error: %v", err)
	}

	if result.Version != "2.1" {
		t.Errorf("expected version 2.1, got %s", result.Version)
	}
}

func TestValidateCsafInvalidJson(t *testing.T) {
	_, err := csaf_ffi.ValidateCsaf("not json", "basic")
	if err == nil {
		t.Fatal("expected error for invalid JSON")
	}
}

func TestValidateCsafMissingVersion(t *testing.T) {
	_, err := csaf_ffi.ValidateCsaf(`{"document": {}}`, "basic")
	if err == nil {
		t.Fatal("expected error for missing version")
	}
}

func TestValidateCsafUnsupportedVersion(t *testing.T) {
	_, err := csaf_ffi.ValidateCsaf(`{"document": {"csaf_version": "1.0"}}`, "basic")
	if err == nil {
		t.Fatal("expected error for unsupported version")
	}
}

func TestValidationResultStructure(t *testing.T) {
	result, err := csaf_ffi.ValidateCsaf(validCsaf21, "basic")
	if err != nil {
		t.Fatalf("ValidateCsaf returned error: %v", err)
	}

	// Check that test results contain expected status types
	for _, tr := range result.TestResults {
		if tr.TestId == "" {
			t.Error("test result has empty test_id")
		}

		switch tr.Status.(type) {
		case csaf_ffi.TestResultStatusSuccess:
			// ok
		case csaf_ffi.TestResultStatusFailure:
			// ok
		case csaf_ffi.TestResultStatusNotFound:
			// ok
		case csaf_ffi.TestResultStatusSkipped:
			// ok
		default:
			t.Errorf("unexpected status type for test %s: %T", tr.TestId, tr.Status)
		}
	}
}

// -- CsafDocument tests ----------------------------------------------------

func TestDocumentFromJson21(t *testing.T) {
	doc, err := csaf_ffi.CsafDocumentFromJson(validCsaf21)
	if err != nil {
		t.Fatalf("CsafDocumentFromJson error: %v", err)
	}
	defer doc.Destroy()

	if doc.GetVersionString() != "2.1" {
		t.Errorf("expected version 2.1, got %s", doc.GetVersionString())
	}
	v := doc.GetVersion()
	if v != csaf_ffi.CsafVersionV21 {
		t.Errorf("expected CsafVersionV21, got %v", v)
	}
}

func TestDocumentValidate(t *testing.T) {
	doc, err := csaf_ffi.CsafDocumentFromJson(validCsaf21)
	if err != nil {
		t.Fatalf("error: %v", err)
	}
	defer doc.Destroy()

	result, err := doc.Validate("basic")
	if err != nil {
		t.Fatalf("Validate error: %v", err)
	}
	if len(result.TestResults) == 0 {
		t.Error("expected at least one test result")
	}
}

func TestDocumentRunTest(t *testing.T) {
	doc, err := csaf_ffi.CsafDocumentFromJson(validCsaf21)
	if err != nil {
		t.Fatalf("error: %v", err)
	}
	defer doc.Destroy()

	tr, err := doc.RunTest("6.1.1")
	if err != nil {
		t.Fatalf("RunTest error: %v", err)
	}
	if tr.TestId != "6.1.1" {
		t.Errorf("expected test_id 6.1.1, got %s", tr.TestId)
	}
}

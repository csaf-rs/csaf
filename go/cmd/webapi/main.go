package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"sort"

	"github.com/csaf-rs/csaf/go/csaf_ffi"
)

type errorResponse struct {
	Error string `json:"error"`
}

type testInPresetResponse struct {
	Name   string `json:"name"`
	Preset string `json:"preset"`
}

type legacyValidateResponse struct {
	IsValid bool               `json:"isValid"`
	Tests   []legacyTestResult `json:"tests"`
}

type legacyTestResult struct {
	Name     string          `json:"name"`
	IsValid  bool            `json:"isValid"`
	Errors   []legacyFinding `json:"errors"`
	Warnings []legacyFinding `json:"warnings"`
	Infos    []legacyFinding `json:"infos"`
}

type legacyFinding struct {
	InstancePath string `json:"instancePath"`
	Message      string `json:"message,omitempty"`
}

type validateRequest struct {
	Tests    *[]testOrPreset `json:"tests"`
	Document json.RawMessage `json:"document"`
}

type testOrPreset struct {
	Type string `json:"type"`
	Name string `json:"name"`
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(v)
}

func writeError(w http.ResponseWriter, status int, msg string) {
	writeJSON(w, status, errorResponse{Error: msg})
}

func validateByRequest(body []byte) (legacyValidateResponse, error) {
	var request validateRequest
	if err := json.Unmarshal(body, &request); err != nil {
		return legacyValidateResponse{}, fmt.Errorf("invalid JSON: %w", err)
	}
	if len(request.Document) == 0 {
		return legacyValidateResponse{}, fmt.Errorf("missing 'document' field")
	}
	if request.Tests == nil {
		return legacyValidateResponse{}, fmt.Errorf("missing 'tests' field")
	}

	documentJSON, err := csafDocumentJSON(request.Document)
	if err != nil {
		return legacyValidateResponse{}, err
	}

	doc, err := csaf_ffi.CsafDocumentFromJson(string(documentJSON))
	if err != nil {
		return legacyValidateResponse{}, err
	}
	defer doc.Destroy()

	testIDs, err := resolveTestIDs(doc.GetVersionString(), *request.Tests)
	if err != nil {
		return legacyValidateResponse{}, err
	}

	result, err := doc.RunTests(testIDs)
	if err != nil {
		return legacyValidateResponse{}, err
	}
	return toLegacyValidateResponse(result), nil
}

func csafDocumentJSON(document json.RawMessage) ([]byte, error) {
	var value map[string]json.RawMessage
	if err := json.Unmarshal(document, &value); err != nil {
		return nil, fmt.Errorf("invalid 'document' field: %w", err)
	}
	if _, ok := value["document"]; ok {
		return document, nil
	}
	return json.Marshal(map[string]json.RawMessage{"document": document})
}

func resolveTestIDs(version string, entries []testOrPreset) ([]string, error) {
	testIDs := []string{}
	for _, entry := range entries {
		switch entry.Type {
		case "test":
			testIDs = append(testIDs, entry.Name)
		case "preset":
			presetTests, err := csaf_ffi.GetTestsInPreset(version, entry.Name)
			if err != nil {
				return nil, err
			}
			testIDs = append(testIDs, presetTests...)
		default:
			return nil, fmt.Errorf("invalid test entry type %q", entry.Type)
		}
	}
	if len(testIDs) == 0 {
		testIDs = append(testIDs, "schema")
	}

	sort.Strings(testIDs)
	deduped := testIDs[:0]
	for _, testID := range testIDs {
		if len(deduped) == 0 || deduped[len(deduped)-1] != testID {
			deduped = append(deduped, testID)
		}
	}
	return deduped, nil
}

func toLegacyValidateResponse(result csaf_ffi.ValidationResult) legacyValidateResponse {
	tests := make([]legacyTestResult, 0, len(result.TestResults))
	for _, testResult := range result.TestResults {
		isValid := true
		errors := []legacyFinding{}
		warnings := []legacyFinding{}
		infos := []legacyFinding{}

		if failure, ok := testResult.Status.(csaf_ffi.TestResultStatusFailure); ok {
			isValid = false
			errors = toLegacyFindings(failure.Errors)
			warnings = toLegacyFindings(failure.Warnings)
			infos = toLegacyFindings(failure.Infos)
		}

		tests = append(tests, legacyTestResult{
			Name:     testResult.TestId,
			IsValid:  isValid,
			Errors:   errors,
			Warnings: warnings,
			Infos:    infos,
		})
	}

	return legacyValidateResponse{
		IsValid: result.Success,
		Tests:   tests,
	}
}

func toLegacyFindings(findings []csaf_ffi.ValidationError) []legacyFinding {
	result := make([]legacyFinding, len(findings))
	for i, finding := range findings {
		result[i] = legacyFinding{
			InstancePath: finding.InstancePath,
			Message:      finding.Message,
		}
	}
	return result
}

func versionParam(r *http.Request) string {
	if v := r.URL.Query().Get("version"); v != "" {
		return v
	}
	return "2.0"
}

// GET /api/v1/tests — available tests with their primary preset
func handleGetTests(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		writeError(w, http.StatusMethodNotAllowed, "GET required")
		return
	}

	tests, err := csaf_ffi.GetTests(versionParam(r))
	if err != nil {
		writeError(w, http.StatusNotFound, err.Error())
		return
	}

	response := make([]testInPresetResponse, len(tests))
	for i, test := range tests {
		response[i] = testInPresetResponse{
			Name:   test.Name,
			Preset: test.Preset,
		}
	}
	writeJSON(w, http.StatusOK, response)
}

// POST /api/v1/validate — raw JSON body
func handleValidateJSON(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		writeError(w, http.StatusMethodNotAllowed, "POST required")
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 50<<20)

	data, err := io.ReadAll(r.Body)
	if err != nil {
		writeError(w, http.StatusBadRequest, "failed to read request body: "+err.Error())
		return
	}
	if len(data) == 0 {
		writeError(w, http.StatusBadRequest, "empty request body")
		return
	}

	result, err := validateByRequest(data)
	if err != nil {
		writeError(w, http.StatusBadRequest, err.Error())
		return
	}
	writeJSON(w, http.StatusOK, result)
}

// CORS middleware
func corsMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func main() {
	addr := ":8084"
	if port := os.Getenv("PORT"); port != "" {
		addr = ":" + port
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/tests", handleGetTests)
	mux.HandleFunc("/api/v1/validate", handleValidateJSON)

	handler := corsMiddleware(mux)

	fmt.Printf("CSAF Validation API listening on %s\n", addr)
	fmt.Println("  GET  /api/v1/tests       — available tests")
	fmt.Println("  POST /api/v1/validate    — validation request body")
	fmt.Println("  Query params: ?version=2.0|2.1")

	if err := http.ListenAndServe(addr, handler); err != nil {
		log.Fatal(err)
	}
}

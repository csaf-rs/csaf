package csaf_ffi

import "encoding/json"

// json.Marshaler implementations for FFI types.
// Produces camelCase keys matching the WASM TypeScript bindings so that
// both the Go API and the WASM demo share the same JSON schema.

func (r ValidationResult) MarshalJSON() ([]byte, error) {
	testResults := r.TestResults
	if testResults == nil {
		testResults = []TestResult{}
	}
	return json.Marshal(struct {
		Success     bool         `json:"success"`
		Version     string       `json:"version"`
		TestResults []TestResult `json:"testResults"`
		NumErrors   uint64       `json:"numErrors"`
		NumWarnings uint64       `json:"numWarnings"`
		NumInfos    uint64       `json:"numInfos"`
		NumNotFound uint64       `json:"numNotFound"`
	}{r.Success, r.Version, testResults, r.NumErrors, r.NumWarnings, r.NumInfos, r.NumNotFound})
}

func (r TestResult) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		TestID string           `json:"testId"`
		Status TestResultStatus `json:"status"`
	}{r.TestId, r.Status})
}

func (e ValidationError) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		Message      string `json:"message"`
		InstancePath string `json:"instancePath"`
	}{e.Message, e.InstancePath})
}

func (TestResultStatusSuccess) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		Tag string `json:"tag"`
	}{"Success"})
}

func (s TestResultStatusFailure) MarshalJSON() ([]byte, error) {
	errors := s.Errors
	if errors == nil {
		errors = []ValidationError{}
	}
	warnings := s.Warnings
	if warnings == nil {
		warnings = []ValidationError{}
	}
	infos := s.Infos
	if infos == nil {
		infos = []ValidationError{}
	}
	return json.Marshal(struct {
		Tag      string            `json:"tag"`
		Errors   []ValidationError `json:"errors"`
		Warnings []ValidationError `json:"warnings"`
		Infos    []ValidationError `json:"infos"`
	}{"Failure", errors, warnings, infos})
}

func (TestResultStatusNotFound) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		Tag string `json:"tag"`
	}{"NotFound"})
}

func (TestResultStatusSkipped) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		Tag string `json:"tag"`
	}{"Skipped"})
}

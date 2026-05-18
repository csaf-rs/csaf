package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/csaf-rs/csaf/go/csaf_ffi"
)

type errorResponse struct {
	Error string `json:"error"`
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(v)
}

func writeError(w http.ResponseWriter, status int, msg string) {
	writeJSON(w, status, errorResponse{Error: msg})
}

func validate(w http.ResponseWriter, jsonStr string, preset string) {
	result, err := csaf_ffi.ValidateCsaf(jsonStr, preset)
	if err != nil {
		writeError(w, http.StatusUnprocessableEntity, err.Error())
		return
	}
	writeJSON(w, http.StatusOK, result)
}

func presetParam(r *http.Request) string {
	if p := r.URL.Query().Get("preset"); p != "" {
		return p
	}
	return "basic"
}

// POST /api/validate/upload — multipart file upload
func handleValidateUpload(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		writeError(w, http.StatusMethodNotAllowed, "POST required")
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 50<<20)

	file, _, err := r.FormFile("file")
	if err != nil {
		writeError(w, http.StatusBadRequest, "missing or invalid 'file' field: "+err.Error())
		return
	}
	defer file.Close()

	data, err := io.ReadAll(file)
	if err != nil {
		writeError(w, http.StatusBadRequest, "failed to read file: "+err.Error())
		return
	}

	validate(w, string(data), presetParam(r))
}

// POST /api/validate/json — raw JSON body
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

	validate(w, string(data), presetParam(r))
}

// CORS middleware
func corsMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "POST, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusNoContent)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func main() {
	addr := ":8080"
	if port := os.Getenv("PORT"); port != "" {
		addr = ":" + port
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/api/validate/upload", handleValidateUpload)
	mux.HandleFunc("/api/validate/json", handleValidateJSON)

	handler := corsMiddleware(mux)

	fmt.Printf("CSAF Validation API listening on %s\n", addr)
	fmt.Println("  POST /api/validate/upload  — multipart file upload (field: 'file')")
	fmt.Println("  POST /api/validate/json    — raw JSON body")
	fmt.Println("  Query param: ?preset=basic|extended|full")

	if err := http.ListenAndServe(addr, handler); err != nil {
		log.Fatal(err)
	}
}

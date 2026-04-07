// download-libs downloads the pre-built libcsaf_ffi.a for the current
// platform from the latest GitHub release of csaf-rs/csaf and places it
// under <out-dir>/lib/<GOOS>_<GOARCH>/libcsaf_ffi.a so that the per-platform
// #cgo LDFLAGS directives in go/csaf_ffi/cgo_*.go can find it without any
// CGO_LDFLAGS environment variable.
//
// Usage (via go generate from go/csaf_ffi/):
//
//	go generate ./csaf_ffi/
//
// Or directly:
//
//	go run ./cmd/download-libs [-out-dir <path>] [-version <tag>] [-repo owner/name]
package main

import (
	"archive/zip"
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
	"strings"
)

const defaultRepo = "csaf-rs/csaf"

// rustTarget maps (GOOS, GOARCH) to the Rust target triple used in release
// asset names.
var rustTarget = map[[2]string]string{
	{"linux", "amd64"}:  "x86_64-unknown-linux-gnu",
	{"linux", "arm64"}:  "aarch64-unknown-linux-gnu",
	{"darwin", "amd64"}: "x86_64-apple-darwin",
	{"darwin", "arm64"}: "aarch64-apple-darwin",
}

type releaseAsset struct {
	Name               string `json:"name"`
	BrowserDownloadURL string `json:"browser_download_url"`
}

type release struct {
	TagName string         `json:"tag_name"`
	Assets  []releaseAsset `json:"assets"`
}

func main() {
	home, _ := os.UserHomeDir()
	defaultOut := filepath.Join(home, ".cache", "csaf-ffi")

	outDir := flag.String("out-dir", defaultOut, "directory for lib/<os>_<arch>/libcsaf_ffi.a")
	system := flag.Bool("system", false, "install to /usr/local/lib (may require sudo)")
	version := flag.String("version", "", "release tag to download (default: latest)")
	repo := flag.String("repo", defaultRepo, "GitHub repository owner/name")
	flag.Parse()

	goos := runtime.GOOS
	goarch := runtime.GOARCH

	target, ok := rustTarget[[2]string{goos, goarch}]
	if !ok {
		fatalf("unsupported platform %s/%s — pre-built library not available\n"+
			"Build from source: cargo build --release -p csaf-ffi\n"+
			"Then copy target/release/libcsaf_ffi.a to %s/lib/%s_%s/",
			goos, goarch, *outDir, goos, goarch)
	}

	rel, err := fetchRelease(*repo, *version)
	if err != nil {
		fatalf("fetch release: %v", err)
	}
	fmt.Printf("Downloading libcsaf_ffi for %s/%s from release %s…\n", goos, goarch, rel.TagName)

	asset := findAsset(rel.Assets, target)
	if asset == nil {
		fatalf("no asset found for target %q in release %s\n"+
			"Available assets:\n%s",
			target, rel.TagName, listAssets(rel.Assets))
	}

	data, err := downloadBytes(asset.BrowserDownloadURL)
	if err != nil {
		fatalf("download %s: %v", asset.Name, err)
	}

	if *system {
		// Write directly to /usr/local/lib/libcsaf_ffi.a (no subdirectory needed)
		dest := "/usr/local/lib/libcsaf_ffi.a"
		if err := extractFromZip(data, "libcsaf_ffi.a", dest); err != nil {
			fatalf("extract: %v", err)
		}
		fmt.Printf("✓ Installed to %s\n", dest)
		fmt.Println("  go build will find it automatically.")
	} else {
		// Existing behavior: write to outDir/lib/<os>_<arch>/
		destDir := filepath.Join(*outDir, "lib", goos+"_"+goarch)
		os.MkdirAll(destDir, 0o755)
		dest := filepath.Join(destDir, "libcsaf_ffi.a")
		extractFromZip(data, "libcsaf_ffi.a", dest)
		fmt.Printf("✓ Written to %s\n", dest)
		fmt.Printf("  Build with: CGO_LDFLAGS=\"-L%s\" go build ./...\n", destDir)
	}
}

func fetchRelease(repo, version string) (*release, error) {
	var url string
	if version == "" {
		url = fmt.Sprintf("https://api.github.com/repos/%s/releases/latest", repo)
	} else {
		url = fmt.Sprintf("https://api.github.com/repos/%s/releases/tags/%s", repo, version)
	}

	req, _ := http.NewRequest(http.MethodGet, url, nil)
	req.Header.Set("Accept", "application/vnd.github+json")
	req.Header.Set("X-GitHub-Api-Version", "2022-11-28")
	if token := os.Getenv("GITHUB_TOKEN"); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return nil, fmt.Errorf("GitHub API %s: %s", resp.Status, bytes.TrimSpace(body))
	}

	var rel release
	if err := json.NewDecoder(resp.Body).Decode(&rel); err != nil {
		return nil, err
	}
	return &rel, nil
}

// findAsset picks the first release asset whose name contains the Rust target
// triple and ends in .zip. There may be two (MSRV + stable) — either works.
func findAsset(assets []releaseAsset, target string) *releaseAsset {
	for i := range assets {
		a := &assets[i]
		if strings.Contains(a.Name, target) && strings.HasSuffix(a.Name, ".zip") {
			return a
		}
	}
	return nil
}

func listAssets(assets []releaseAsset) string {
	var sb strings.Builder
	for _, a := range assets {
		fmt.Fprintf(&sb, "  %s\n", a.Name)
	}
	return sb.String()
}

func downloadBytes(url string) ([]byte, error) {
	resp, err := http.Get(url) //nolint:gosec
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("HTTP %s", resp.Status)
	}
	return io.ReadAll(resp.Body)
}

func extractFromZip(data []byte, filename, dest string) error {
	r, err := zip.NewReader(bytes.NewReader(data), int64(len(data)))
	if err != nil {
		return err
	}
	for _, f := range r.File {
		if filepath.Base(f.Name) != filename {
			continue
		}
		rc, err := f.Open()
		if err != nil {
			return err
		}
		defer rc.Close()

		out, err := os.Create(dest)
		if err != nil {
			return err
		}
		defer out.Close()

		_, err = io.Copy(out, rc)
		return err
	}
	return fmt.Errorf("%q not found in zip archive", filename)
}

func fatalf(format string, args ...any) {
	fmt.Fprintf(os.Stderr, "download-libs: "+format+"\n", args...)
	os.Exit(1)
}

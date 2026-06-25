# csaf-rust [![build](https://github.com/csaf-rs/csaf/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/csaf-rs/csaf/actions/workflows/build.yml) [![CVSS](https://api.devguard.org/api/v1/badges/cvss/06c84617-8925-4cfa-af31-f609ebce390c)](https://api.devguard.org/api/v1/badges/cvss/06c84617-8925-4cfa-af31-f609ebce390c)

This repository is a reference implementation for the CSAF standard in Rust that relies on automatically generating CSAF document structs from the JSON schema.

This is work-in-progress.

## Repository structure

- `csaf-validator` contains a command line tool to validate CSAF documents.
- `csaf-rs` contains the actual validator library which currently publishes a crate to [crates.io](https://crates.io/crates/csaf-rs).
- `csaf-ffi` contains [UniFFI](https://github.com/mozilla/uniffi-rs) bindings that expose `csaf-rs` to other languages (Go, WASM/TypeScript, and more).
- `csaf-service` contains a web API that exposes the validation functionality of `csaf-rs` via HTTP endpoints.
- `go/` contains generated Go bindings and integration tests.
- `wasm/` contains generated WASM/TypeScript bindings and integration tests.

## Minimum required Rust version (MSRV)

1.88.0


## Usage 

After [building](README.md#build) or downloading `csaf-validator` from [the available releases](https://github.com/csaf-rs/csaf/releases), the usage is quite simple and additional help can be display using `--help`.

```
A validator for CSAF documents

Usage: csaf-validator [OPTIONS] <PATH>

Arguments:
  <PATH>  

Options:
  -c, --csaf-version <CSAF_VERSION>  Version of CSAF to use [default: 2.0]
  -p, --preset <PRESET>              The validation preset to use [default: basic]
  -t, --test-id <TEST_ID>            Run only the selected tests, may be specified multiple times
  -h, --help                         Print help
  -V, --version                      Print version
```

Some examples to use are included below. Please note that the validation is not yet fully implemented!

```bash
# validate a CSAF 2.0 document with profile basic (the default)
csaf-validator --csaf-version 2.0 my-csaf-2-0-document.json

# validate a CSAF 2.0 document with profile full
csaf-validator --csaf-version 2.0 --preset full my-csaf-2-0-document.json

# validate a CSAF 2.1 document with one specific test
csaf-validator --csaf-version 2.1 --test-id 6.1.34 my-csaf-2-1-document.json
```

You can also use the library version as depicted here:
```rust
use csaf::csaf2_0::loader::load_document;
use csaf::schema::csaf2_0::schema::CommonSecurityAdvisoryFramework;
use csaf::validation::{Validatable, validate_by_preset, validate_by_test, validate_by_tests};

let csaf_version = "2.0";
let path = "/path/to/local/cve-2025-9820.json";

// validate a preset
let preset_results = validate_by_preset(&document, csaf_version, "basic");

// validate multiple tests
let multiple_tests_result = validate_by_tests(&document, csaf_version, &["6.1.1", "6.1.10", "6.1.20"]);

// validate a single test
let single_test_result = validate_by_test(&document, "6.1.13");

// get all test ids from a preset
let test_ids_in_basic_preset = CommonSecurityAdvisoryFramework::tests_in_preset("basic");
```

### Go 

To use this library you have to download the binaries for your specific operating system and platform. A download script is provided to help you with that.

You can either install systemwide (`sudo` may be needed)
```go
go run github.com/csaf-rs/csaf/go/cmd/download-libs --system
go build ./...
```
Or without root permissions, but you have to tell the linker where to find the artifacts:
```go
go run github.com/csaf-rs/csaf/go/cmd/download-libs   # one-time, writes to ~/.cache/csaf-ffi/
CGO_LDFLAGS="-L$HOME/.cache/csaf-ffi/lib/$(go env GOOS)_$(go env GOARCH)" go build ./...
```
Alternatively you can put the `CGO_LDFLAGS` export in your shell profile.

Then add it to your project with
```bash
go get github.com/csaf-rs/csaf/go
```
or directly to your `go.mod` file with
```bash
require github.com/csaf-rs/csaf/go
```

You can then import the necessary package into your code like this
```go
import (
	"github.com/csaf-rs/csaf/go/csaf_ffi"
)
```
The easiest way to validate a document is to use the generic `ValidateCsaf` function, which takes the document and a preset to test against.
```go
result, err := csaf_ffi.ValidateCsaf(string(data), preset)
```

## Versioning

To bump the version across all crates and `wasm/package.json` at once, use the provided script. It requires [`cargo-edit`](https://crates.io/crates/cargo-edit) and `npm`.

```bash
# Install cargo-edit (one-time)
cargo install cargo-edit

./bump_version.sh patch        # x.y.z -> x.y.z+1
./bump_version.sh minor        # x.y.z -> x.y+1.0
./bump_version.sh major        # x.y.z -> x+1.0.0
./bump_version.sh set 1.2.3    # explicit version
```

## Build

If you want to build `csaf-validator` on your own, please install Rust (see https://rustup.rs) and then run

```bash
# make sure submodules are up-to-date
git submodule init
git submodule update --remote

# make sure that local assets are in sync with git submodules
./update_assets.sh

# run the tests
cargo test

# build for release
cargo build --release
```

The final binary will be in `target/release` and can then be installed, for example, in a system-wide folder.

### Building for other languages

Bindings for other languages are created through the `csaf-ffi` crate, which is built by the language specific *generate_XXX_bindings* script.

#### Go

The Go bindings are generated via [uniffi-bindgen-go](https://github.com/NordSecurity/uniffi-bindgen-go).

```bash
# Install the Go binding generator (one-time)
cargo install uniffi-bindgen-go \
  --git https://github.com/NordSecurity/uniffi-bindgen-go \
  --tag v0.7.0+v0.31.0

# Build the Rust library, generate bindings, and copy the static archive
./generate_go_bindings.sh
```
This creates GO code in the `go/csaf_ffi` folder.

To run the Go tests:

```bash
cd go
go test -v ./csaf_ffi/
```

As a demonstration there is a small CLI and Webserver example included.

##### Cli

```bash
cd go
CGO_LDFLAGS="-L$HOME/.cache/csaf_ffi/lib/$(go env GOOS)_$(go env GOARCH)" go run -buildvcs=false ./cmd/example/ <PATH_TO_CSAF_FILE>
```

##### Web server (API)

```bash
cd go
CGO_LDFLAGS="-L$HOME/.cache/csaf_ffi/lib/$(go env GOOS)_$(go env GOARCH)" go run -buildvcs=false ./cmd/webapi/
```

The server listens on port `8082` by default. Set the `PORT` environment variable
to use a different port:

```bash
PORT=9090 go run -buildvcs=false ./cmd/webapi/
```

*Endpoints*

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/v1/tests` | Retrieve available tests with their primary preset |
| `POST` | `/api/v1/validate` | Validate a CSAF document sent as a raw JSON body |

- `GET /api/v1/tests` accepts the optional query parameter `?version=2.0` (default) or `?version=2.1`.
- `POST /api/v1/validate` accepts a JSON body with `tests` and a CSAF `document` metadata object.

*Example:*

```bash
curl -X POST http://localhost:8082/api/v1/validate \
  -H 'Content-Type: application/json' \
  --data-binary '{
    "tests": [{ "type": "preset", "name": "basic" }],
    "document": {
      "category": "csaf_base",
      "csaf_version": "2.0"
    }
  }'
```

#### WASM

The WASM bindings are generated via [uniffi-bindgen-js](https://crates.io/crates/uniffi-bindgen-js).

```bash
# Install the WASM binding generator (one-time)
cargo install uniffi-bindgen-js --version 0.2.1

# Build and generate (uses generate_wasm_bindings.sh)
./generate_wasm_bindings.sh
```

This creates TypeScript + WASM output in `wasm/`. 


## Docker

You can also run the service via Docker. To build and run the container, use the following commands:
```bash
docker build -t csaf-service .
docker run -p 8082:8082 csaf-service
```
> Note: This is intended for development and testing purposes only, as the container is not hardened for production use.

You can then access the service at `http://localhost:8082` and navigate to `/swagger-ui` for the API documentation.
For further configuration options, please refer to the [csaf-service README](csaf-service/README.md).


## Implementation status in regards to the Standard

* ✅ Implemented
* ⭕ Not applicable

### Mandatory Tests

| Test specification | 2.0 | 2.1 (experimental) |
| --- | --- | -- |
| 6.1.1 | ✅ | ✅ |
| 6.1.2 | ✅ | ✅ |
| 6.1.3 | ✅ | ✅ |
| 6.1.4 | ✅ | ✅ |
| 6.1.5 | ✅ | ✅ |
| 6.1.6 | ✅ | ✅ |
| 6.1.7 | ✅ |   |
| 6.1.8 | ✅ |   |
| 6.1.9 | ✅  | ✅ |
| 6.1.10 | ✅ | ✅ |
| 6.1.11 | ✅ |   |
| 6.1.12 | ✅ | ✅ |
| 6.1.13 | ✅ | ✅ |
| 6.1.14 | ✅ |   |
| 6.1.15 | ✅ | ✅ |
| 6.1.16 | ✅ | ✅ |
| 6.1.17 | ✅ | ✅ |
| 6.1.18 | ✅ | ✅ |
| 6.1.19 | ✅ | ✅ |
| 6.1.20 | ✅ | ✅ |
| 6.1.21 | ✅ | ✅ |
| 6.1.22 | ✅ | ✅ |
| 6.1.23 | ✅ | ✅ |
| 6.1.24 | ✅ | ✅ |
| 6.1.25 | ✅ | ✅ |
| 6.1.26 | ✅ | ✅ |
| 6.1.27.1 | ✅ | ✅ |
| 6.1.27.2 | ✅ | ✅ |
| 6.1.27.3 | ✅ | ✅ |
| 6.1.27.4 | ✅ | ✅ |
| 6.1.27.5 | ✅ | ✅ |
| 6.1.27.6 | ✅ | ✅ |
| 6.1.27.7 | ✅ | ✅ |
| 6.1.27.8 | ✅ | ✅ |
| 6.1.27.9 | ✅ | ✅ |
| 6.1.27.10 | ✅ | ✅ |
| 6.1.27.11 | ✅ | ✅ |
| 6.1.27.12 | ⭕ | ✅ |
| 6.1.27.13 | ⭕ |   |
| 6.1.27.14 | ⭕ | ✅ |
| 6.1.27.15 | ⭕ | ✅ |
| 6.1.27.16 | ⭕ | ✅ |
| 6.1.27.17 | ⭕ | ✅ |
| 6.1.27.18 | ⭕ | ✅ |
| 6.1.27.19 | ⭕ | ✅ |
| 6.1.28 | ✅ | ✅ |
| 6.1.29 | ✅ | ✅ |
| 6.1.30 | ✅ | ✅ |
| 6.1.31 | ✅ | ✅ |
| 6.1.32 | ✅ | ✅ |
| 6.1.33 | ✅ |   |
| 6.1.34 | ⭕ | ✅ |
| 6.1.35 | ⭕ | ✅ |
| 6.1.36 | ⭕ |   |
| 6.1.37 | ⭕ |   |
| 6.1.38 | ⭕ | ✅ |
| 6.1.39 | ⭕ | ✅ |
| 6.1.40 | ⭕ | ✅ |
| 6.1.41 | ⭕ | ✅ |
| 6.1.42 | ⭕ | ✅ |
| 6.1.43 | ⭕ | ✅ |
| 6.1.44 | ⭕ | ✅ |
| 6.1.45 | ⭕ |   |
| 6.1.46 | ⭕ | ✅ |
| 6.1.47 | ⭕ | ✅ |
| 6.1.48 | ⭕ |   |
| 6.1.49 | ⭕ |   |
| 6.1.50 | ⭕ |   |
| 6.1.51 | ⭕ |   |
| 6.1.52 | ⭕ |   |
| 6.1.53 | ⭕ | ✅ |
| 6.1.54 | ⭕ | ✅ |
| 6.1.55 | ⭕ | ✅ |
| 6.1.56 | ⭕ | ✅ |
| 6.1.57 | ⭕ | ✅ |
| 6.1.58 | ⭕ | ✅ |
| 6.1.59 | ⭕ |   |
| 6.1.60.1 | ⭕  |   |
| 6.1.60.2 | ⭕  |   |
| 6.1.60.3 | ⭕  |   |
| 6.1.61 | ⭕ | ✅ |

### Recommended Tests

| Test specification | 2.0                | 2.1 (experimental) |
| --- |--------------------|--------------------|
| 6.2.1 |  |  |

### Informative Tests

| Test specification | 2.0                | 2.1 (experimental) |
| --- |--------------------|--------------------|
| 6.3.1 |  |  |

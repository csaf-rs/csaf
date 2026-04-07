# csaf-rust [![build](https://github.com/csaf-rs/csaf/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/csaf-rs/csaf/actions/workflows/build.yml) [![CVSS](https://api.devguard.org/api/v1/badges/cvss/06c84617-8925-4cfa-af31-f609ebce390c)](https://api.devguard.org/api/v1/badges/cvss/06c84617-8925-4cfa-af31-f609ebce390c)

This repository is a reference implementation for the CSAF standard in Rust that relies on automatically generating CSAF document structs from the JSON schema.

This is work-in-progress.

## Structure

- `csaf-validator` contains a command line tool to validate CSAF documents.
- `csaf-rs` contains the actual validator library which currently publishes a crate to [crates.io](https://crates.io/crates/csaf-rs). In the future there will be a WASM version published to NPM and also bindings to other languages via [UniFFI](https://github.com/mozilla/uniffi-rs).

## Minimum required Rust version (MSRV)

1.88.0

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

## Build WASM Bindings

If you want to build the WASM bindings locally, install `wasm-pack` (make sure `~/.cargo/bin` is in
the path) and execute it:
```bash
cargo install wasm-pack
wasm-pack build csaf-rs --scope csaf-rs
```

This will create a JavaScript/TypeScript package in `csaf-rs/pkg`.

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


## Implementation status in regards to the Standard

* :white_check_mark: Implemented
* :o: Not applicable

### Mandatory Tests

| Test specification | 2.0                | 2.1 (experimental) |
| --- |--------------------|--------------------|
| 6.1.1 | :white_check_mark: | :white_check_mark: |
| 6.1.2 | :white_check_mark: | :white_check_mark: |
| 6.1.3 | :white_check_mark: | :white_check_mark: |
| 6.1.4 | :white_check_mark: | :white_check_mark: |
| 6.1.5 | :white_check_mark: | :white_check_mark: |
| 6.1.6 | :white_check_mark: | :white_check_mark: |
| 6.1.7 | :white_check_mark: |                    |
| 6.1.8 | :white_check_mark: |                    |
| 6.1.9 | :white_check_mark:  | :white_check_mark: |
| 6.1.10 | :white_check_mark: | :white_check_mark: |
| 6.1.11 | :white_check_mark: |                    |
| 6.1.12 | :white_check_mark: | :white_check_mark: |
| 6.1.13 | :white_check_mark: | :white_check_mark: |
| 6.1.14 | :white_check_mark: |                    |
| 6.1.15 | :white_check_mark: | :white_check_mark: |
| 6.1.16 | :white_check_mark: | :white_check_mark: |
| 6.1.17 | :white_check_mark: | :white_check_mark: |
| 6.1.18 | :white_check_mark: | :white_check_mark: |
| 6.1.19 | :white_check_mark: | :white_check_mark: |
| 6.1.20 | :white_check_mark: | :white_check_mark: |
| 6.1.21 | :white_check_mark: | :white_check_mark: |
| 6.1.22 | :white_check_mark: | :white_check_mark: |
| 6.1.23 | :white_check_mark: | :white_check_mark: |
| 6.1.24 | :white_check_mark: | :white_check_mark: |
| 6.1.25 | :white_check_mark: | :white_check_mark: |
| 6.1.26 | :white_check_mark: | :white_check_mark: |
| 6.1.27.1 | :white_check_mark: | :white_check_mark: |
| 6.1.27.2 | :white_check_mark: | :white_check_mark: |
| 6.1.27.3 | :white_check_mark: | :white_check_mark: |
| 6.1.27.4 | :white_check_mark: | :white_check_mark: |
| 6.1.27.5 | :white_check_mark: | :white_check_mark: |
| 6.1.27.6 | :white_check_mark: | :white_check_mark: |
| 6.1.27.7 | :white_check_mark: | :white_check_mark: |
| 6.1.27.8 | :white_check_mark: | :white_check_mark: |
| 6.1.27.9 | :white_check_mark: | :white_check_mark: |
| 6.1.27.10 | :white_check_mark: | :white_check_mark: |
| 6.1.27.11 | :white_check_mark: | :white_check_mark: |
| 6.1.27.12 | :o:                | :white_check_mark: |
| 6.1.27.13 | :o:                |                    |
| 6.1.27.14 | :o:                | :white_check_mark: |
| 6.1.27.15 | :o:                | :white_check_mark: |
| 6.1.27.16 | :o:                | :white_check_mark: |
| 6.1.27.17 | :o:                | :white_check_mark: |
| 6.1.27.18 | :o:                | :white_check_mark: |
| 6.1.27.19 | :o:                | :white_check_mark: |
| 6.1.28 | :white_check_mark: | :white_check_mark: |
| 6.1.29 | :white_check_mark: | :white_check_mark: |
| 6.1.30 | :white_check_mark: | :white_check_mark: |
| 6.1.31 | :white_check_mark: | :white_check_mark: |
| 6.1.32 | :white_check_mark: | :white_check_mark: |
| 6.1.33 | :white_check_mark: |                    |
| 6.1.34 | :o:                | :white_check_mark: |
| 6.1.35 | :o:                | :white_check_mark: |
| 6.1.36 | :o:                |                    |
| 6.1.37 | :o:                |                    |
| 6.1.38 | :o:                | :white_check_mark: |
| 6.1.39 | :o:                | :white_check_mark: |
| 6.1.40 | :o:                | :white_check_mark: |
| 6.1.41 | :o:                | :white_check_mark: |
| 6.1.42 | :o:                | :white_check_mark: |
| 6.1.43 | :o:                | :white_check_mark: |
| 6.1.44 | :o:                | :white_check_mark: |
| 6.1.45 | :o:                |                    |
| 6.1.46 | :o:                |                    |
| 6.1.47 | :o:                |                    |
| 6.1.48 | :o:                |                    |
| 6.1.49 | :o:                |                    |
| 6.1.50 | :o:                |                    |
| 6.1.51 | :o:                |                    |
| 6.1.52 | :o:                |                    |
| 6.1.53 | :o:                | :white_check_mark: |
| 6.1.54 | :o:                | :white_check_mark: |
| 6.1.55 | :o:                |                    |
| 6.1.56 | :o:                | :white_check_mark: |
| 6.1.57 | :o:                | :white_check_mark: |
| 6.1.58 | :o:                | :white_check_mark: |
| 6.1.59 | :o:                |                    |
| 6.1.60.1 | :o:                |                    |
| 6.1.60.2 | :o:                |                    |
| 6.1.60.3 | :o:                |                    |
| 6.1.61 | :o:                | :white_check_mark: |

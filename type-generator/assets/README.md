# type-generator Asset Sources & Licenses

This folder contains assets used by the type-generator to produce generated Rust code for `csaf-rs`.

## git-based Assets

### CSAF RVISC

- `registry.json`

### CSAF Schemas

- `csaf_2.0_json_schema.json`
- `csaf_2.1_json_schema.json`
- `csaf_2.0_testcases_json_schema.json`
- `csaf_2.1_testcases_json_schema.json`

See https://github.com/oasis-tcs/csaf for information regarding these files and their respective licenses.
They are 1:1 copies from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

These files are used to generate the schema and test schema types in `csaf-rs`
via `cargo run -- --schema` and `cargo run -- --test-schema`.

## External Assets

### IANA Language Subtag Registry

- `language-subtag-registry.txt`

Contains IANA's language subtag registry found at
https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry.
Assumed to be public domain material, according to https://www.iana.org/help/licensing-terms.

This file is used to generate `csaf-rs/src/csaf/types/language/language_subtags.generated.rs`
via `cargo run -- --language-tags`.

## Supplementary Test Cases

- `tests/**`

Supplementary test case files for CSAF 2.0 and 2.1, organized under `tests/csaf_2.0/` and `tests/csaf_2.1/`.
These supplementary test cases are maintained by the csaf-rs project. They complement the official CSAF TC test suite
found in the `csaf` submodule. They are intended to be proposed upstream to the CSAF TC for incorporation.

Naming conventions:
- Do not use the OASIS CSAF prefix or namespace.
- Use `s` for the test case number to indicate supplementary tests.
- Use the same counting schema as the original tests (even numbers in the decade indicate invalid cases).
- Highlight the use case of the test in the `document/title` and optionally in the test code.

These test cases are used (along with the upstream test cases) to generate test definitions in `csaf-rs` via
`cargo run -- --test-definitions`.


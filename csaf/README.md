# csaf [![build](https://github.com/csaf-rs/csaf/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/csaf-rs/csaf/actions/workflows/build.yml)

This repository is a reference implementation for the CSAF standard in Rust that relies on automatically generating CSAF document structs from the JSON schema.

This is work-in-progress.

## Usage

### Library

To install the library, add it with
```bash
cargo add csaf
```
or reference directly in your Cargo.toml
```toml
[dependencies]
csaf = "0.6"
```
which has both `converter` and `validator` features active.

```rust
use csaf::{InMemoryDiagnosticCollector, Settings, convert, validate};

let doc = r#"{"document": {"category": "csaf_base","csaf_version": "2.0"}}"#;

// validate the document
let result = validate(doc, "auto", &["basic".to_string()]).unwrap();
println!("Is valid: {:?}", result.success); // false

// convert to a 2.1 document
let mut diagnostic_collector = InMemoryDiagnosticCollector::new();
// allow invalid documents because we know it is invalid
let allow_invalid = true;
let result = convert(doc, &Settings::default(), &mut diagnostic_collector, allow_invalid);
if let Ok(converted) = result {
    println!("{0}", serde_json::to_string(&converted).unwrap());
    // {"$schema":"https://docs.oasis-open.org/csaf/csaf/v2.1/schema/csaf.json","document":{"category":"csaf_base","csaf_version":"2.1","distribution":{"tlp":{"label":"CLEAR"}}}}
}
```

### CLI / Binary

To install as a cli tool
```bash
cargo install csaf --features="cli"
```
Then pass in the document to validate or convert along with the required parameters.
```bash
# validate a file with the basic preset
csaf validate csaf-example.json
# show more details
csaf validate -v csaf-example.json
# use an explicit preset or test with --test or -T (multiple times)
csaf validate -v csaf-example.json --test basic -T 6.2.5 -T 6.3.1

# convert a CSAF 2.0 file and print to command line
csaf convert csaf-example.json
# allow an invalid input file
csaf convert --allow-invalid csaf-example.json
# save the converted file
csaf convert --allow-invalid csaf-example.json -o csaf-example-2-1.json

# show help
csaf --help
# show validator help
csaf validate --help
# show converter help
csaf convert --help
```

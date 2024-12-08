# csaf-rust

This repository is a proof-of-concept for a CSAF library in Rust that relies on automatically generating CSAF document structs from the JSON schema.

This is work-in-progress.

## Build

If you want to build `csaf-validator` on your own, please install Rust (see https://rustup.rs) and then run

```bash
cargo build --release
```

The final binary will be in `target/release` and can then installed for example in a system-wide folder.

## Usage

After building or downloading `csaf-validator`, the usage is quite simple and additional help can be display using `--help`.

```
A validator for CSAF documents

Usage: csaf-validator [OPTIONS] <PATH>

Arguments:
  <PATH>  

Options:
  -c, --csaf-version <CSAF_VERSION>  Version of CSAF to use [default: 2.0]
  -p, --profile <PROFILE>            The profile to use [default: basic]
  -o, --only-test <ONLY_TEST>        Run only the selected test
  -h, --help                         Print help
  -V, --version                      Print version
```

Some examples to use are included below. Please note that the validation is not yet fully implemented!

```
# validate a CSAF 2.0 document with profile basic (the default)
csaf-validator --csaf-version 2.0 my-csaf-2-0-document.json

# validate a CSAF 2.0 document with profile full
csaf-validator --csaf-version 2.0 --profile full my-csaf-2-0-document.json

# validate a CSAF 2.1 document with a specific test
csaf-validator --csaf-version 2.1 --only-test 6.1.34 my-csaf-2-1-document.json
```

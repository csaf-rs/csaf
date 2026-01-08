# Summary

This is a type generator for the csaf-rs library.
It creates rust types from the official CSAF schema.
Optionally you can create types for the testcases schema and a structure for the testcases itself.

# Usage

Create rust types for the CSAF schema
```
cargo run
```

Additionally create types for the testcases.json file
```
cargo run -- --include-test-schema
```

Additionally create types for the testcases itself, to work with them in a type safe manner
```
cargo run -- --create-test-definitions
```

By default the target folder is set to `../csaf-rs` and the generator is creating files the specific subfolder used in this library, but you can override it by passing the `--target-folder` option.

| Type | Default target folder|
| --- | --- |
| CSAF-Schema | `csaf-rs/src/schema/<VERSION>/schema.rs` |
| CSAF-Testcases-Schema | `csaf-rs/src/schema/<VERSION>/testcases_schema.rs` |
| CSAF-Testcases | `csaf-rs/src/<VERSION>/testcases.generated.rs` |

You can always see the available options by running `cargo run -- --help`.
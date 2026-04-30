# Summary

This is a type generator for the csaf-rs library.
It creates rust types from the official CSAF schema.
Optionally you can create types for the testcases schema and a structure for the testcases itself.

# Usage

By default, all generations are run:
```
cargo run
```

To run only specific generations, pass one or more flags. Only the selected generations will run:

Only create types for the CSAF schema
```
cargo run -- --schema
```

Only create types for the testcases.json file
```
cargo run -- --test-schema
```

Only create types for the testcases itself, to work with them in a type safe manner
```
cargo run -- --test-definitions
```

Only create language subtag registry types from the IANA registry
```
cargo run -- --language-tags
```

Only create validation schemas for the different JSON schemas used in the library.
```
cargo run -- --validation-schemas
```

Combine flags to run multiple specific generations
```
cargo run -- --schema --test-schema
```

By default, the target folder is set to `../csaf-rs`, and the generator creates files in the specific subfolders used in this library, but you can override it by passing the `--target-folder` option.

| Type | Default target folder|  
|  | --- |  
| CSAF-Schema | `csaf-rs/src/schema/<VERSION>/schema.rs` |  
| CSAF-Testcases-Schema | `csaf-rs/src/schema/<VERSION>/testcases_schema.rs` |  
| CSAF-Testcases | `csaf-rs/src/<VERSION>/testcases.generated.rs` |  
| Language-Subtags | `csaf-rs/src/csaf/types/language/language_subtags.generated.rs` |  
| Validation-Schemas | `csaf-rs/src/validations/utils/validation_schemas.rs` `csaf-rs/src/validations/utils/validation_schema_urls.rs`  |

You can always see the available options by running `cargo run -- --help`.

## Custom test cases (see [supplementary tests](assets/README.md))

If you want to add custom test cases, you can put them in `assets/tests` in the same manner as they are in the `csaf` folder (JSON file + listed in `testcases.json`)
> Make sure to run the generator with `--test-definitions` afterward, so the new cases get picked up and the code is updated.

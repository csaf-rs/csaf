# Coding Guidelines

- Use the `CsafTrait` / `VulnerabilityTrait` / `ProductTreeTrait` / etc. abstractions for version-agnostic logic.
- When implementing a validation tests, collect all validation errors rather than failing fast — return `Err(Vec<ValidationError>)`, not a single error.

## Pre-Commit Guidelines

To ensure high maintainability, we encourage you to do some checks before you commit code. Those are also checked in the build pipeline and will fail the build if not addressed before. 
```rust
cargo fmt --all // formatting
cargo clippy --all-targets -- -D warnings // linting
cargo test --verbose // tests
```

## Schemas and type generation

We have several json schema files included in the code base from which code is generated. The most prominent being the CSAF repository which is integrated as a git submodule. If there are changes to those schemas, we have to review these changes and incorporate them into our code. This can mean regenerating code (this is here referred to as 'asset-update') and updating existing code to match the new requirements.
To help with this, there are a few update scripts in [scripts/update](./scripts/update) to make this process easier.
Additionally, after using those scripts or after updating a submodule, you should run the type-generator to ensure the generated code is up-to-date.
```rust
cargo run -p type-generator
```
> Updates of the git submodules is also handled by dependabot, so there should not be the need to do this manually.

## Test Harness

In order to test our implementation, the CSAF standard, which is integrated as a git submodule, provides many [test files](./csaf/csaf_2.0/test/validator/data). Those files are incorporated into our test harness in a way, that every file must be addressed with expected results (see [test_6_1_01.rs](./csaf-rs/src/validations/test_6_1_01.rs) for an example). The provided tests do not cover all edge cases, thus we provide supplementary test cases, which are located in the [type-generator](./type-generator/assets/tests). Those can, and possibly should, be extended if a new test is implemented or requirements for an existing test change.
Here is a small step-by-step guide for implementing a new test:

1. Create the new test file following the existing naming convention (`test_6_X_XX.rs`) in the validations folder [csaf-rs/src/validations](./csaf-rs/src/validations)
2. Add the test function, i.e.
```rust
fn validate_missing_product_id<Doc: CsafTrait>(doc: &Doc) -> Result<(), Vec<ValidationError>> {
}
```
3. Add the validator hooks
```rust
crate::test_validation::impl_validator!(ValidatorForTest6_X_X, validate_missing_product_id);
```
4. Update the validation files for 2.0 [csaf-rs/src/csaf2_0/validation.rs](./csaf-rs/src/csaf2_0/validation.rs) and/or 2.1 [csaf-rs/src/csaf2_1/validation.rs](./csaf-rs/src/csaf2_1/validation.rs) to enable a new validation.
```rust
// old
"6.3.5" => None,
// new
"6.3.5" => Some(ValidatorForTest6_3_5.validate(self)),
```
5. Add the test module in [csaf-rs/src/validations/mod.rs](./csaf-rs/src/validations/mod.rs)
6. Add supplementary test files if needed. You can use this script [create-new-testfile.sh](./create-new-testfile.sh) to make this easier. This creates the new testfile, adds it to the testcases.json files and runs the type-generator.
```sh
./create-new-testfile.sh --test 6.2.35 --csaf-version 2.1 --validity failure
```
7. Implement the validation code
8. When finished, update the [implementation status](./README.md#implementation-status-in-regards-to-the-standard) to indicate the implementation state.

When defining the expected results, there is only an `Ok(())` return value for valid test cases. Use either a speaking variable name or leave a comment what these cases are about, i.e.
```rust
// Case 11: disclosure_date equals newest revision date
// Case 12: disclosure_date is definitely not in the past (9999-12-31)
// Case 13: disclosure_date is earlier than newest revision, with timezones

TESTS_2_1.test_6_2_33.expect(
    case_01_disclosure_date_newer_than_newest_rev,
    case_02_disclosure_date_newer_than_newest_rev_with_timezone,
    case_03_disclosure_date_newer_than_newest_rev_with_timezone,
    Ok(()),
    Ok(()),
    Ok(()),
);
```

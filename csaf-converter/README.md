# Architecture & test setup

## Design goal

The main design goal is to ensure that the development of the converter can be performed by several people in parallel.
The achieve this, it must be possible to implement and test each converter feature *independently* with minimal merge conflicts.
Where "independently" means that one feature must not depend on another; and work on a single feature must not change shared data structures or interfaces that other features will use.

Once the converter is feature completed and well covered with tests, further refactors can optimize the implementation for speed and maintainability.

## Piecemeal transformations

The `Convert` trait captures each transform described in the CSAF standard. [1]
The `Convert::convert` method operates on a *WIP CSAF document*.
Although the input to the tool is a valid CSAF 2.0 document, once *a* transform is applied, the WIP document does not adhere to either CSAF 2.0 nor CSAF 2.1.
After *all* transforms have been applied the document must be a valid CSAF 2.1 document; 
this is checked doing a validation according to the CSAF 2.1 standard.
If this validation fails then that indicates either a bug in the tool or that not all transforms have been implemented.

[1]: https://github.com/oasis-tcs/csaf/blob/editor-revision-2026-09-11/csaf_2.1/prose/share/csaf-v2.1-draft.md#conformance-clause-18-csaf-2-0-to-csaf-2-1-converter

For convenience the signature of the `Convert::convert` method is shown below:

``` rust
trait Convert {
    fn convert(
        &self,
        wip: &mut WipDocument,
        opts: &ConversionOptions,
        diagnostics: &mut dyn Diagnostics,
    );
}
```

`ConversionOptions` captures options that affect the behavior of the converter.

`Diagnostics` is used to emit warnings when performing certain transforms.

## The intermediate object

The intermediate object, WIP CSAF document, consists of two components:

- The original document parsed into the CSAF 2.0 schema. This component is immutable.
- A raw JSON version of the original document. This component is mutable and modified by each transform.

After all conversions have been executed, the raw JSON version will be validated according to the CSAF 2.1 schema.

## Grouping transforms

The transforms described in [1] have already been logically grouped in GitHub issues; the list is in [2].
It is therefore recommended to implement the `Convert` trait with the granularity set to that logical grouping.

[2]: https://github.com/csaf-rs/csaf/issues/238

## Unit testing

Each `Convert` implementation should be tested with unit tests.
Each unit test takes an input JSON and compares it to an expect output JSON.
The recommended layout for these artifacts is shown below

``` console
$ eza --tree src
src
├── gh1066
│   ├── input.json
│   ├── mod.rs
│   └── output.json
├── gh1067
│   ├── case1-input.json
│   ├── case1-output.json
│   ├── case2-input.json
│   ├── case2-output.json
│   └── mod.rs
└── main.rs
```

`gh1066/mod.rs` implements the transforms described in issue 1066 [3].
`gh1066/input.json` is the JSON input to the transform; this input must be a valid CSAF 2.0 document.
`gh1066/output.json` is the JSON output of the transform; this output does not have to be a valid CSAF 2.1 document.

[3]: https://github.com/csaf-rs/csaf/issues/1066

It is recommended to keep the JSON files formatted as this makes it easy to quickly compare ("diff") them with a tool like `git-delta` [4]

[4]: https://crates.io/crates/git-delta

A transform may need to be tested under different scenarios, which means more than one pair of JSON files are needed.
It should be easy to tell from the filenames which JSON files belong to the same test scenario.
`gh1067` provides an example of such naming convention.

## Diagnostics

As per the specification, the converter must emit warnings or errors under certain conditions.
The `Diagnostics` trait encodes this functionality.
The `Diagnostics` argument in the `Convert` method has been made a trait object on purpose so it can be mocked with a "spy" in unit tests.
This allows inspecting diagnostics during testing in a "white box" manner while avoiding file or standard stream IO.

### Conformance testing

A test suite for a CSAF 2.0 to 2.1 converter exists in [5].
`converter-testcases-20-21.json` describes what the expected diagnostics (warnings and errors) and pass/fail outcome are for each input file.
`tests/conformance.rs` will execute all the test cases described in said JSON.
At the moment, all tests are marked as `#[ignore]`.
As converter features are implemented tests can be unignored.

To ensure that one does not forget to unignore a test, CI should run the ignored tests and ensure that zero tests pass.
If any ignored test passes that means it should be unignored.

[5]: https://github.com/csaf-testsuite/csaf-2.0-to-csaf-2.1

## Open questions

### Do some transformations need to be performed in a certain order?

Ideally transformations described in [1] can be logically grouped so that no logic group depends on another.
A `priority` "property" has been added to the `Convert` trait to allow ordering transforms.
Transforms will be executed lower priority first.
If transform don't need to be executed in any particular order then said `priority` property can be removed.

If transformations are really independent then it should be possible to execute them in any order without affecting the final CSAF 2.1 document outcome.
The conformance tests could use a development-only feature to execute all transforms in random order; or in *all* possible permutations in which transforms can be ordered.
This would validate that transforms have been logically grouped in a way they are truly independent from each other.

### Reuse logic for quick fixes

*this is pending an example of a quick fix as I didn't find any mention of "quick fix" in the codebase at a glance* on what data structure does it operate? a CSAF 2.0 type or raw JSON? if it's the later then maybe the `Convert` trait can be used more or less "as is" for quick fixes.

## Alternatives

The transformations are performed on raw JSON, which is a relatively low level data structure and somewhat cumbersome to work with as it involves indexing with strings.
Alternative, a new set of schema types could be generated using `type-generator`; let's call this `csaf_2x`.
These `csaf_2x` types should be able to represent valid CSAF 2.0 and valid CSAF 2.1 documents as well as the whole spectrum in between.
Meaning that any given transform on a `csaf_2x` document must result in a valid `csaf_2x` document. 
As not many CSAF tests ("validations") may be applicable to this wide spectrum of documents, a `csaf_2x` document must only *structurally* support all the intermediate states between CSAF 2.0 and CSAF 2.1.
In principle, these `csaf_2x` types are higher level ("more typed") than raw JSON so it should be less error prone to modify fields in the document.
However, generating all these types would increase the amount of generated code in the code base and possibly would require changes to `type-generator`; 
all this adds to the maintenance burden.


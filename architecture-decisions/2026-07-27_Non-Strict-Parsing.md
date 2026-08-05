# 2026-07 Non-Strict Parsing for Validation

## Option A (see csaf-rs/src/extractor)

Implement test as a two-step approach: 

- A stream-based extraction approach to get data needed for a specific tests while traversing a JSON document
- A validation step that uses the extracted data to perform the test

Implement a mechanism to wire the new two-tests to the existing test framework (without the performance gains of running all extractions in parallel, or supporting syntactically invalid JSON).

Consequences:

- Tests have to be rewritten using the new scheme
- Tests must be implemented without the abstractions introduced by the traits or generated types (or have to newly implement custom helpers)
- Tests can be implemented incrementally (without the option of streaming parsing yet)
- Parsing of syntactically invalid JSON can become available once all tests are migrated
- Streaming parsing can become available once all tests are migrated


## Option B (see csaf-rs/src/non_strict)

Introduce non-strict traits for CSAF documents on top of serde_json::Value

- All functions return Option<T> (because elements may not be present in the JSON)
- Collections return an Iterator<T> with reference types
- Traits are no longer directly implemented by the typify types, but by reference wrappers around the types
- Traits are also implemented by reference wrappers around serde_json::Value

To allow incremental conversion, set up a new non-strict trait hierarchy next to the existing traits, to be used by gradually more tests.
Non-strict tests are based on RawValidatable<T>, using a reference wrapper around the serde_json::Value as trait instance.

Consequences:

- Tests have to be adapted to the new traits, but stay similar to the previous state
- Tests can be implemented incrementally
- Parsing of syntactically invalid JSON can be added
- Streaming parsing is not supported


## Context

- A CSAF validator has to work with documents that are not fully compliant with the standard. Tests that are specific to a certain part of the document (e.g. vulnerabilities) should not be influenced by errors in other parts of the document (e.g. revision history).
- During editing, CSAF documents may even be in a state where they are not syntactically valid JSON. Ideally, a CSAF validator can work on these documents and report errors in the part before the syntax error.
- CSAF documents can become large (>100MB). It would be preferable if tests could be executed on streaming data, without keeping the whole document in memory, and ideally with only one traversal of the document for the entire validation. 
- Transition to the new parsing scheme should be possible as an ongoing effort, converting one test after the other while keeping the system running.

## Alternatives

- Schema-aware parsing of the document into types, and then running the tests (first implementation)
    - ❌ Not acceptable, because this parsing can fail due to non-matched regular expression constraints in unrelated parts of the document
- Implement all tests natively upon `serde_json::Value`, without using any intermediate classes
    - ✅ Allows non-strict parsing
    - ✅ Working on syntactically invalid documents is possible, using a partial `serde_json::Value` deserializer
    - ⭕ Tests cannot make use of the abstractions introduced by the traits or generated types (or have to newly implement custom helpers)
    - ❌ Stream-based parsing is not possible, because a `serde_json::Value` is needed
- Introduce non-strict type for CSAF documents that do not enforce constraints at parse time, and deserialize into them
    - ✅ Allows non-strict parsing
    - ✅ Working on syntactically invalid documents is possible, using a partial `serde_json::Value` deserializer
    - ⭕ Introduces a third class structure for CSAF documents (next to typify and traits)
    - ❌ Stream-based parsing is not possible, because a `serde_json::Value` is needed
- Introduce non-strict traits for CSAF documents on top of serde_json::Value (Option B)
    - ✅ Allows non-strict parsing
    - ✅ Working on syntactically invalid documents is possible, using a partial `serde_json::Value` deserializer
    - ⭕ Weakens the existing traits (making usage less convenient) or introduces duplicate traits (one strict, one non-strict)
    - ❌ Stream-based parsing is not possible, because a `serde_json::Value` is needed
- Implement a stream-based extraction approach to get data needed for a specific tests while traversing a JSON document (Option A)
    - ✅ Allows non-strict parsing
    - ✅ Naturally works on syntactically invalid documents (just handling the stream as far as available)
    - ✅ Supports stream-based parsing by wiring the extractors to a custom serde deserializer
    - ⭕ Tests cannot make use of the abstractions introduced by the traits or generated types (or have to newly implement custom helpers)

## Consequences

...

## Confidence Level

...

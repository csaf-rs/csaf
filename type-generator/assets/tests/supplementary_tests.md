# We gather supplementary tests here which will later be proposed to the CSAF TC to incorporate in the csaf repository.

Rules for naming:
- don't use OASIS CSAF prefix or namespace
- use `s` for the test case to indicate supplementary tests
- use the same counting schema as for the original tests, i.e. even numbers in decade indicates invalid cases

## 6.1.1-s01

Covers all relevent paths defined in the spec.

## 6.1.2-s01

Covers all relevant paths defined in the spec and the relations between them.

## 6.1.5-s01

Covers having three groups with the same group_id, which should result in 3 seperate error messages.

## 6.1.5-s11

Adds a valid case for 6.1.5: two groups with different group_ids.

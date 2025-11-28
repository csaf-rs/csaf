# csaf-rs Asset Sources & Licenses

This library is meant to be published as rust crate(s).
For the sake of successful publishing and reproducibility, we have to bundle/vendor relevant (external) assets.
These assets are described within this `README.md`.

## git-based Assets

### CSAF Schemas

- `csaf_2.0_json_schema.json`
- `csaf_2.1_json_schema.json`

See https://github.com/oasis-tcs/csaf for information regarding these files and their respective licenses.
They are 1:1 copies from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

### SSVC Resources

- `ssvc_decision_points/**`
- `decision_point_json_schema.json`

See https://github.com/CERTCC/SSVC/tree/main/data/json/decision_points for information regarding these files and
https://github.com/CERTCC/SSVC/blob/main/data/LICENSE for information about licenses.
They are 1:1 copies from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

- `decision_point_value_selection_list_json_schema.json`

See https://github.com/CERTCC/SSVC/blob/main/data/schema/v2/SelectionList_2_0_0.schema.json for this schema file and
https://github.com/CERTCC/SSVC/blob/main/data/LICENSE for information about licenses.
This is a 1:1 copy from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

## External Assets

- `language-subtag-registry.txt`

Contains IANA's language subtag registry found at 
https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry.
Assumed to be public domain material, according to https://www.iana.org/help/licensing-terms.

### Metric resources

Schema definitions for metric content

- `cvss-v2.0.json` see https://www.first.org/cvss/cvss-v2.0.json
- `cvss-v3.0.json` see https://www.first.org/cvss/cvss-v3.0.json
- `cvss-v3.1.json` see https://www.first.org/cvss/cvss-v3.1.json
- `cvss-v4.0.json` see https://www.first.org/cvss/cvss-v4.0.json

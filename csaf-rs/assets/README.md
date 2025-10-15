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
- `decision_point_1.0.1_json_schema.json`

See https://github.com/CERTCC/SSVC/tree/main/data/json/decision_points for information regarding these files and their 
respective licenses. They are 1:1 copies from the respective repository (i.e., git submodule), cloned solely for
successful crate publishing.

- `Decision_Point_Value_Selection-1-0-1_merged.schema.json`

The SSVC decision point value selection schema provided here is derived from 
https://certcc.github.io/SSVC/data/schema/v1/Decision_Point_Value_Selection-1-0-1.schema.json.
Problematic (e.g., external) `$refs` have been resolved to allow proper parsing during the build process.
Again, see https://github.com/CERTCC/SSVC/tree/main/data/json/decision_points for any further (license) information.

- `Decision_Point_Value_Selection-2-0-0.schema.json`

The SSVC decision point value selection schema provided here has been copied from
https://raw.githubusercontent.com/CERTCC/SSVC/refs/heads/main/data/schema/v2/Decision_Point_Value_Selection-2-0-0.schema.json.


## External Assets

- `language-subtag-registry.txt`

Contains IANA's language subtag registry found at 
https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry.
Assumed to be public domain material, according to https://www.iana.org/help/licensing-terms.
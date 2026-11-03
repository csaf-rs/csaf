use crate::csaf::types::csaf_document_category::CsafDocumentCategory;
use crate::csaf_traits::CsafVersion;
use crate::validation::{TestFinding, TestFindingData};
use crate::validations::utils::raw_json_value_helper::property_string_value_is;
use serde_json::Value;

fn create_missing_external_reference_error(doc_category: &CsafDocumentCategory) -> TestFinding {
    TestFinding::Error(TestFindingData {
        message: format!(
            "Document with category '{doc_category}' must have at least one reference with category 'external'"
        ),
        instance_path: "/document/references".to_string(),
    })
}

/// 6.1.27.2 Document References
///
/// It SHALL be tested that at least one item in `$.document.references` exists that contains a link to an `external` source.
/// The property `category` SHALL be present for this item.
///
/// The relevant values for `$.document.category` are: `csaf_informational_advisory`,
/// `csaf_security_incident_response`, `csaf_superseded`.
pub fn test_6_1_27_02_document_references(json: &Value) -> Result<(), Vec<TestFinding>> {
    let Some(doc_category) = json.pointer("/document/category").and_then(Value::as_str) else {
        return Ok(()); // TODO: skip if there is no doc category
    };
    let doc_category = CsafDocumentCategory::from(doc_category);

    let test_applies = matches!(
        &doc_category,
        CsafDocumentCategory::CsafInformationalAdvisory | CsafDocumentCategory::CsafSecurityIncidentResponse
    ) || (doc_category == CsafDocumentCategory::CsafSuperseded
        && json
            .pointer("/document/csaf_version")
            .and_then(Value::as_str)
            .is_some_and(|version| version == CsafVersion::X21.to_string()));

    if !test_applies {
        return Ok(()); // ToDo generate skipped https://github.com/csaf-rs/csaf/issues/409
    }

    let found_external_reference = json
        .pointer("/document/references")
        .and_then(Value::as_array)
        .is_some_and(|references| {
            references
                .iter()
                .any(|reference| property_string_value_is("/category", "external", reference))
        });

    // if there isn't a reference with category 'external', return an error
    if !found_external_reference {
        return Err(vec![create_missing_external_reference_error(&doc_category)]);
    }

    Ok(())
}

crate::test_validation::impl_raw_json_validator!(ValidatorForTest6_1_27_2, test_6_1_27_02_document_references);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_0::testcases::ExpectedResults_6_1_27_2 as ExpectedResults_2_0;
    use crate::csaf2_0::testcases::TESTS_2_0;
    use crate::csaf2_1::testcases::ExpectedResults_6_1_27_2 as ExpectedResults_2_1;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_27_02() {
        let case_informational_advisory_category_self = Err(vec![create_missing_external_reference_error(
            &CsafDocumentCategory::CsafInformationalAdvisory,
        )]);
        let case_security_incident_response_category_not_present = Err(vec![create_missing_external_reference_error(
            &CsafDocumentCategory::CsafSecurityIncidentResponse,
        )]);
        let case_superseded_category_self = Err(vec![create_missing_external_reference_error(
            &CsafDocumentCategory::CsafSuperseded,
        )]);

        TESTS_2_0.test_6_1_27_2.expect(ExpectedResults_2_0 {
            case_01: case_informational_advisory_category_self.clone(),
        });
        TESTS_2_1.test_6_1_27_2.expect(ExpectedResults_2_1 {
            case_01: case_informational_advisory_category_self,
            case_02: case_security_incident_response_category_not_present,
            case_03: case_superseded_category_self,
            case_11: Ok(()),
            case_12: Ok(()),
            case_13: Ok(()),
        });
    }
}

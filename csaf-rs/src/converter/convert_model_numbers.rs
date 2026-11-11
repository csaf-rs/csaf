use serde_json::Value;

use super::helpers::for_each_product_identification_helper_with_path;
use super::{Convert, Diagnostics, ProductIdentificationNumberSettings, Settings, Warning};
use crate::csaf::types::csaf_product_id_helper_number::ProductIdentificationHelperNumber;
use crate::schema::loose;

pub struct ConvertModelNumbers;

impl Convert for ConvertModelNumbers {
    fn convert(&self, doc: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        convert_numbers(
            doc,
            "model_numbers",
            &settings.model_numbers,
            |value| Warning::PartialModelNumber { value },
            |value| Warning::ModelNumberWithMultipleStars { value },
            diagnostics,
        );
    }
}

pub struct ConvertSerialNumbers;

impl Convert for ConvertSerialNumbers {
    fn convert(&self, doc: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        convert_numbers(
            doc,
            "serial_numbers",
            &settings.serial_numbers,
            |value| Warning::PartialSerialNumber { value },
            |value| Warning::SerialNumberWithMultipleStars { value },
            diagnostics,
        );
    }
}

pub struct ConvertStockKeepingUnits;

impl Convert for ConvertStockKeepingUnits {
    fn convert(&self, doc: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        convert_numbers(
            doc,
            "skus",
            &settings.skus,
            |value| Warning::PartialStockKeepingUnit { value },
            |value| Warning::StockKeepingUnitWithMultipleStars { value },
            diagnostics,
        );
    }
}

/// Converts one product-identification-helper array using the shared CSAF 2.1
/// wildcard rules, reporting field-specific diagnostics and removing ambiguous
/// values with multiple unescaped asterisks.
fn convert_numbers(
    doc: &mut loose::Document,
    field_name: &'static str,
    settings: &ProductIdentificationNumberSettings,
    partial_warning: fn(String) -> Warning,
    multiple_stars_warning: fn(String) -> Warning,
    diagnostics: &mut dyn Diagnostics,
) {
    for_each_product_identification_helper_with_path(doc, |helper, helper_path| {
        let Some(Value::Array(numbers)) = helper.get_mut(field_name) else {
            return;
        };

        let mut index = 0;
        numbers.retain_mut(|number| {
            let current_index = index;
            index += 1;
            let Value::String(value) = number else {
                return true;
            };
            let original = value.clone();
            let converted = convert_number(
                value,
                settings.interpret_as_complete,
                settings.question_mark_is_literal,
                settings.asterisk_is_literal,
            );
            let path = format!("{helper_path}/{field_name}/{current_index}");

            if converted.unescaped_stars > 1 {
                diagnostics.warning(path.into(), multiple_stars_warning(original));
                return false;
            }
            if converted.added_star {
                diagnostics.warning(path.into(), partial_warning(original));
            }
            *value = converted.value;
            true
        });

        if numbers.is_empty() {
            helper.remove(field_name);
        }
    });
}

/// Result of normalizing a product-identification-helper number.
struct ConvertedModelNumber {
    value: String,
    unescaped_stars: usize,
    added_star: bool,
}

/// Escapes a number, optionally adds its open-ended suffix, and counts the
/// resulting unescaped asterisks for ambiguity validation.
fn convert_number(
    input: &str,
    interpret_as_complete: bool,
    question_mark_is_literal: bool,
    asterisk_is_literal: bool,
) -> ConvertedModelNumber {
    // If the stock keeping unit contains a \, the CSAF 2.0 to CSAF 2.1 Converter SHALL escape it by inserting an additional \ before the character.
    // A tool MAY provide a non-default option to interpret the ? in all stock keeping units as part of the stock keeping unit itself and therefore escape it.
    // A tool MAY provide a non-default option to interpret the * in all stock keeping units as part of the stock keeping unit itself and therefore escape it.
    let mut value =
        ProductIdentificationHelperNumber::escape_for_csaf21(input, question_mark_is_literal, asterisk_is_literal);

    let mut added_star = false;
    // If a number is given that does not end on a star, the CSAF 2.0 to CSAF 2.1 Converter SHOULD add a * to the end and issue a warning that a partial stock keeping unit was detected and a star has been added.
    // A tool MAY provide a non-default option to interpret all stock keeping units as complete and therefore does not add any stars.
    // A tool MAY provide a non-default option to interpret the * in all stock keeping units as part of the stock keeping unit itself and therefore escape it.
    if !interpret_as_complete
        && !asterisk_is_literal
        && !ProductIdentificationHelperNumber::ends_in_unescaped_star(&value)
    {
        value.push('*');
        added_star = true;
    }

    // If the stock keeping unit contains multiple unescaped * after the conversion, the CSAF 2.0 to CSAF 2.1 Converter SHALL remove the entry and issue a warning that a stock keeping unit with multiple stars was detected and removed.
    let unescaped_stars = ProductIdentificationHelperNumber::count_unescaped_stars_in(&value) as usize;
    ConvertedModelNumber {
        value,
        unescaped_stars,
        added_star,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::tests::OnlyWarnings;
    use serde_json::json;

    #[test]
    fn converts_model_numbers_and_reports_partial_values() {
        let mut doc = loose::Document::new(json!({
            "product_tree": {"full_product_names": [{
                "product_identification_helper": {"model_numbers": ["ABC\\10", "XYZ*"]}
            }]}
        }));
        let mut diagnostics = OnlyWarnings::default();
        ConvertModelNumbers.convert(&mut doc, &Settings::default(), &mut diagnostics);
        assert_eq!(
            doc.into_raw()["product_tree"]["full_product_names"][0]["product_identification_helper"]["model_numbers"],
            json!(["ABC\\\\10*", "XYZ*"])
        );
        assert_eq!(diagnostics.warnings().len(), 1);
    }

    #[test]
    fn removes_multiple_unescaped_stars() {
        let mut doc = loose::Document::new(json!({
            "product_tree": {"full_product_names": [{
                "product_identification_helper": {"model_numbers": ["A*B*"]}
            }]}
        }));
        let mut diagnostics = OnlyWarnings::default();
        ConvertModelNumbers.convert(&mut doc, &Settings::default(), &mut diagnostics);
        assert_eq!(doc.into_raw()["product_tree"]["full_product_names"][0], json!({}));
        assert!(matches!(
            diagnostics.warnings()[0].1,
            Warning::ModelNumberWithMultipleStars { .. }
        ));
    }

    #[test]
    fn converts_serial_numbers_using_shared_logic() {
        let mut doc = loose::Document::new(json!({
            "product_tree": {"full_product_names": [{
                "product_identification_helper": {"serial_numbers": ["SN\\10", "BAD*A*"]}
            }]}
        }));
        let mut diagnostics = OnlyWarnings::default();
        ConvertSerialNumbers.convert(&mut doc, &Settings::default(), &mut diagnostics);
        assert_eq!(
            doc.into_raw()["product_tree"]["full_product_names"][0]["product_identification_helper"]["serial_numbers"],
            json!(["SN\\\\10*"])
        );
        assert!(matches!(
            diagnostics.warnings()[0].1,
            Warning::PartialSerialNumber { .. }
        ));
        assert!(matches!(
            diagnostics.warnings()[1].1,
            Warning::SerialNumberWithMultipleStars { .. }
        ));
    }

    #[test]
    fn converts_skus_using_shared_logic() {
        let mut doc = loose::Document::new(json!({
            "product_tree": {"full_product_names": [{
                "product_identification_helper": {"skus": ["SKU\\10", "BAD*A*"]}
            }]}
        }));
        let mut diagnostics = OnlyWarnings::default();
        ConvertStockKeepingUnits.convert(&mut doc, &Settings::default(), &mut diagnostics);
        assert_eq!(
            doc.into_raw()["product_tree"]["full_product_names"][0]["product_identification_helper"]["skus"],
            json!(["SKU\\\\10*"])
        );
        assert!(matches!(
            diagnostics.warnings()[0].1,
            Warning::PartialStockKeepingUnit { .. }
        ));
        assert!(matches!(
            diagnostics.warnings()[1].1,
            Warning::StockKeepingUnitWithMultipleStars { .. }
        ));
    }
}

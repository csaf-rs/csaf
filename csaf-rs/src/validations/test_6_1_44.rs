use crate::csaf::types::csaf_product_id_helper_number::CsafSerialNumber;
use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;

fn create_multiple_stars_serial_number_error(number: &CsafSerialNumber, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("Serial number '{number}' must not contain multiple unescaped asterisks (stars)"),
        instance_path: format!("{path}/product_identification_helper/serial_numbers/{index}"),
    }
}

pub fn test_6_1_44_multiple_stars_in_serial_number(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper()
                && let Some(serial_numbers) = helper.get_serial_numbers()
            {
                for (index, serial_number) in serial_numbers.iter().enumerate() {
                    if serial_number.count_unescaped_stars() > 1 {
                        errors
                            .get_or_insert_default()
                            .push(create_multiple_stars_serial_number_error(serial_number, path, index));
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
    for crate::csaf2_1::testcases::ValidatorForTest6_1_44
{
    fn validate(
        &self,
        doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
    ) -> Result<(), Vec<ValidationError>> {
        test_6_1_44_multiple_stars_in_serial_number(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_44() {
        // Ideas for supplementary test cases:
        // S01: 1 serial number, no stars

        // Only CSAF 2.1 has this test with 5 test cases (2 error cases, 3 success cases)
        TESTS_2_1.test_6_1_44.expect(
            // Case 01: One serial number with two unescaped stars
            Err(vec![create_multiple_stars_serial_number_error(
                &CsafSerialNumber::from("P*A*"),
                "/product_tree/full_product_names/0",
                0,
            )]),
            // Case 02: One serial number with one escaped and two unescaped stars
            Err(vec![create_multiple_stars_serial_number_error(
                &CsafSerialNumber::from("*P*\\*?*"),
                "/product_tree/full_product_names/0",
                0,
            )]),
            // Case 03: 5 serial numbers, all end with one unescaped star (and some '?' in between)
            Ok(()),
            // Case 04: 1 serial number, starts with unescaped star, 3 escaped stars
            Ok(()),
            // Case 05: 1 serial number, 2 escaped stars, one escaped backslash
            Ok(()),
        );
    }
}

use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::helpers::count_unescaped_stars;
use crate::validation::ValidationError;

fn create_multiple_stars_error(path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: "Serial number must not contain multiple unescaped asterisks (stars)".to_string(),
        instance_path: format!("{path}/product_identification_helper/serial_numbers/{index}"),
    }
}

pub fn test_6_1_44_multiple_stars_in_serial_number(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                if let Some(serial_numbers) = helper.get_serial_numbers() {
                    for (index, serial_number) in serial_numbers.enumerate() {
                        if count_unescaped_stars(serial_number) > 1 {
                            errors
                                .get_or_insert_with(Vec::new)
                                .push(create_multiple_stars_error(path, index));
                        }
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
        // Only CSAF 2.1 has this test with 5 test cases (2 error cases, 3 success cases)
        TESTS_2_1.test_6_1_44.expect(
            Err(vec![create_multiple_stars_error(
                "/product_tree/full_product_names/0",
                0,
            )]),
            Err(vec![create_multiple_stars_error(
                "/product_tree/full_product_names/0",
                0,
            )]),
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}

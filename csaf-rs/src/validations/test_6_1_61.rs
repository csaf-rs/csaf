use crate::csaf::types::csaf_product_id_helper_number::CsafStockKeepingUnit;
use crate::csaf_traits::{CsafTrait, ProductIdentificationHelperTrait, ProductTrait, ProductTreeTrait};
use crate::validation::ValidationError;

fn create_multiple_stars_sku_error(sku: &CsafStockKeepingUnit, path: &str, index: usize) -> ValidationError {
    ValidationError {
        message: format!("Stock keeping unit '{sku}' must not contain multiple unescaped asterisks (stars)"),
        instance_path: format!("{path}/product_identification_helper/skus/{index}"),
    }
}

pub fn test_6_1_61_multiple_stars_in_sku(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;

    if let Some(product_tree) = doc.get_product_tree() {
        product_tree.visit_all_products(&mut |product, path| {
            if let Some(helper) = product.get_product_identification_helper() {
                let skus = helper.get_skus();
                for (index, sku) in skus.iter().enumerate() {
                    if sku.count_unescaped_stars() > 1 {
                        errors
                            .get_or_insert_default()
                            .push(create_multiple_stars_sku_error(sku, path, index));
                    }
                }
            }
        });
    }

    errors.map_or(Ok(()), Err)
}

crate::test_validation::impl_validator!(csaf2_1, ValidatorForTest6_1_61, test_6_1_61_multiple_stars_in_sku);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf2_1::testcases::TESTS_2_1;

    #[test]
    fn test_test_6_1_61() {
        let case01_two_unescaped = Err(vec![create_multiple_stars_sku_error(
            &CsafStockKeepingUnit::from("NL*12*"),
            "/product_tree/full_product_names/0",
            0,
        )]);
        let case02_escaped_unescaped_mixed = Err(vec![create_multiple_stars_sku_error(
            &CsafStockKeepingUnit::from("*P*\\*?*"),
            "/product_tree/full_product_names/0",
            0,
        )]);
        // Case 11: 5 SKUs, all with only one star
        // Case 12: One unescaped star, multiple escaped stars
        // Case 13: Escaped stars, also escaped question mark

        TESTS_2_1.test_6_1_61.expect(
            case01_two_unescaped,
            case02_escaped_unescaped_mixed,
            Ok(()),
            Ok(()),
            Ok(()),
        );
    }
}

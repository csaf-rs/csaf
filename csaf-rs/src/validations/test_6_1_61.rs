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

// TODO: Implement this after test was merged upstream
// impl crate::test_validation::TestValidator<crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework>
// for crate::csaf2_1::testcases::ValidatorForTest6_1_61
// {
//     fn validate(
//         &self,
//         doc: &crate::schema::csaf2_1::schema::CommonSecurityAdvisoryFramework,
//     ) -> Result<(), Vec<ValidationError>> {
//         test_6_1_61_multiple_stars_in_sku(doc)
//     }
// }
//
// #[cfg(test)]
// mod tests {
//
//     #[test]
//     fn test_test_6_1_61() {
//     }
// }

use crate::csaf_traits::{
    CsafTrait, MetricTrait, ProductGroupTrait, ProductStatusTrait, ProductTreeTrait, RelationshipTrait,
    VulnerabilityTrait,
};

pub fn prepend_path(prefix: &str, idx: &usize, id_path_tuples: Vec<(String, String)>) -> Vec<(String, String)> {
    id_path_tuples
        .iter()
        .map(|(group_or_product_id, path)| (group_or_product_id.to_owned(), format!("/{}/{}/{}", prefix, idx, path)))
        .collect()
}

pub fn gather_product_references(doc: &impl CsafTrait) -> Vec<(String, String)> {
    let mut ids = Vec::<(String, String)>::new();

    ids.append(&mut doc.get_product_references());

    if let Some(pt) = doc.get_product_tree().as_ref() {
        // /product_tree/product_groups[]/product_ids[]
        for (g_i, g) in pt.get_product_groups().iter().enumerate() {
            for (i_i, i) in g.get_product_ids().enumerate() {
                ids.push((
                    (*i).to_owned(),
                    format!("/product_tree/product_groups/{}/product_ids/{}", g_i, i_i),
                ))
            }
        }
        // /product_tree/relationships[]/product_reference
        // /product_tree/relationships[]/relates_to_product_reference
        for (r_i, r) in pt.get_relationships().iter().enumerate() {
            ids.push((
                r.get_product_reference().to_owned(),
                format!("/product_tree/relationships/{}/product_reference", r_i),
            ));
            ids.push((
                r.get_relates_to_product_reference().to_owned(),
                format!("/product_tree/relationships/{}/relates_to_product_reference", r_i),
            ));
        }
    }

    ids
}

use crate::csaf::traits::product_tree::product_path_trait::ProductPathTrait;
use crate::csaf::traits::util::impl_str_field_getter;
use crate::csaf_traits::{CategoryOfTheBranch, ProductGroupTrait, ProductTrait};
use crate::json::JsonPointer;
use crate::schema::csaf2_0::schema::{
    Branch as Branch20, FullProductNameT as FullProductNameT20, ProductGroup as ProductGroup20,
    ProductTree as ProductTree20, Relationship as Relationship20,
};
use crate::schema::csaf2_1::schema::{
    Branch as Branch21, FullProductNameT as FullProductNameT21, ProductGroup as ProductGroup21,
    ProductPath as ProductPath21, ProductTree as ProductTree21,
};
use std::fmt::Write as _;

/// Trait representing an abstract product tree in a CSAF document.
///
/// The `ProductTreeTrait` defines the structure of a product tree and allows
/// access to its product groups.
pub trait ProductTreeTrait {
    // Type Associations

    /// The associated type representing the type of branch in the product tree.
    type BranchType: BranchTrait<Self::FullProductNameType>;

    /// The associated type representing the type of product groups in the product tree.
    type ProductGroupType: ProductGroupTrait;

    /// The associated type representing the type of relationships in the product tree.
    type ProductPathType: ProductPathTrait<Self::FullProductNameType>;

    /// The associated type representing the type of the full product name.
    type FullProductNameType: ProductTrait;

    // Simple Getter methods

    /// Returns an optional reference to the list of branches in the product tree.
    fn get_branches(&self) -> Option<&Vec<Self::BranchType>>;

    /// Retrieves a reference to the list of product groups in the product tree.
    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType>;

    /// Returns the JSON pointer prefix for product path elements based on CSAF version.
    fn get_product_path_prefix(&self) -> &'static str;

    /// Retrieves a reference to the list of relationships in the product tree.
    fn get_product_paths(&self) -> &Vec<Self::ProductPathType>;

    /// Retrieves a reference to the list of full product names in the product tree.
    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType>;

    /// Visits every product ID definition, including duplicate IDs.
    fn visit_all_product_ids(&self, visitor: &mut impl FnMut(&str, &str)) {
        let mut buffer = String::from("/product_tree");
        let mut path = JsonPointer::new(&mut buffer);
        if let Some(branches) = self.get_branches() {
            let mut branches_path = path.push("branches");
            for (index, branch) in branches.iter().enumerate() {
                branch.visit_all_product_ids(&mut branches_path.push(index), visitor);
            }
        }
        {
            let mut names_path = path.push("full_product_names");
            for (index, product) in self.get_full_product_names().iter().enumerate() {
                let mut product_path = names_path.push(index);
                visitor(product.get_product_id(), product_path.push("product_id").as_str());
            }
        }
        let field = self.get_product_path_prefix().rsplit('/').next().unwrap_or_default();
        let mut paths_path = path.push(field);
        for (index, product_path) in self.get_product_paths().iter().enumerate() {
            let mut entry_path = paths_path.push(index);
            let mut name_path = entry_path.push("full_product_name");
            visitor(
                product_path.get_full_product_name().get_product_id(),
                name_path.push("product_id").as_str(),
            );
        }
    }

    /// Visits every product group ID definition, including duplicate IDs.
    fn visit_all_product_group_ids(&self, visitor: &mut impl FnMut(&str, &str)) {
        let mut buffer = String::from("/product_tree/product_groups");
        let mut path = JsonPointer::new(&mut buffer);
        for (index, group) in self.get_product_groups().iter().enumerate() {
            let mut group_path = path.push(index);
            visitor(group.get_group_id(), group_path.push("group_id").as_str());
        }
    }

    /// Visits product references from groups and relationships relative to the product tree.
    fn visit_all_product_references(&self, path: &mut JsonPointer<'_>, visitor: &mut impl FnMut(&str, &str)) {
        {
            let mut groups_path = path.push("product_groups");
            for (group_index, group) in self.get_product_groups().iter().enumerate() {
                let mut group_path = groups_path.push(group_index);
                let mut ids_path = group_path.push("product_ids");
                for (id_index, id) in group.get_product_ids().enumerate() {
                    visitor(id, ids_path.push(id_index).as_str());
                }
            }
        }
        self.visit_relationships_product_references(path, visitor);
    }

    /// Visits only references in relationships or product paths.
    fn visit_relationships_product_references(&self, path: &mut JsonPointer<'_>, visitor: &mut impl FnMut(&str, &str)) {
        let field = self.get_product_path_prefix().rsplit('/').next().unwrap_or_default();
        let mut paths_path = path.push(field);
        for (index, product_path) in self.get_product_paths().iter().enumerate() {
            product_path.visit_all_product_references(&mut paths_path.push(index), visitor);
        }
    }

    // Lookup functions

    /// Returns every product definition carrying the given product ID, searching the same
    /// sources as `visit_all_products_generic()`: branches (recursively), the top-level
    /// full product names, and (for 2.0) the relationships' full product names, or (for 2.1) the product-paths' full product names.
    ///
    /// A valid document defines each product ID once (test 6.1.2), so any element beyond the
    /// first is a duplicate definition; handling those is the caller's choice.
    fn get_products_by_id(&self, product_id: &str) -> Vec<&Self::FullProductNameType> {
        let mut products = Vec::new();
        if let Some(branches) = self.get_branches() {
            for branch in branches {
                branch.collect_products_by_id(product_id, &mut products);
            }
        }
        products.extend(
            self.get_full_product_names()
                .iter()
                .filter(|fpn| fpn.get_product_id() == product_id),
        );
        products.extend(
            self.get_product_paths()
                .iter()
                .map(|path| path.get_full_product_name())
                .filter(|fpn| fpn.get_product_id() == product_id),
        );
        products
    }

    // Visitors for node types in the tree

    /// A trait wrapper for `visit_all_products_generic()` that allows implementations to provide
    /// type-specific callbacks for product traversal.
    ///
    /// This method is intended to be implemented by trait objects to handle their specific
    /// product name types while reusing the generic traversal logic defined in
    /// `visit_all_products_generic()`.
    ///
    /// # Parameters
    /// * `callback` - A mutable function that takes a reference to a product and its path string,
    ///   returning a `Result<(), ValidationError>`. The callback will be invoked with the concrete
    ///   type specified by the implementing trait.
    ///
    /// # Returns
    /// * `Ok(())` if all products were visited successfully
    /// * `Err(ValidationError)` if the callback returned an error for any product
    ///
    /// # Implementation Notes
    /// Trait implementers should typically implement this by delegating to
    /// `visit_all_products_generic()` with the same callback.
    fn visit_all_products(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str));

    /// Visits all product references in the product tree by invoking the provided callback for each
    /// product. Returns with collected error Results provided by `callback`, if occurring.
    ///
    /// This method traverses all locations in the product tree where products can be referenced:
    /// - Products within branches (recursively)
    /// - Full product names at the top level
    /// - Full product names within relationships
    ///
    /// # Parameters
    /// * `callback` - A mutable function that takes a reference to a product and its path string
    ///   and returns a `Result<(), Vec<ValidationError>>`. The path string represents the JSON
    ///   pointer to the product's location in the document.
    ///
    /// # Returns
    /// * `Ok(())` if all products were visited successfully
    /// * `Err(Vec<ValidationError>)` if any callback(s) returned errors for any products
    fn visit_all_products_generic(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str)) {
        // Visit products in branches
        self.visit_all_branches(&mut |branch: &Self::BranchType, path| {
            if let Some(product_ref) = branch.get_product() {
                callback(product_ref, &format!("{path}/product"));
            }
        });

        // Visit full_product_names
        for (i, fpn) in self.get_full_product_names().iter().enumerate() {
            callback(fpn, &format!("/product_tree/full_product_names/{i}"));
        }

        // Visit relationships/product_paths using the new prefix method
        let prefix = self.get_product_path_prefix();
        // Visit relationships
        for (i, rel) in self.get_product_paths().iter().enumerate() {
            callback(rel.get_full_product_name(), &format!("{prefix}/{i}/full_product_name"));
        }
    }

    /// Visits all branches in the product tree by invoking the provided callback for each branch.
    ///
    /// This method traverses all branches in the product tree recursively, calling the callback
    /// function for each branch with its path.
    ///
    /// # Parameters
    /// * `callback` - A mutable function that takes a reference to a branch and its path string.
    ///   The path string represents the JSON pointer to the branch's location in the document.
    fn visit_all_branches(&self, callback: &mut impl FnMut(&Self::BranchType, &str)) {
        if let Some(branches) = self.get_branches().as_ref() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_all_branches(&format!("/product_tree/branches/{i}"), callback);
            }
        }
    }

    /// Collects all paths from the product tree root to each leaf node (FPN).
    ///
    /// It also collects the branch indices of that leaf node. For this, it utilizes recursion to do
    /// depth-first traversal with backtracking.
    ///
    /// The indices can be converted to an instance path string on demand via
    /// [`build_leaf_instance_path`].
    ///
    /// # Returns
    /// A vector of tuples, one for each leaf node, where each tuple contains:
    /// - `Vec<&BranchType>`: references of branches from root to leaf
    /// - `Vec<usize>`: branch indices along the path
    fn collect_leaf_paths(&self) -> Vec<(Vec<&Self::BranchType>, Vec<usize>)> {
        let mut result: Vec<(Vec<&Self::BranchType>, Vec<usize>)> = Vec::new();

        if let Some(branches) = self.get_branches().as_ref() {
            // for each root branch, initialize a new vec of branches and indices
            for (i, branch) in branches.iter().enumerate() {
                let mut current_path: Vec<&Self::BranchType> = Vec::new();
                let mut indices: Vec<usize> = vec![i];
                // start recursion and collect all paths to leaf nodes into the result
                branch.collect_leaf_paths(&mut current_path, &mut indices, &mut result);
            }
        }

        result
    }
}

/// Constructs a JSON-pointer instance path from collected branch indices.
///
/// # Arguments
/// - indices: slice of branch indices
///
/// # Returns
/// instance path constructed from branch indices (e.g. 0,0,0 -> "/product_tree/branches/0/branches/0/branches/0/product")
pub fn build_leaf_instance_path(indices: &[usize]) -> String {
    // 13 for "/product_tree" + per index (10 for "/branches/" + up to 5 digits) + 8 for "/product"
    let mut path = String::with_capacity(13 + indices.len() * 15 + 8);
    path.push_str("/product_tree");
    for idx in indices {
        path.push_str("/branches/");
        // Writing to a String should never fail
        let _ = write!(path, "{idx}");
    }
    path.push_str("/product");
    path
}

/// Trait representing an abstract branch in a product tree.
pub trait BranchTrait<FPN: ProductTrait>: Sized {
    /// Visits product ID definitions in this branch and its descendants.
    fn visit_all_product_ids(&self, path: &mut JsonPointer<'_>, visitor: &mut impl FnMut(&str, &str)) {
        if let Some(product) = self.get_product() {
            let mut product_path = path.push("product");
            visitor(product.get_product_id(), product_path.push("product_id").as_str());
        }
        if let Some(branches) = self.get_branches() {
            let mut branches_path = path.push("branches");
            for (index, branch) in branches.iter().enumerate() {
                branch.visit_all_product_ids(&mut branches_path.push(index), visitor);
            }
        }
    }

    /// Returns an optional reference to the child branches of this branch.
    fn get_branches(&self) -> Option<&Vec<Self>>;

    fn get_category(&self) -> CategoryOfTheBranch;

    fn get_name(&self) -> &str;

    /// Retrieves the full product name associated with this branch, if available.
    fn get_product(&self) -> Option<&FPN>;

    /// Appends every product definition carrying the given product ID from this branch and
    /// its descendants to `products`.
    fn collect_products_by_id<'a>(&'a self, product_id: &str, products: &mut Vec<&'a FPN>) {
        if let Some(product) = self.get_product()
            && product.get_product_id() == product_id
        {
            products.push(product);
        }
        if let Some(branches) = self.get_branches() {
            for branch in branches {
                branch.collect_products_by_id(product_id, products);
            }
        }
    }

    /// Recursively visits all branches in the tree structure,
    /// applying the provided callback function to each branch.
    ///
    /// This method traverses the entire branch hierarchy, starting from the current branch and
    /// proceeding depth-first through all child branches. For each branch, it calls the
    /// provided callback function with the branch object and its path representation.
    ///
    /// # Parameters
    /// * `path` - A string representing the current path in the branch hierarchy
    /// * `callback` - A mutable function that takes a reference to Self and the
    ///   current path string and returns a Result
    ///
    /// # Returns
    /// * `Ok(())` if the traversal completes successfully
    /// * `Err(Vec<ValidationError>)` if the callback returns an error for any branch
    fn visit_all_branches(&self, path: &str, callback: &mut impl FnMut(&Self, &str)) {
        callback(self, path);
        if let Some(branches) = self.get_branches() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_all_branches(&format!("{path}/branches/{i}"), callback);
            }
        }
    }

    /// Searches for branches that exceed the maximum allowed depth in the branch hierarchy.
    ///
    /// This method recursively checks if the branch structure exceeds the specified depth limit.
    /// It traverses the branch hierarchy depth-first, decrementing the remaining depth parameter
    /// at each level. If branches are found beyond the allowed depth, it returns the path to the
    /// first excessive branch.
    ///
    /// # Parameters
    /// * `remaining_depth` - The maximum number of branch levels still allowed
    ///
    /// # Returns
    /// * `Some(String)` containing the path to the first branch that exceeds the allowed depth
    /// * `None` if no branches exceed the allowed depth
    fn find_excessive_branch_depth(&self, remaining_depth: u32) -> Option<String> {
        if let Some(branches) = self.get_branches() {
            // If we've reached the depth limit and there are branches, we've found a violation
            if remaining_depth == 1 {
                return Some("/branches/0".to_string());
            }
            for (i, branch) in branches.iter().enumerate() {
                if let Some(sub_path) = branch.find_excessive_branch_depth(remaining_depth - 1) {
                    return Some(format!("/branches/{i}{sub_path}"));
                }
            }
        }
        None
    }

    /// Recursively collects the branches and their indices from the current branch to all leaf nodes.
    /// Utilizes depth-first traversal with backtracking.
    ///
    /// This is a helper method for `ProductTreeTrait::collect_leaf_paths()`.
    ///
    /// # Arguments
    /// * `branches_on_path` - A mutable vector for the branches along the current path
    /// * `indices_on_path` - A mutable vector of branch indices along the current path
    /// * `result` - A mutable vector to collect (branches, indices) tuples for each leaf node
    fn collect_leaf_paths<'a>(
        &'a self,
        branches_on_path: &mut Vec<&'a Self>,
        indices_on_path: &mut Vec<usize>,
        result: &mut Vec<(Vec<&'a Self>, Vec<usize>)>,
    ) {
        // push the current branch to the branches
        branches_on_path.push(self);

        match self.get_branches() {
            // TODO: depending on how we implement extended schema validation, the children.is_empty() might not be necessary
            Some(branches) if !branches.is_empty() => {
                // there are still nodes to visit, recurse for each branch
                for (i, child) in branches.iter().enumerate() {
                    indices_on_path.push(i);
                    child.collect_leaf_paths(branches_on_path, indices_on_path, result);
                    indices_on_path.pop();
                }
            },
            _ => {
                // we are at a leaf node
                result.push((branches_on_path.clone(), indices_on_path.clone()));
            },
        };

        // backtrack
        branches_on_path.pop();
    }
}

impl ProductTreeTrait for ProductTree20 {
    type BranchType = Branch20;
    type ProductGroupType = ProductGroup20;
    type ProductPathType = Relationship20;
    type FullProductNameType = FullProductNameT20;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_deref()
    }

    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType> {
        &self.product_groups
    }

    fn get_product_path_prefix(&self) -> &'static str {
        "/product_tree/relationships"
    }

    fn get_product_paths(&self) -> &Vec<Self::ProductPathType> {
        &self.relationships
    }

    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType> {
        &self.full_product_names
    }

    fn visit_all_products(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str)) {
        self.visit_all_products_generic(callback)
    }
}

impl ProductTreeTrait for ProductTree21 {
    type BranchType = Branch21;
    type ProductGroupType = ProductGroup21;
    type ProductPathType = ProductPath21;
    type FullProductNameType = FullProductNameT21;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_deref()
    }

    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType> {
        &self.product_groups
    }

    fn get_product_path_prefix(&self) -> &'static str {
        "/product_tree/product_paths"
    }

    fn get_product_paths(&self) -> &Vec<Self::ProductPathType> {
        &self.product_paths
    }

    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType> {
        &self.full_product_names
    }

    fn visit_all_products(&self, callback: &mut impl FnMut(&Self::FullProductNameType, &str)) {
        self.visit_all_products_generic(callback)
    }
}

impl BranchTrait<FullProductNameT20> for Branch20 {
    fn get_branches(&self) -> Option<&Vec<Self>> {
        self.branches.as_deref()
    }

    fn get_category(&self) -> CategoryOfTheBranch {
        self.category.into()
    }

    impl_str_field_getter!(get_name, name);

    fn get_product(&self) -> Option<&FullProductNameT20> {
        self.product.as_ref()
    }
}

impl BranchTrait<FullProductNameT21> for Branch21 {
    fn get_branches(&self) -> Option<&Vec<Self>> {
        self.branches.as_deref()
    }

    fn get_category(&self) -> CategoryOfTheBranch {
        self.category.into()
    }

    impl_str_field_getter!(get_name, name);

    fn get_product(&self) -> Option<&FullProductNameT21> {
        self.product.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn get_products_by_id_searches_all_sources_in_csaf_2_0() {
        let tree: ProductTree20 = serde_json::from_value(json!({
            "branches": [{
                "category": "vendor",
                "name": "Vendor",
                "branches": [{
                    "category": "product_name",
                    "name": "Product",
                    "product": { "product_id": "CSAFPID-0001", "name": "Vendor Product 1.0" }
                }]
            }],
            "full_product_names": [{ "product_id": "CSAFPID-0002", "name": "Top-level product" }],
            "relationships": [{
                "category": "installed_on",
                "full_product_name": { "product_id": "CSAFPID-0003", "name": "Product on top-level" },
                "product_reference": "CSAFPID-0001",
                "relates_to_product_reference": "CSAFPID-0002"
            }]
        }))
        .expect("product tree deserializes");

        let names = |id: &str| -> Vec<&str> { tree.get_products_by_id(id).iter().map(|fpn| fpn.get_name()).collect() };
        assert_eq!(names("CSAFPID-0001"), ["Vendor Product 1.0"]);
        assert_eq!(names("CSAFPID-0002"), ["Top-level product"]);
        assert_eq!(names("CSAFPID-0003"), ["Product on top-level"]);
        assert!(names("CSAFPID-9999").is_empty());
        let mut paths = Vec::new();
        tree.visit_all_product_ids(&mut |_, path| paths.push(path.to_owned()));
        assert_eq!(
            paths,
            [
                "/product_tree/branches/0/branches/0/product/product_id",
                "/product_tree/full_product_names/0/product_id",
                "/product_tree/relationships/0/full_product_name/product_id",
            ]
        );
    }

    #[test]
    fn get_products_by_id_returns_every_definition_in_csaf_2_0() {
        let tree: ProductTree20 = serde_json::from_value(json!({
            "branches": [{
                "category": "product_name",
                "name": "Product",
                "product": { "product_id": "CSAFPID-0001", "name": "Defined in a branch" }
            }],
            "full_product_names": [{ "product_id": "CSAFPID-0001", "name": "Defined at the top level" }]
        }))
        .expect("product tree deserializes");

        let names: Vec<&str> = tree
            .get_products_by_id("CSAFPID-0001")
            .iter()
            .map(|fpn| fpn.get_name())
            .collect();
        assert_eq!(names, ["Defined in a branch", "Defined at the top level"]);
    }

    #[test]
    fn get_products_by_id_searches_all_sources_in_csaf_2_1() {
        let tree: ProductTree21 = serde_json::from_value(json!({
            "branches": [{
                "category": "vendor",
                "name": "Vendor",
                "branches": [{
                    "category": "product_name",
                    "name": "Product",
                    "product": { "product_id": "CSAFPID-0001", "name": "Vendor Product 1.0" }
                }]
            }],
            "full_product_names": [{ "product_id": "CSAFPID-0002", "name": "Top-level product" }],
            "product_paths": [{
                "beginning_product_reference": "CSAFPID-0001",
                "full_product_name": { "product_id": "CSAFPID-0003", "name": "Product on top-level" },
                "subpaths": []
            }]
        }))
        .expect("product tree deserializes");

        let names = |id: &str| -> Vec<&str> { tree.get_products_by_id(id).iter().map(|fpn| fpn.get_name()).collect() };
        assert_eq!(names("CSAFPID-0001"), ["Vendor Product 1.0"]);
        assert_eq!(names("CSAFPID-0002"), ["Top-level product"]);
        assert_eq!(names("CSAFPID-0003"), ["Product on top-level"]);
        assert!(names("CSAFPID-9999").is_empty());
        let mut paths = Vec::new();
        tree.visit_all_product_ids(&mut |_, path| paths.push(path.to_owned()));
        assert_eq!(
            paths,
            [
                "/product_tree/branches/0/branches/0/product/product_id",
                "/product_tree/full_product_names/0/product_id",
                "/product_tree/product_paths/0/full_product_name/product_id",
            ]
        );
    }

    #[test]
    fn get_products_by_id_returns_every_definition_in_csaf_2_1() {
        let tree: ProductTree21 = serde_json::from_value(json!({
            "branches": [{
                "category": "product_name",
                "name": "Product",
                "product": { "product_id": "CSAFPID-0001", "name": "Defined in a branch" }
            }],
            "full_product_names": [{ "product_id": "CSAFPID-0001", "name": "Defined at the top level" }]
        }))
        .expect("product tree deserializes");

        let names: Vec<&str> = tree
            .get_products_by_id("CSAFPID-0001")
            .iter()
            .map(|fpn| fpn.get_name())
            .collect();
        assert_eq!(names, ["Defined in a branch", "Defined at the top level"]);
    }
}

use crate::csaf::traits::product_tree::relationship_trait::RelationshipTrait;
use crate::csaf_traits::{CategoryOfTheBranch as CategoryOfTheBranchTrait, ProductGroupTrait, ProductTrait};
use crate::schema::csaf2_0::schema::{
    Branch as Branch20, CategoryOfTheBranch as CategoryOfTheBranch20, FullProductNameT as FullProductNameT20,
    ProductGroup as ProductGroup20, ProductTree as ProductTree20, Relationship as Relationship20,
};
use crate::schema::csaf2_1::schema::{
    Branch as Branch21, CategoryOfTheBranch as CategoryOfTheBranch21, FullProductNameT as FullProductNameT21,
    ProductGroup as ProductGroup21, ProductPath as ProductPath21, ProductTree as ProductTree21,
};
use std::ops::Deref;

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
    type RelationshipType: RelationshipTrait<Self::FullProductNameType>;

    /// The associated type representing the type of the full product name.
    type FullProductNameType: ProductTrait;

    // Simple Getter methods

    /// Returns an optional reference to the list of branches in the product tree.
    fn get_branches(&self) -> Option<&Vec<Self::BranchType>>;

    /// Retrieves a reference to the list of product groups in the product tree.
    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType>;

    /// Retrieves a reference to the list of relationships in the product tree.
    fn get_relationships(&self) -> &Vec<Self::RelationshipType>;

    /// Retrieves a reference to the list of full product names in the product tree.
    fn get_full_product_names(&self) -> &Vec<Self::FullProductNameType>;

    // Aggregator functions for product references

    /// Utility function to get all product references in product groups along with their JSON paths
    fn get_product_groups_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        for (pg_i, pg) in self.get_product_groups().iter().enumerate() {
            for (p_i, p) in pg.get_product_ids().enumerate() {
                ids.push((
                    (*p).to_owned(),
                    format!("/product_tree/product_groups/{pg_i}/product_ids/{p_i}"),
                ));
            }
        }

        ids
    }

    /// Utility function to get all product references in relationships along with their JSON paths
    fn get_relationships_product_references(&self) -> Vec<(String, String)> {
        let mut ids: Vec<(String, String)> = Vec::new();

        for (rel_i, rel) in self.get_relationships().iter().enumerate() {
            ids.push((
                rel.get_product_reference().to_owned(),
                format!("/product_tree/relationships/{rel_i}/product_reference"),
            ));
            ids.push((
                rel.get_relates_to_product_reference().to_owned(),
                format!("/product_tree/relationships/{rel_i}/relates_to_product_reference"),
            ));
        }

        ids
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

        // Visit relationships
        for (i, rel) in self.get_relationships().iter().enumerate() {
            callback(
                rel.get_full_product_name(),
                &format!("/product_tree/relationships/{i}/full_product_name"),
            );
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
                branch.visit_branches_rec(&format!("/product_tree/branches/{i}"), callback);
            }
        }
    }

    /// Collects all paths from the product tree root to each leaf node (FPN).
    ///
    /// It also collects the instance path of that leaf node. For this, it utilizes recursion to do
    /// depth-first traversal with backtracking.
    ///
    /// # Returns
    /// A vector of tuples, one for each leaf node, where each tuple contains:
    /// - `Vec<&BranchType>`: references of branches from root to leaf
    /// - `String`: instance path (i.e. `/product_tree/branches/0/branches/0`)
    fn collect_leaf_paths(&self) -> Vec<(Vec<&Self::BranchType>, String)> {
        let mut result: Vec<(Vec<&Self::BranchType>, String)> = Vec::new();

        if let Some(branches) = self.get_branches().as_ref() {
            // for each root branch, initialize a new vec of branches and path
            for (i, branch) in branches.iter().enumerate() {
                let mut current_path: Vec<&Self::BranchType> = Vec::new();
                let instance_path = format!("/product_tree/branches/{i}");
                // start recursion and collect all paths to leaf nodes into the result
                result.extend(branch.collect_leaf_paths_rec(&mut current_path, instance_path));
            }
        }

        result
    }
}

/// Trait representing an abstract branch in a product tree.
pub trait BranchTrait<FPN: ProductTrait>: Sized {
    /// Returns an optional reference to the child branches of this branch.
    fn get_branches(&self) -> Option<&Vec<Self>>;

    fn get_category(&self) -> &CategoryOfTheBranchTrait;

    fn get_name(&self) -> &str;

    /// Retrieves the full product name associated with this branch, if available.
    fn get_product(&self) -> &Option<FPN>;

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
    fn visit_branches_rec(&self, path: &str, callback: &mut impl FnMut(&Self, &str)) {
        callback(self, path);
        if let Some(branches) = self.get_branches().as_ref() {
            for (i, branch) in branches.iter().enumerate() {
                branch.visit_branches_rec(&format!("{path}/branches/{i}"), callback);
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
    /// * `branches` - A mutable vector for the branches along the current path
    /// * `instance_path` - The current instance path
    ///
    /// # Returns
    /// A vector of tuples (vector of branches along the path, instance_path) for each leaf node found
    fn collect_leaf_paths_rec<'a>(
        &'a self,
        branches: &mut Vec<&'a Self>,
        instance_path: String,
    ) -> Vec<(Vec<&'a Self>, String)> {
        // push the current branch to the branches
        branches.push(self);

        let result = match self.get_branches() {
            // TODO: depending on how we implement extended schema validation, the children.is_empty() might not be necessary
            Some(children) if !children.is_empty() => {
                // there are still nodes to visit
                let mut collected = Vec::new();
                for (i, child) in children.iter().enumerate() {
                    // add another level to the instance path and continue recursion
                    let child_instance_path = format!("{}/branches/{}", instance_path, i);
                    collected.extend(child.collect_leaf_paths_rec(branches, child_instance_path));
                }
                collected
            },
            _ => {
                // we are at a leaf node
                let leaf_path = format!("{instance_path}/product");
                vec![(branches.clone(), leaf_path)]
            },
        };

        // backtrack
        branches.pop();
        result
    }


}

impl ProductTreeTrait for ProductTree20 {
    type BranchType = Branch20;
    type ProductGroupType = ProductGroup20;
    type RelationshipType = Relationship20;
    type FullProductNameType = FullProductNameT20;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_deref()
    }

    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType> {
        &self.product_groups
    }

    fn get_relationships(&self) -> &Vec<Self::RelationshipType> {
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
    type RelationshipType = ProductPath21;
    type FullProductNameType = FullProductNameT21;

    fn get_branches(&self) -> Option<&Vec<Self::BranchType>> {
        self.branches.as_deref()
    }

    fn get_product_groups(&self) -> &Vec<Self::ProductGroupType> {
        &self.product_groups
    }

    fn get_relationships(&self) -> &Vec<Self::RelationshipType> {
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

    fn get_category(&self) -> &CategoryOfTheBranchTrait {
        match self.category {
            CategoryOfTheBranch20::Architecture => &CategoryOfTheBranchTrait::Architecture,
            CategoryOfTheBranch20::HostName => &CategoryOfTheBranchTrait::HostName,
            CategoryOfTheBranch20::Language => &CategoryOfTheBranchTrait::Language,
            CategoryOfTheBranch20::Legacy => &CategoryOfTheBranchTrait::Legacy,
            CategoryOfTheBranch20::PatchLevel => &CategoryOfTheBranchTrait::PatchLevel,
            CategoryOfTheBranch20::ProductFamily => &CategoryOfTheBranchTrait::ProductFamily,
            CategoryOfTheBranch20::ProductName => &CategoryOfTheBranchTrait::ProductName,
            CategoryOfTheBranch20::ProductVersion => &CategoryOfTheBranchTrait::ProductVersion,
            CategoryOfTheBranch20::ProductVersionRange => &CategoryOfTheBranchTrait::ProductVersionRange,
            CategoryOfTheBranch20::ServicePack => &CategoryOfTheBranchTrait::ServicePack,
            CategoryOfTheBranch20::Specification => &CategoryOfTheBranchTrait::Specification,
            CategoryOfTheBranch20::Vendor => &CategoryOfTheBranchTrait::Vendor,
        }
    }

    fn get_name(&self) -> &str {
        self.name.deref()
    }

    fn get_product(&self) -> &Option<FullProductNameT20> {
        &self.product
    }
}

impl BranchTrait<FullProductNameT21> for Branch21 {
    fn get_branches(&self) -> Option<&Vec<Self>> {
        self.branches.as_deref()
    }

    fn get_category(&self) -> &CategoryOfTheBranchTrait {
        match self.category {
            CategoryOfTheBranch21::Architecture => &CategoryOfTheBranchTrait::Architecture,
            CategoryOfTheBranch21::HostName => &CategoryOfTheBranchTrait::HostName,
            CategoryOfTheBranch21::Language => &CategoryOfTheBranchTrait::Language,
            CategoryOfTheBranch21::PatchLevel => &CategoryOfTheBranchTrait::PatchLevel,
            CategoryOfTheBranch21::ProductFamily => &CategoryOfTheBranchTrait::ProductFamily,
            CategoryOfTheBranch21::ProductName => &CategoryOfTheBranchTrait::ProductName,
            CategoryOfTheBranch21::ProductVersion => &CategoryOfTheBranchTrait::ProductVersion,
            CategoryOfTheBranch21::ProductVersionRange => &CategoryOfTheBranchTrait::ProductVersionRange,
            CategoryOfTheBranch21::ServicePack => &CategoryOfTheBranchTrait::ServicePack,
            CategoryOfTheBranch21::Specification => &CategoryOfTheBranchTrait::Specification,
            CategoryOfTheBranch21::Vendor => &CategoryOfTheBranchTrait::Vendor,
            CategoryOfTheBranch21::Platform => &CategoryOfTheBranchTrait::Platform,
        }
    }

    fn get_name(&self) -> &str {
        self.name.deref()
    }

    fn get_product(&self) -> &Option<FullProductNameT21> {
        &self.product
    }
}

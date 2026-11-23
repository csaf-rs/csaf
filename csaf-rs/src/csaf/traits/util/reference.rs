use crate::csaf::enums::product_status::ProductStatus;
use std::fmt;

/// The kind of ID list an element references other elements by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdList {
    /// A `product_ids` list.
    ProductIds,
    /// A `group_ids` list.
    GroupIds,
}

impl IdList {
    /// Returns the JSON property name of the list.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductIds => "product_ids",
            Self::GroupIds => "group_ids",
        }
    }
}

/// Where a product or product group ID is referenced in a CSAF document.
///
/// Locations do not allocate. The JSON path is only rendered when the location is displayed,
/// so they can be created for every reference of a large document.
///
/// Locations inside a vulnerability carry its index as `vulnerability`. It is `None` when the
/// reference was obtained from the vulnerability itself, in which case the path is relative to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReferenceLocation<'a> {
    /// `[/vulnerabilities/{vulnerability}]/{field}/{item}/{list}/{index}`, for elements referencing
    /// products or groups through `product_ids` or `group_ids`, like flags, notes or remediations.
    Field {
        /// Index of the containing vulnerability, if any.
        vulnerability: Option<usize>,
        /// Path of the array containing the element, like `remediations` or `document/notes`.
        field: &'static str,
        /// Index of the element in its array.
        item: usize,
        /// The list containing the reference.
        list: IdList,
        /// Index of the reference in the list.
        index: usize,
    },
    /// `[/vulnerabilities/{vulnerability}]/product_status/{status}/{index}`.
    ProductStatus {
        /// Index of the containing vulnerability, if any.
        vulnerability: Option<usize>,
        /// The product status list containing the reference.
        status: ProductStatus,
        /// Index of the reference in the list.
        index: usize,
    },
    /// `[/vulnerabilities/{vulnerability}]/{property}/{metric}/products/{index}`.
    Metric {
        /// Index of the containing vulnerability, if any.
        vulnerability: Option<usize>,
        /// Property name of the metrics, `scores` in CSAF 2.0 and `metrics` in CSAF 2.1.
        property: &'a str,
        /// Index of the metric.
        metric: usize,
        /// Index of the reference in the metric's products.
        index: usize,
    },
    /// `/product_tree/product_groups/{group}/product_ids/{index}`.
    ProductGroup {
        /// Index of the product group.
        group: usize,
        /// Index of the reference in the group's products.
        index: usize,
    },
    /// `/product_tree/relationships/{relationship}/product_reference` or
    /// `/product_tree/relationships/{relationship}/relates_to_product_reference` (CSAF 2.0).
    Relationship {
        /// Index of the relationship.
        relationship: usize,
        /// Whether this is the `relates_to_product_reference` instead of the `product_reference`.
        relates_to: bool,
    },
    /// `/product_tree/product_paths/{product_path}/beginning_product_reference` or
    /// `/product_tree/product_paths/{product_path}/subpaths/{subpath}/next_product_reference` (CSAF 2.1).
    ProductPath {
        /// Index of the product path.
        product_path: usize,
        /// Index of the subpath, or `None` for the beginning product reference.
        subpath: Option<usize>,
    },
}

impl ReferenceLocation<'_> {
    /// Returns this location placed in the vulnerability with the given index.
    ///
    /// Locations outside of vulnerabilities are returned unchanged.
    pub fn in_vulnerability(mut self, index: usize) -> Self {
        match &mut self {
            Self::Field { vulnerability, .. }
            | Self::ProductStatus { vulnerability, .. }
            | Self::Metric { vulnerability, .. } => *vulnerability = Some(index),
            Self::ProductGroup { .. } | Self::Relationship { .. } | Self::ProductPath { .. } => {},
        }
        self
    }
}

/// Writes the `/vulnerabilities/{index}` prefix, if any.
fn write_vulnerability(f: &mut fmt::Formatter<'_>, vulnerability: Option<usize>) -> fmt::Result {
    match vulnerability {
        Some(index) => write!(f, "/vulnerabilities/{index}"),
        None => Ok(()),
    }
}

impl fmt::Display for ReferenceLocation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Field {
                vulnerability,
                field,
                item,
                list,
                index,
            } => {
                write_vulnerability(f, vulnerability)?;
                write!(f, "/{field}/{item}/{}/{index}", list.as_str())
            },
            Self::ProductStatus {
                vulnerability,
                status,
                index,
            } => {
                write_vulnerability(f, vulnerability)?;
                write!(f, "/product_status/{status}/{index}")
            },
            Self::Metric {
                vulnerability,
                property,
                metric,
                index,
            } => {
                write_vulnerability(f, vulnerability)?;
                write!(f, "/{property}/{metric}/products/{index}")
            },
            Self::ProductGroup { group, index } => {
                write!(f, "/product_tree/product_groups/{group}/product_ids/{index}")
            },
            Self::Relationship {
                relationship,
                relates_to,
            } => {
                let property = if relates_to {
                    "relates_to_product_reference"
                } else {
                    "product_reference"
                };
                write!(f, "/product_tree/relationships/{relationship}/{property}")
            },
            Self::ProductPath { product_path, subpath } => match subpath {
                Some(subpath) => write!(
                    f,
                    "/product_tree/product_paths/{product_path}/subpaths/{subpath}/next_product_reference"
                ),
                None => write!(
                    f,
                    "/product_tree/product_paths/{product_path}/beginning_product_reference"
                ),
            },
        }
    }
}

/// A product or product group ID referenced somewhere in a document, borrowed from the document.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IdReference<'a> {
    /// The referenced ID.
    pub id: &'a str,
    /// Where the reference is located.
    pub location: ReferenceLocation<'a>,
}

impl<'a> IdReference<'a> {
    /// Creates a new reference.
    pub const fn new(id: &'a str, location: ReferenceLocation<'a>) -> Self {
        Self { id, location }
    }

    /// Returns this reference placed in the vulnerability with the given index.
    pub fn in_vulnerability(self, index: usize) -> Self {
        Self {
            id: self.id,
            location: self.location.in_vulnerability(index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_paths() {
        let cases = [
            (
                ReferenceLocation::Field {
                    vulnerability: None,
                    field: "document/notes",
                    item: 1,
                    list: IdList::GroupIds,
                    index: 2,
                },
                "/document/notes/1/group_ids/2",
            ),
            (
                ReferenceLocation::Field {
                    vulnerability: None,
                    field: "remediations",
                    item: 0,
                    list: IdList::ProductIds,
                    index: 3,
                }
                .in_vulnerability(4),
                "/vulnerabilities/4/remediations/0/product_ids/3",
            ),
            (
                ReferenceLocation::ProductStatus {
                    vulnerability: Some(1),
                    status: ProductStatus::KnownNotAffected,
                    index: 5,
                },
                "/vulnerabilities/1/product_status/known_not_affected/5",
            ),
            (
                ReferenceLocation::Metric {
                    vulnerability: Some(0),
                    property: "scores",
                    metric: 2,
                    index: 1,
                },
                "/vulnerabilities/0/scores/2/products/1",
            ),
            (
                ReferenceLocation::ProductGroup { group: 3, index: 0 },
                "/product_tree/product_groups/3/product_ids/0",
            ),
            (
                ReferenceLocation::Relationship {
                    relationship: 2,
                    relates_to: true,
                },
                "/product_tree/relationships/2/relates_to_product_reference",
            ),
            (
                ReferenceLocation::ProductPath {
                    product_path: 1,
                    subpath: Some(0),
                },
                "/product_tree/product_paths/1/subpaths/0/next_product_reference",
            ),
            (
                ReferenceLocation::ProductPath {
                    product_path: 1,
                    subpath: None,
                },
                "/product_tree/product_paths/1/beginning_product_reference",
            ),
        ];
        for (location, expected) in cases {
            assert_eq!(location.to_string(), expected);
        }
    }

    #[test]
    fn tree_locations_ignore_vulnerability() {
        let location = ReferenceLocation::ProductGroup { group: 0, index: 0 };
        assert_eq!(location.in_vulnerability(3), location);
    }
}

use crate::csaf_traits::{CsafTrait, ProductGroupTrait, ProductTreeTrait};
use chrono::NaiveDate;
use rust_embed::RustEmbed;
use std::collections::{BTreeSet, HashMap};
use std::sync::LazyLock;
use uuid::{Uuid, uuid};

/// Special name for public sharing groups
pub static SG_NAME_PUBLIC: &str = "Public";
/// Special name for private sharing groups
pub static SG_NAME_PRIVATE: &str = "No sharing allowed";

/// Special "max" UUID value
pub static MAX_UUID: &Uuid = &uuid!("ffffffff-ffff-ffff-ffff-ffffffffffff");
/// Special "nil" UUID value
pub static NIL_UUID: &Uuid = &uuid!("00000000-0000-0000-0000-000000000000");

pub fn resolve_product_groups<'a, I>(doc: &impl CsafTrait, product_groups: I) -> Option<BTreeSet<String>>
where
    I: IntoIterator<Item = &'a String>,
{
    let product_groups: Vec<&String> = product_groups.into_iter().collect();

    doc.get_product_tree().as_ref().map(|product_tree| {
        product_tree
            .get_product_groups()
            .iter()
            .filter(|x| product_groups.iter().any(|g| *g == x.get_group_id()))
            .flat_map(|x| x.get_product_ids())
            .map(|p| p.to_string())
            .collect()
    })
}

#[derive(RustEmbed)]
#[folder = "assets/cwe/"]
#[include = "*.csv"]
struct CweCsvFiles;

pub type CweReleaseDateAndData = (NaiveDate, HashMap<String, String>);
pub static CWE_ENTRIES: LazyLock<HashMap<String, CweReleaseDateAndData>> = LazyLock::new(|| {
    let mut entries = HashMap::new();

    for filename in CweCsvFiles::iter() {
        if let Some(file) = CweCsvFiles::get(&filename) {
            let version_and_date = &filename
                .strip_prefix("cwe_")
                .expect("Filenames in assets/cwe should start with 'cwe_'.")
                .strip_suffix(".csv")
                .expect("Filenames in assets/cwe should end with '.csv'.");
            let version_parts = version_and_date.split("_").collect::<Vec<&str>>();
            let version = version_parts[0];
            let release_date = match version_parts[1] {
                "" => NaiveDate::from_ymd_opt(1970, 1, 1).expect("Fallback date should be valid."),
                date_str => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .expect("Date part of filenames in assets/cwe should be in 'YYYY-MM-DD' format."),
            };
            let mut versioned_data: HashMap<String, String> = HashMap::new();
            let content =
                std::str::from_utf8(&file.data).expect("Files in assets/cwe should be valid UTF-8 encoded text files.");
            for line in content.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    let id = format!("CWE-{}", parts[0].trim());
                    let name = parts[1].trim().to_string();
                    versioned_data.insert(id, name);
                }
            }
            entries.insert(version.to_string(), (release_date, versioned_data));
        }
    }

    entries
});

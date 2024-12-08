use std::collections::HashMap;

pub fn find_duplicates<T: std::hash::Hash + Eq + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut occurrences = HashMap::new();
    let mut duplicates = Vec::new();

    for item in vec.iter() {
        let count = occurrences.entry(item.clone()).or_insert(0);
        *count += 1;
    }

    for (item, count) in occurrences {
        if count > 1 {
            duplicates.push(item);
        }
    }

    duplicates
}

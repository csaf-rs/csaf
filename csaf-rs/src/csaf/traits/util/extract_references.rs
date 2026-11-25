use crate::csaf_traits::{WithOptionalGroupIds, WithOptionalProductIds};
use crate::json::JsonPointer;

pub(crate) fn visit_group_references<'a, T: WithOptionalGroupIds + 'a>(
    items: impl Iterator<Item = &'a T>,
    path: &mut JsonPointer<'_>,
    visitor: &mut impl FnMut(&str, &str),
) {
    for (index, item) in items.enumerate() {
        if let Some(ids) = item.get_group_ids() {
            let mut item_path = path.push(index);
            let mut ids_path = item_path.push("group_ids");
            for (id_index, id) in ids.enumerate() {
                visitor(id, ids_path.push(id_index).as_str());
            }
        }
    }
}

pub(crate) fn visit_product_references<'a, T: WithOptionalProductIds + 'a>(
    items: impl Iterator<Item = &'a T>,
    path: &mut JsonPointer<'_>,
    visitor: &mut impl FnMut(&str, &str),
) {
    for (index, item) in items.enumerate() {
        if let Some(ids) = item.get_product_ids() {
            let mut item_path = path.push(index);
            let mut ids_path = item_path.push("product_ids");
            for (id_index, id) in ids.enumerate() {
                visitor(id, ids_path.push(id_index).as_str());
            }
        }
    }
}

/// Generates visitors for reference-bearing collections relative to a supplied pointer.
macro_rules! define_reference_accessors {
    (
        $(vec: [$(($vg:ident, $vp:ident, $vg_get:ident, $vg_path:literal)),* $(,)?],)?
        $(optional_vec: [$(($og:ident, $op:ident, $og_get:ident, $og_path:literal)),* $(,)?],)?
        $(iter: [$(($ig:ident, $ip:ident, $ig_get:ident, $ig_path:literal)),* $(,)?],)?
        $(optional_iter: [$(($oig:ident, $oip:ident, $oig_get:ident, $oig_path:literal)),* $(,)?],)?
        $(custom_group_extraction: [$($cg:ident),* $(,)?],)?
        $(custom_product_extraction: [$($cp:ident),* $(,)?] $(,)?)?
    ) => {
        $($(
            define_reference_accessors!(@methods $vg, $vp, $vg_get, $vg_path, iter);
        )*)?
        $($(
            define_reference_accessors!(@methods $og, $op, $og_get, $og_path, optional);
        )*)?
        $($(
            define_reference_accessors!(@methods $ig, $ip, $ig_get, $ig_path, direct);
        )*)?
        $($(
            define_reference_accessors!(@methods $oig, $oip, $oig_get, $oig_path, optional);
        )*)?
        /// Visits every group reference, preserving field and array order.
        fn visit_all_group_references(&self, path: &mut $crate::json::JsonPointer<'_>, visitor: &mut impl FnMut(&str, &str)) {
            $($(self.$vg(path, visitor);)*)?
            $($(self.$og(path, visitor);)*)?
            $($(self.$ig(path, visitor);)*)?
            $($(self.$oig(path, visitor);)*)?
            $($(self.$cg(path, visitor);)*)?
        }
        /// Visits every product reference, preserving field and array order.
        fn visit_all_product_references(&self, path: &mut $crate::json::JsonPointer<'_>, visitor: &mut impl FnMut(&str, &str)) {
            $($(self.$vp(path, visitor);)*)?
            $($(self.$op(path, visitor);)*)?
            $($(self.$ip(path, visitor);)*)?
            $($(self.$oip(path, visitor);)*)?
            $($(self.$cp(path, visitor);)*)?
        }
    };
    (@items $self:ident, $getter:ident, iter) => { $self.$getter().iter() };
    (@items $self:ident, $getter:ident, optional) => { $self.$getter().into_iter().flatten() };
    (@items $self:ident, $getter:ident, direct) => { $self.$getter() };
    (@methods $group:ident, $product:ident, $getter:ident, $field:literal, $kind:ident) => {
        fn $group(&self, path: &mut $crate::json::JsonPointer<'_>, visitor: &mut impl FnMut(&str, &str)) {
            $crate::csaf::traits::util::extract_references::visit_group_references(
                define_reference_accessors!(@items self, $getter, $kind), &mut path.push($field), visitor);
        }
        fn $product(&self, path: &mut $crate::json::JsonPointer<'_>, visitor: &mut impl FnMut(&str, &str)) {
            $crate::csaf::traits::util::extract_references::visit_product_references(
                define_reference_accessors!(@items self, $getter, $kind), &mut path.push($field), visitor);
        }
    };
}

pub(crate) use define_reference_accessors;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csaf_traits::VulnerabilityTrait;
    use crate::schema::csaf2_1::schema::Vulnerability;
    use serde_json::json;

    #[test]
    fn visitors_restore_prefix_and_preserve_reference_order() {
        let vulnerability: Vulnerability = serde_json::from_value(json!({
            "notes": [{
                "category": "description", "text": "Note",
                "group_ids": ["CSAFGID-0001", "CSAFGID-0001"],
                "product_ids": ["CSAFPID-0001", "CSAFPID-0002"]
            }],
            "metrics": [{"products": ["CSAFPID-0003"], "content": {}}]
        }))
        .expect("vulnerability deserializes");
        let mut buffer = String::from("/vulnerabilities/12");
        let mut path = JsonPointer::new(&mut buffer);
        let mut references = Vec::new();
        vulnerability.visit_all_group_references(&mut path, &mut |id, path| {
            references.push((id.to_owned(), path.to_owned()));
        });
        assert_eq!(path.as_str(), "/vulnerabilities/12");
        assert_eq!(
            references,
            [
                (
                    "CSAFGID-0001".to_owned(),
                    "/vulnerabilities/12/notes/0/group_ids/0".to_owned()
                ),
                (
                    "CSAFGID-0001".to_owned(),
                    "/vulnerabilities/12/notes/0/group_ids/1".to_owned()
                ),
            ]
        );
        references.clear();
        vulnerability.visit_all_product_references(&mut path, &mut |id, path| {
            references.push((id.to_owned(), path.to_owned()));
        });
        assert_eq!(path.as_str(), "/vulnerabilities/12");
        assert_eq!(
            references,
            [
                (
                    "CSAFPID-0001".to_owned(),
                    "/vulnerabilities/12/notes/0/product_ids/0".to_owned()
                ),
                (
                    "CSAFPID-0002".to_owned(),
                    "/vulnerabilities/12/notes/0/product_ids/1".to_owned()
                ),
                (
                    "CSAFPID-0003".to_owned(),
                    "/vulnerabilities/12/metrics/0/products/0".to_owned()
                ),
            ]
        );
    }
}

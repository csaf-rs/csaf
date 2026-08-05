use crate::extractor::{
    extract::ExtractNone,
    traits::{CanExtract, Extractor},
};

#[derive(Clone)]
enum TreeState {
    Main,
    Children,
    Element(usize),
}

type ExtraIntegrator<ElementType, ExtraType> = fn(ElementType, &ExtraType) -> ElementType;

/// An extractor that collects elements from a tree structure.
/// It collects elements defined on any level of the tree,
/// and optionally integrates data along the path to the element.
#[derive(Clone)]
pub struct CollectTree<Element, ElementType, Extra, ExtraType>
where
    Element: CanExtract<ElementType> + Extractor + Clone,
    Extra: CanExtract<ExtraType> + Extractor + Clone,
{
    tree_state: TreeState,
    children: String,
    element_template: Element,
    extra_template: Extra,
    extra_integrator: ExtraIntegrator<ElementType, ExtraType>,
    stack: Vec<(Vec<ElementType>, Element, Extra)>,
}

impl<
    ElementType,
    Element: CanExtract<ElementType> + Extractor + Clone,
    ExtraType,
    Extra: CanExtract<ExtraType> + Extractor + Clone,
> CanExtract<Vec<ElementType>> for CollectTree<Element, ElementType, Extra, ExtraType>
{
    fn extract(&mut self) -> Vec<ElementType> {
        self.finish_layer()
    }
}

impl<ElementType, Element: CanExtract<ElementType> + Extractor + Clone>
    CollectTree<Element, ElementType, ExtractNone, ()>
{
    /// Creates an extractor for data from trees with objects that have an array of children in the field
    /// children_key, and the object to collect in element_key.
    pub fn new(children_key: &str, element_extractor: Element) -> Self {
        CollectTree {
            children: children_key.to_string(),
            element_template: element_extractor,
            extra_template: ExtractNone {},
            extra_integrator: |element, _extra| element,
            tree_state: TreeState::Main,
            stack: Vec::new(),
        }
    }
}

impl<
    ElementType,
    Element: CanExtract<ElementType> + Extractor + Clone,
    ExtraType,
    Extra: CanExtract<ExtraType> + Extractor + Clone,
> CollectTree<Element, ElementType, Extra, ExtraType>
{
    /// Creates an extractor for data from trees with objects that have an array of children in the field
    /// children_key, and the object to collect in element_key. Additionally, data from the field extra_key
    /// along the path is integrated into the result using extra_integrator.
    pub fn new_with_extra(
        children_key: &str,
        element_extractor: Element,
        extra_extractor: Extra,
        extra_integrator: ExtraIntegrator<ElementType, ExtraType>,
    ) -> Self {
        CollectTree {
            children: children_key.to_string(),
            element_template: element_extractor,
            extra_template: extra_extractor,
            extra_integrator,
            tree_state: TreeState::Main,
            stack: Vec::new(),
        }
    }

    fn enter<ForwardFunc: Fn(&mut dyn Extractor) -> bool>(&mut self, name: &str, forward: ForwardFunc) -> bool {
        match self.tree_state {
            TreeState::Main => {
                if name == self.children {
                    self.tree_state = TreeState::Children;
                    true
                } else {
                    self.tree_state = TreeState::Element(0);
                    if let Some((_children, element, extra)) = self.stack.last_mut() {
                        (forward)(element) | (forward)(extra)
                    } else {
                        false
                    }
                }
            },
            TreeState::Children => {
                self.enter_child();
                true
            },
            TreeState::Element(d) => {
                self.tree_state = TreeState::Element(d + 1);
                if let Some((_children, element, extra)) = self.stack.last_mut() {
                    (forward)(element) | (forward)(extra)
                } else {
                    false
                }
            },
        }
    }

    fn leave<ForwardFunc: Fn(&mut dyn Extractor)>(&mut self, forward: ForwardFunc) {
        match self.tree_state {
            TreeState::Main => {
                self.leave_child();
            },
            TreeState::Children => {
                self.tree_state = TreeState::Main;
            },
            TreeState::Element(d) => {
                if d > 0 {
                    if let Some((_children, element, extra)) = self.stack.last_mut() {
                        (forward)(element);
                        (forward)(extra);
                    }
                    self.tree_state = TreeState::Element(d - 1);
                } else {
                    self.tree_state = TreeState::Main;
                }
            },
        }
    }

    fn enter_child(&mut self) {
        self.stack
            .push((vec![], self.element_template.clone(), self.extra_template.clone()));
        self.tree_state = TreeState::Main;
    }

    fn leave_child(&mut self) {
        let mut child_result = self.finish_layer();
        if let Some((children, _, _)) = self.stack.last_mut() {
            children.append(&mut child_result);
        }
        self.tree_state = TreeState::Children;
    }

    fn finish_layer(&mut self) -> Vec<ElementType> {
        let mut ret: Vec<ElementType> = Vec::new();
        if let Some((mut children, mut element, mut extra)) = self.stack.pop() {
            let elem = element.extract();
            let extra = extra.extract();
            ret.push((self.extra_integrator)(elem, &extra));
            for child in std::mem::take(&mut children) {
                ret.push((self.extra_integrator)(child, &extra));
            }
        }
        ret
    }
}

impl<
    ElementType,
    Element: CanExtract<ElementType> + Extractor + Clone,
    ExtraType,
    Extra: CanExtract<ExtraType> + Extractor + Clone,
> Extractor for CollectTree<Element, ElementType, Extra, ExtraType>
{
    fn init_array(&mut self, json_pointer: &str) {
        let mut element = self.element_template.clone();
        element.init_array(json_pointer);
        let mut extra = self.extra_template.clone();
        extra.init_array(json_pointer);
        self.stack.push((vec![], element, extra));
    }

    fn init_object(&mut self, json_pointer: &str) {
        let mut element = self.element_template.clone();
        element.init_object(json_pointer);
        let mut extra = self.extra_template.clone();
        extra.init_object(json_pointer);
        self.stack.push((vec![], element, extra));
    }

    fn init_primitive(&mut self, json_pointer: &str, primitive: &serde_json::Value) {
        let mut element = self.element_template.clone();
        element.init_primitive(json_pointer, primitive);
        let mut extra = self.extra_template.clone();
        extra.init_primitive(json_pointer, primitive);
        self.stack.push((vec![], element, extra));
    }

    fn keyed_primitive(&mut self, json_pointer: &str, name: &str, primitive: &serde_json::Value) {
        self.enter(name, |forward| {
            forward.keyed_primitive(json_pointer, name, primitive);
            false
        });
        self.leave(|_| {});
    }

    fn indexed_primitive(&mut self, json_pointer: &str, index: usize, primitive: &serde_json::Value) {
        self.enter(&index.to_string(), |forward| {
            forward.indexed_primitive(json_pointer, index, primitive);
            false
        });
        self.leave(|_| {});
    }

    fn enter_keyed_object(&mut self, json_pointer: &str, name: &str) -> bool {
        self.enter(name, |forward| forward.enter_keyed_object(json_pointer, name))
    }

    fn leave_keyed_object(&mut self, json_pointer: &str, name: &str) {
        self.leave(|forward| forward.leave_keyed_object(json_pointer, name));
    }

    fn enter_keyed_array(&mut self, json_pointer: &str, name: &str) -> bool {
        self.enter(name, |forward| forward.enter_keyed_array(json_pointer, name))
    }

    fn leave_keyed_array(&mut self, json_pointer: &str, name: &str) {
        self.leave(|forward| forward.leave_keyed_array(json_pointer, name));
    }

    fn enter_indexed_object(&mut self, json_pointer: &str, index: usize) -> bool {
        self.enter(&index.to_string(), |forward| {
            forward.enter_indexed_object(json_pointer, index)
        })
    }

    fn leave_indexed_object(&mut self, json_pointer: &str, index: usize) {
        self.leave(|forward| forward.leave_indexed_object(json_pointer, index));
    }

    fn enter_indexed_array(&mut self, json_pointer: &str, index: usize) -> bool {
        self.enter(&index.to_string(), |forward| {
            forward.enter_indexed_array(json_pointer, index)
        })
    }

    fn leave_indexed_array(&mut self, json_pointer: &str, index: usize) {
        self.leave(|forward| forward.leave_indexed_array(json_pointer, index));
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::extractor::{
        convert::Convert, extract::ExtractPrimitive, navigate::AtPath, visit_json::visit_json_value,
    };

    use super::*;

    #[test]
    fn collect_strings_without_extra() {
        let mut collector = CollectTree::new(
            "children",
            AtPath::new("value", ExtractPrimitive::new_string_with_path()),
        );

        let document = json!({
            "value": "a",
            "children": [
                {"value": "b"},
                {"children": [{"value": "c"}], "value": "d"}
            ]
        });
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(
            result,
            vec![
                Some(("/value".into(), "a".into())),
                Some(("/children/0/value".into(), "b".into())),
                Some(("/children/1/value".into(), "d".into())),
                Some(("/children/1/children/0/value".into(), "c".into()))
            ]
        );
    }

    #[test]
    fn collect_strings_with_breadcrumb() {
        let mut collector = CollectTree::new_with_extra(
            "children",
            Convert::new(AtPath::new("value", ExtractPrimitive::new_string()), |value| {
                (value, vec![])
            }),
            AtPath::new("breadcrumb", ExtractPrimitive::new_string()),
            |(value, mut breadcrumbs), breadcrumb| {
                if let Some(breadcrumb) = &breadcrumb {
                    breadcrumbs.insert(0, breadcrumb.clone());
                }
                (value, breadcrumbs)
            },
        );

        let document = json!({
            "value": "a",
            "breadcrumb": "root",
            "children": [
                {
                    "value": "b",
                    "breadcrumb": "first-child"
                },
                {
                    "children": [
                        {"value": "c"}
                    ],
                    "value": "d",
                    "breadcrumb": "second-child"
                }
            ]
        });
        visit_json_value(&document, &mut [&mut collector]);

        let result = collector.extract();
        assert_eq!(
            result,
            vec![
                (Some("a".to_string()), vec!["root".to_string()]),
                (
                    Some("b".to_string()),
                    vec!["root".to_string(), "first-child".to_string()]
                ),
                (
                    Some("d".to_string()),
                    vec!["root".to_string(), "second-child".to_string()]
                ),
                (
                    Some("c".to_string()),
                    vec!["root".to_string(), "second-child".to_string()]
                )
            ]
        );
    }
}

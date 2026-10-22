use crate::schema::{csaf2_1, loose};

use super::{Convert, Diagnostics, Settings, Warning};

pub struct UpdatePublisherCategory;

impl Convert for UpdatePublisherCategory {
    fn convert(&self, doc: &mut loose::Document, settings: &Settings, diagnostics: &mut dyn Diagnostics) {
        if let Some(category) = doc
            .v21_view_mut()
            .document_mut()
            .and_then(|doc| doc.publisher_mut())
            .and_then(|publisher| publisher.category_mut())
            && let Ok(csaf2_1::schema::CategoryOfPublisher::Other) = category.get_enum()
        {
            if !settings.suppress_publisher_category_other_warning {
                diagnostics.warning("/document/publisher/category".into(), Warning::PublisherCategoryOther);
            }

            if settings.convert_publisher_category_other_to_multiplier {
                category.set_enum(csaf2_1::schema::CategoryOfPublisher::Multiplier);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use crate::converter::tests::{NoDiagnostics, OnlyWarnings};

    use super::*;

    #[rstest]
    #[case::warning_only(false, false, "other", true)]
    #[case::suppress_warning(true, false, "other", false)]
    #[case::convert_to_multiplier(false, true, "multiplier", true)]
    #[case::convert_and_suppress(true, true, "multiplier", false)]
    fn publisher_category_other(
        #[case] suppress_warning: bool,
        #[case] convert_to_multiplier: bool,
        #[case] expected_category: &str,
        #[case] expect_warning: bool,
    ) {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "other",
                    "name": "Example Publisher"
                }
            }
        });

        let expected = json!({
            "document": {
                "publisher": {
                    "category": expected_category,
                    "name": "Example Publisher"
                }
            }
        });

        let settings = Settings {
            suppress_publisher_category_other_warning: suppress_warning,
            convert_publisher_category_other_to_multiplier: convert_to_multiplier,
        };

        let mut doc = loose::Document::new(input);
        let mut diagnostics = OnlyWarnings::default();

        UpdatePublisherCategory.convert(&mut doc, &settings, &mut diagnostics);

        assert_eq!(expected, doc.into_raw());

        if expect_warning {
            assert_eq!(
                diagnostics.warnings(),
                &[("/document/publisher/category".into(), Warning::PublisherCategoryOther)]
            );
        } else {
            assert!(diagnostics.warnings().is_empty());
        }
    }

    #[test]
    fn ignore_other_categories() {
        let input = json!({
            "document": {
                "publisher": {
                    "category": "vendor",
                    "name": "Example Publisher"
                }
            }
        });

        let mut doc = loose::Document::new(input.clone());

        UpdatePublisherCategory.convert(&mut doc, &Settings::default(), &mut NoDiagnostics);

        assert_eq!(input, doc.into_raw());
    }
}

use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::TestFindingData;
use ssvc::{NamespaceError, ParsedNamespace};

/// Returns the JSON instance path to the `namespace` field of an SSVC v2 selection.
pub(crate) fn ssvc_selection_namespace_path(vuln_index: usize, metric_index: usize, selection_index: usize) -> String {
    format!(
        "/vulnerabilities/{vuln_index}/metrics/{metric_index}/content/ssvc_v2/selections/{selection_index}/namespace"
    )
}

/// Returns the JSON instance path to the `decision_point_resources` field of an SSVC v2 metric.
pub(crate) fn ssvc_decision_point_resources_path(vuln_index: usize, metric_index: usize) -> String {
    format!("/vulnerabilities/{vuln_index}/metrics/{metric_index}/content/ssvc_v2/decision_point_resources")
}

/// A single SSVC v2 namespace string with the JSON path at which it was found.
pub(crate) struct SsvcNamespaceAndPath {
    pub namespace: String,
    pub instance_path: String,
}

/// A single SSVC v2 namespace result with the JSON path at which it was found.
pub(crate) struct SsvcNamespaceResultAndPath {
    pub result: Result<ParsedNamespace, NamespaceError>,
    pub instance_path: String,
}

/// Returns an iterator over all SSVC v2 namespace strings and their JSON paths found in the document.
///
/// The iterator skips SSVC metric objects that fail to deserialize (those are reported by test
/// 6.1.46). Every remaining selection yields exactly one [`SsvcNamespaceAndPath`] item.
pub(crate) fn iter_ssvc_namespace_strings<D: CsafTrait>(doc: &D) -> impl Iterator<Item = SsvcNamespaceAndPath> {
    doc.get_vulnerabilities()
        .iter()
        .enumerate()
        .flat_map(|(vuln_index, vuln)| {
            vuln.get_metrics()
                .into_iter()
                .flatten()
                .enumerate()
                .map(move |(metric_index, metric)| (vuln_index, metric_index, metric))
        })
        .flat_map(|(vuln_index, metric_index, metric)| {
            metric
                .get_content()
                .get_ssvc_v2_typed()
                .and_then(Result::ok)
                .into_iter()
                .flat_map(|selection_list| selection_list.selections.into_iter().enumerate())
                .map(move |(selection_index, selection)| SsvcNamespaceAndPath {
                    namespace: selection.namespace.into(),
                    instance_path: ssvc_selection_namespace_path(vuln_index, metric_index, selection_index),
                })
        })
}

// TODO: Consider removing this helper once its remaining consumers (6.2.34, 6.3.14, and 6.3.15) are refactored
//  to consider '/' for base namespace extraction or extension detection.
/// Returns an iterator over all SSVC v2 namespace parse results found in the document.
pub(crate) fn iter_ssvc_namespaces<D: CsafTrait>(
    doc: &D,
    allow_test_namespaces: bool,
) -> impl Iterator<Item = SsvcNamespaceResultAndPath> {
    iter_ssvc_namespace_strings(doc).map(
        move |SsvcNamespaceAndPath {
                  namespace,
                  instance_path,
              }| {
            SsvcNamespaceResultAndPath {
                result: ssvc::validate_namespace(&namespace, allow_test_namespaces),
                instance_path,
            }
        },
    )
}

/// Returns the base namespace by removing all characters including and after the first '/'.
pub(crate) fn ssvc_base_namespace(namespace: &str) -> &str {
    namespace.split_once('/').map_or(namespace, |(base, _)| base)
}

/// Returns a [`TestFindingData`] for a generic SSVC namespace error, using the given JSON instance path.
pub(crate) fn create_generic_namespace_finding_data(err: &NamespaceError, instance_path: &str) -> TestFindingData {
    err.to_finding_data(instance_path.to_owned())
}

/// Extension trait to convert a [`NamespaceError`] into a [`TestFindingData`], attaching the
/// JSON instance path at which the error occurred.
trait NamespaceErrorExt {
    /// Builds a [`TestFindingData`] from this error, using the error's `Display` message and the
    /// given `instance_path`.
    fn to_finding_data(&self, instance_path: String) -> TestFindingData;
}

impl NamespaceErrorExt for NamespaceError {
    fn to_finding_data(&self, instance_path: String) -> TestFindingData {
        TestFindingData {
            message: self.to_string(),
            instance_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_test_finding_data() {
        let err = NamespaceError::ReservedForbiddenNamespace {
            namespace: "invalid".to_string(),
        };
        let data =
            err.to_finding_data("/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/namespace".to_string());

        assert_eq!(data.message, "Reserved forbidden namespace 'invalid' must not be used");
        assert_eq!(
            data.instance_path,
            "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/namespace"
        );
    }
}

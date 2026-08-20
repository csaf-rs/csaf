use crate::validation::{TestFindingData};
use ssvc::NamespaceError;

pub(crate) fn create_other_namespace_error(
    err: &NamespaceError,
    i_v: usize,
    i_m: usize,
    i_s: usize,
) -> TestFindingData {
    err.to_test_finding_data(format!(
        "/vulnerabilities/{i_v}/metrics/{i_m}/content/ssvc_v2/selections/{i_s}/namespace"
    ))
}

/// Extension trait to convert a [`NamespaceError`] into a [`TestFindingData`], attaching the
/// JSON instance path at which the error occurred.
trait NamespaceErrorExt {
    /// Builds a [`TestFindingData`] from this error, using the error's `Display` message and the
    /// given `instance_path`.
    fn to_test_finding_data(&self, instance_path: String) -> TestFindingData;
}

impl NamespaceErrorExt for NamespaceError {
    fn to_test_finding_data(&self, instance_path: String) -> TestFindingData {
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
            err.to_test_finding_data("/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/namespace".to_string());

        assert_eq!(data.message, "Reserved forbidden namespace 'invalid' must not be used");
        assert_eq!(
            data.instance_path,
            "/vulnerabilities/0/metrics/0/content/ssvc_v2/selections/0/namespace"
        );
    }
}

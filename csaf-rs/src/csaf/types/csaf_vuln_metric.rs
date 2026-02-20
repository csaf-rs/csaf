use std::fmt::{Display, Formatter};

/// Types of vulnerability metrics known until CSAF 2.1
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum CsafVulnerabilityMetric {
    SsvcV1,
    CvssV2(String),
    CvssV3(String),
    CvssV4(String),
    Epss,
}

impl CsafVulnerabilityMetric {
    /// Checks if the metric is valid according to the known versions.
    /// Known versions are: CVSS-v2.0, CVSS-v3.0, CVSS-v3.1, CVSS-v4.0.
    /// For SSVC-v1 and EPSS, there are no versions / they are not parsed so far, so they are always considered known.
    pub fn is_known_version(&self) -> bool {
        match self {
            CsafVulnerabilityMetric::SsvcV1 => true,
            CsafVulnerabilityMetric::CvssV2(version) => version == "2.0",
            CsafVulnerabilityMetric::CvssV3(version) => version == "3.0" || version == "3.1",
            CsafVulnerabilityMetric::CvssV4(version) => version == "4.0",
            CsafVulnerabilityMetric::Epss => true,
        }
    }

    /// Returns the property name for the metric, which is used in the JSON representation.
    pub fn get_metric_prop_name(&self) -> &'static str {
        match self {
            CsafVulnerabilityMetric::SsvcV1 => "ssvc_v1",
            CsafVulnerabilityMetric::CvssV2(_) => "cvss_v2",
            CsafVulnerabilityMetric::CvssV3(_) => "cvss_v3",
            CsafVulnerabilityMetric::CvssV4(_) => "cvss_v4",
            CsafVulnerabilityMetric::Epss => "epss",
        }
    }
}

/// Display implementation for VulnerabilityMetrics.
impl Display for CsafVulnerabilityMetric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CsafVulnerabilityMetric::SsvcV1 => write!(f, "SSVC-v1"),
            CsafVulnerabilityMetric::CvssV2(version) => write!(f, "CVSS-v{}", *version),
            CsafVulnerabilityMetric::CvssV3(version) => write!(f, "CVSS-v{}", *version),
            CsafVulnerabilityMetric::CvssV4(version) => write!(f, "CVSS-v{}", *version),
            CsafVulnerabilityMetric::Epss => write!(f, "EPSS"),
        }
    }
}

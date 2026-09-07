use std::fmt::{Display, Formatter};

/// Types of vulnerability metrics known until CSAF 2.1
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum CsafVulnerabilityMetric {
    SsvcV2,
    CvssV2(String),
    CvssV3(String),
    CvssV4(String),
    Epss,
    QualitativeSeverityRating,
}

impl CsafVulnerabilityMetric {
    /// Returns the property name for the metric, which is used in the JSON representation.
    pub fn get_metric_prop_name(&self) -> &'static str {
        match self {
            Self::SsvcV2 => "ssvc_v2",
            Self::CvssV2(_) => "cvss_v2",
            Self::CvssV3(_) => "cvss_v3",
            Self::CvssV4(_) => "cvss_v4",
            Self::Epss => "epss",
            Self::QualitativeSeverityRating => "qualitative_severity_rating",
        }
    }
}

/// Display implementation for VulnerabilityMetrics.
impl Display for CsafVulnerabilityMetric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SsvcV2 => write!(f, "SSVC-v2"),
            Self::CvssV2(version) => write!(f, "CVSS-v{}", *version),
            Self::CvssV3(version) => write!(f, "CVSS-v{}", *version),
            Self::CvssV4(version) => write!(f, "CVSS-v{}", *version),
            Self::Epss => write!(f, "EPSS"),
            Self::QualitativeSeverityRating => write!(f, "Qualitative Severity Rating"),
        }
    }
}

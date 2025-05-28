use std::collections::{HashMap, HashSet};
use crate::csaf::getter_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::csaf::validation::ValidationError;
use std::fmt::{Display, Formatter};
use crate::csaf::validations::test_6_1_07::VulnerabilityMetrics::{CvssV2, CvssV30, CvssV31, CvssV4, SsvcV1, Epss};

/// Types of metrics known until CSAF 2.1
#[derive(Hash, Eq, PartialEq, Clone)]
enum VulnerabilityMetrics {
    SsvcV1,
    CvssV2,
    CvssV30,
    CvssV31,
    CvssV4,
    Epss,
}

impl Display for VulnerabilityMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SsvcV1 => write!(f, "SSVC-v1"),
            CvssV2 => write!(f, "CVSS-v2"),
            CvssV30 => write!(f, "CVSS-v3.0"),
            CvssV31 => write!(f, "CVSS-v3.1"),
            CvssV4 => write!(f, "CVSS-v4"),
            Epss => write!(f, "EPSS"),
        }
    }
}

fn get_metric_prop_name(metric: &VulnerabilityMetrics) -> &'static str {
    match metric {
        SsvcV1 => "ssvc_v1",
        CvssV2 => "cvss_v2",
        CvssV30 => "cvss_v3",
        CvssV31 => "cvss_v3",
        CvssV4 => "cvss_v4",
        Epss => "epss",
    }
}

pub fn test_6_1_07_multiple_same_scores_per_product(
    doc: &impl CsafTrait,
) -> Result<(), ValidationError> {
    for (v_i, v) in doc.get_vulnerabilities().iter().enumerate() {
        let mut seen_metrics: HashMap<String, HashSet<VulnerabilityMetrics>> = HashMap::new();
        if let Some(metrics) = v.get_metrics() {
            for (m_i, m) in metrics.iter().enumerate() {
                let content = m.get_content();
                let mut content_metrics = Vec::<VulnerabilityMetrics>::new();
                if content.has_ssvc_v1() {
                    content_metrics.push(SsvcV1);
                }
                if content.get_cvss_v2().is_some() {
                    content_metrics.push(CvssV2);
                }
                if let Some(cvss_v3) = content.get_cvss_v3() {
                    if let Some(version) = cvss_v3.get("version") {
                        if version == "3.1" {
                            content_metrics.push(CvssV31);
                        } else if version == "3.0" {
                            content_metrics.push(CvssV30);
                        } else {
                            return Err(ValidationError {
                                message: format!("CVSS-v3 version {} is not supported.", version),
                                instance_path: format!(
                                    "{}/{}",
                                    content.get_content_json_path(v_i, m_i),
                                    get_metric_prop_name(&CvssV30),
                                ),
                            });
                        }
                    }
                }
                if content.get_cvss_v4().is_some() {
                    content_metrics.push(CvssV4);
                }
                if content.get_epss().is_some() {
                    content_metrics.push(Epss);
                }
                for p in m.get_products() {
                    let metrics_set = seen_metrics.entry(p.to_string()).or_insert_with(|| HashSet::new());
                    for cm in content_metrics.iter() {
                        if metrics_set.contains(cm) {
                            return Err(ValidationError {
                                message: format!(
                                    "Product {} already has another metric \"{}\" assigned.",
                                    p,
                                    cm,
                                ),
                                instance_path: format!(
                                    "{}/{}",
                                    content.get_content_json_path(v_i, m_i),
                                    get_metric_prop_name(cm)),
                            });
                        } else {
                            metrics_set.insert(cm.to_owned());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::run_csaf21_tests;
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_07::test_6_1_07_multiple_same_scores_per_product;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_07() {
        let cvss_v31_error_message = "Product CSAFPID-9080700 already has another metric \"CVSS-v3.1\" assigned.";
        let cvss_v3_error_path = "/vulnerabilities/0/metrics/1/content/cvss_v3";
        run_csaf21_tests(
            "07",
            test_6_1_07_multiple_same_scores_per_product,
            &HashMap::from([
                ("01", &ValidationError {
                    message: cvss_v31_error_message.to_string(),
                    instance_path: cvss_v3_error_path.to_string()
                }),
                ("02", &ValidationError {
                    message: "Product CSAFPID-9080700 already has another metric \"CVSS-v3.0\" assigned.".to_string(),
                    instance_path: cvss_v3_error_path.to_string()
                }),
                ("03", &ValidationError {
                    message: "Product CSAFPID-9080700 already has another metric \"CVSS-v2\" assigned.".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/1/content/cvss_v2".to_string()
                }),
                ("04", &ValidationError {
                    message: "Product CSAFPID-9080700 already has another metric \"CVSS-v4\" assigned.".to_string(),
                    instance_path: "/vulnerabilities/0/metrics/1/content/cvss_v4".to_string(),
                }),
                ("05", &ValidationError {
                    message: cvss_v31_error_message.to_string(),
                    instance_path: cvss_v3_error_path.to_string(),
                }),
            ]),
        );
    }
}

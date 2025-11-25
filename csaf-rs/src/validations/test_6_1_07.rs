use crate::csaf_traits::{ContentTrait, CsafTrait, MetricTrait, VulnerabilityTrait};
use crate::validation::ValidationError;
use crate::validations::test_6_1_07::VulnerabilityMetrics::{CvssV2, CvssV4, CvssV30, CvssV31, Epss, SsvcV1};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

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

/// Display implementation for VulnerabilityMetrics.
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

/// Returns the name of the metric property for the given metric type.
fn get_metric_prop_name(metric: VulnerabilityMetrics) -> &'static str {
    match metric {
        SsvcV1 => "ssvc_v1",
        CvssV2 => "cvss_v2",
        CvssV30 => "cvss_v3",
        CvssV31 => "cvss_v3",
        CvssV4 => "cvss_v4",
        Epss => "epss",
    }
}

/// Test 6.1.7: Check for multiple identical metric types per vulnerability.
pub fn test_6_1_07_multiple_same_scores_per_product(doc: &impl CsafTrait) -> Result<(), Vec<ValidationError>> {
    let mut errors: Option<Vec<ValidationError>> = None;
    for (vulnerability_index, vulnerability) in doc.get_vulnerabilities().iter().enumerate() {
        let metrics = vulnerability.get_metrics();
        if metrics.is_none() {
            continue;
        }

        let mut product_metrics: HashMap<String, HashMap<VulnerabilityMetrics, Vec<String>>> = HashMap::new();
        for (metric_index, metric) in metrics.unwrap().iter().enumerate() {
            let content = metric.get_content();
            let mut present_metric_types = HashSet::<VulnerabilityMetrics>::new();
            if content.has_ssvc() {
                present_metric_types.insert(SsvcV1);
            }
            if content.get_cvss_v2().is_some() {
                present_metric_types.insert(CvssV2);
            }
            if let Some(cvss_v3) = content.get_cvss_v3() {
                if let Some(version) = cvss_v3.get("version") {
                    if version == "3.1" {
                        present_metric_types.insert(CvssV31);
                    } else if version == "3.0" {
                        present_metric_types.insert(CvssV30);
                    } else {
                        errors.get_or_insert_with(Vec::new).push(ValidationError {
                            message: format!("CVSS-v3 version {} is not supported.", version),
                            instance_path: format!(
                                "{}/{}",
                                content.get_content_json_path(vulnerability_index, metric_index),
                                get_metric_prop_name(CvssV30),
                            ),
                        });
                    }
                }
            }
            if content.get_cvss_v4().is_some() {
                present_metric_types.insert(CvssV4);
            }
            if content.get_epss().is_some() {
                present_metric_types.insert(Epss);
            }
            for p in metric.get_products() {
                for m in present_metric_types.iter() {
                    product_metrics
                        .entry(p.to_owned())
                        .or_insert_with(HashMap::new)
                        .entry(m.to_owned())
                        .or_insert_with(Vec::new)
                        .push(content.get_content_json_path(vulnerability_index, metric_index));
                }
            }
        }

        for (p, metrics_map) in product_metrics.iter() {
            for (metric_type, paths) in metrics_map.iter() {
                if paths.len() > 1 {
                    for path in paths {
                        errors.get_or_insert_with(Vec::new).push(ValidationError {
                            message: create_error_message(metric_type, p),
                            instance_path: format!(
                                "{}/{}",
                                path.to_string(),
                                get_metric_prop_name(metric_type.to_owned())
                            ),
                        });
                    }
                }
            }
        }
    }
    errors.map_or(Ok(()), Err)
}

fn create_error_message(score_type: &VulnerabilityMetrics, product_id: &str) -> String {
    format!(
        "Multiple {} scores are given for {}.",
        score_type.to_string(),
        product_id
    )
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::validation::ValidationError;
    use crate::validations::test_6_1_07::{
        VulnerabilityMetrics, create_error_message, test_6_1_07_multiple_same_scores_per_product,
    };
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_07() {
        let cvss_v31_error_message = create_error_message(&VulnerabilityMetrics::CvssV31, "CSAFPID-9080700");

        run_csaf20_tests(
            "07",
            test_6_1_07_multiple_same_scores_per_product,
            HashMap::from([(
                "01",
                vec![
                    ValidationError {
                        message: cvss_v31_error_message.to_string(),
                        instance_path: "/vulnerabilities/0/scores/0/cvss_v3".to_string(),
                    },
                    ValidationError {
                        message: cvss_v31_error_message.to_string(),
                        instance_path: "/vulnerabilities/0/scores/1/cvss_v3".to_string(),
                    },
                ],
            )]),
        );

        // let csaf_21_path_prefix = "/vulnerabilities/0/metrics/1/content";
        // run_csaf21_tests(
        //     "07",
        //     test_6_1_07_multiple_same_scores_per_product,
        //     HashMap::from([
        //         ("01", vec![ValidationError {
        //             message: cvss_v31_error_message.to_string(),
        //             instance_path: format!("{}/cvss_v3", csaf_21_path_prefix),
        //         }]),
        //         ("02", vec![ValidationError {
        //             message: "Product CSAFPID-9080700 already has another metric \"CVSS-v3.0\" without a source assigned.".to_string(),
        //             instance_path: format!("{}/cvss_v3", csaf_21_path_prefix),
        //         }]),
        //         ("03", vec![ValidationError {
        //             message: "Product CSAFPID-9080700 already has another metric \"CVSS-v2\" without a source assigned.".to_string(),
        //             instance_path: format!("{}/cvss_v2", csaf_21_path_prefix),
        //         }]),
        //         ("04", vec![ValidationError {
        //             message: "Product CSAFPID-9080700 already has another metric \"CVSS-v4\" without a source assigned.".to_string(),
        //             instance_path: format!("{}/cvss_v4", csaf_21_path_prefix),
        //         }]),
        //         ("05", vec![ValidationError {
        //             message: "Product CSAFPID-9080700 already has another metric \"CVSS-v3.1\" with the same source \
        //             \"https://www.example.com/.well-known/csaf/clear/2024/esa-2024-0001.json\" assigned.".to_string(),
        //             instance_path: format!("{}/cvss_v3", csaf_21_path_prefix),
        //         }]),
        //     ]),
        // );
    }
}

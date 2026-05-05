use std::collections::HashMap;

use crate::csaf::types::csaf_vuln_metric::CsafVulnerabilityMetric;
use crate::csaf_traits::{ContentTrait, MetricTrait, VulnerabilityTrait};

/// Maps product ids to a per-metric-type, per-source list of JSON paths.
pub type ProductMetricMap = HashMap<String, ProductCsafVulnerabilityMetricMap>;
/// Maps a metric type (e.g., CVSS v3.1) to a per-source list of JSON paths.
pub type ProductCsafVulnerabilityMetricMap = HashMap<CsafVulnerabilityMetric, ProductCsafVulnerabilityMetricSourceMap>;
/// Maps a source (None for CSAF 2.0 or if None is provided) to a list of JSON paths.
pub type ProductCsafVulnerabilityMetricSourceMap = HashMap<Option<String>, Vec<String>>;

/// Gather all cvss metric entries for a vulnerability into a nested map keyed by
/// product id → metric type → source → JSON paths.
/// More details can be found in [ProductMetricMap], [ProductCsafVulnerabilityMetricMap] and [ProductCsafVulnerabilityMetricSourceMap]
pub fn aggregate_product_cvss_metrics(
    vulnerability: &impl VulnerabilityTrait,
    vulnerability_index: usize,
) -> Option<ProductMetricMap> {
    // return None if there are no metrics
    let metrics = vulnerability.get_metrics()?;

    let mut product_metrics: ProductMetricMap = HashMap::new();
    for (metric_index, metric) in metrics.iter().enumerate() {
        let content = metric.get_content();
        let metric_json_path = content.get_content_json_path(vulnerability_index, metric_index);
        let present_metric_types = content.get_cvss_metric_types();
        for product_id in metric.get_products() {
            for metric_type in &present_metric_types {
                product_metrics
                    .entry(product_id.to_owned())
                    .or_default()
                    .entry(metric_type.to_owned())
                    .or_default()
                    .entry(metric.get_source().map(|s| s.to_owned()))
                    .or_default()
                    .push(metric_json_path.clone());
            }
        }
    }
    Some(product_metrics)
}

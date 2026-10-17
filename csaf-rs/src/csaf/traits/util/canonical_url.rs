use crate::csaf_traits::ReferenceTrait;
use crate::schema::csaf2_1::schema::CategoryOfReference as CategoryOfReference21;

/// Returns an iterator over the reference URLs that satisfy the canonical URL requirements:
/// `category = "self"`, starts with `https://`, has a non-empty hostname,
/// and has the tracking-ID-derived filename preceded by `/`.
pub fn filter_canonical_url_candidates<'a, R: ReferenceTrait>(
    references: Option<&'a Vec<R>>,
    expected_filename: &str,
) -> impl Iterator<Item = &'a str> {
    references
        .into_iter()
        .flatten()
        // using CategoryOfReference21 here is fine, CategoryOfReference20 is 1:1 mapped to this
        .filter(|r| r.get_category() == CategoryOfReference21::Self_)
        .map(|r| r.get_url())
        .filter(move |url| {
            // Check that the URL starts with "https://"
            if let Some(after_scheme) = url.strip_prefix("https://")
                // Split the part after the scheme at the first '/' to get the potential authority part
                && let Some((authority_part, path)) = after_scheme.split_once('/')
                // Check that the authority part is non-empty
                && !authority_part.is_empty()
                // Check that the authority part contains no query or fragment delimiter
                && !authority_part.contains('?')
                && !authority_part.contains('#')
                // Check that the authority part has a non-empty hostname
                && has_non_empty_hostname(authority_part)
                // Check that the last segment of the path matches the expected filename
                && path.rsplit('/').next() == Some(expected_filename)
            {
                true
            } else {
                false
            }
        })
}

/// URI syntax is validated separately by the JSON Schema ('format: "uri"').
/// This helper only checks the additional 6.2.11 CSAF 2.1 requirement that the hostname is non-empty.
fn has_non_empty_hostname(authority: &str) -> bool {
    // Strip optional userinfo
    let hostname_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, hostname_port)| hostname_port);

    // Strip optional port (or the part after the last ':' for IPv6)
    let hostname = hostname_port
        .rsplit_once(':')
        .map_or(hostname_port, |(hostname, _)| hostname);

    // The hostname is non-empty if any character remains after handling userinfo and port
    !hostname.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::csaf2_1::schema::Reference as Reference21;
    use rstest::rstest;
    use serde_json::json;

    /// Reference21 mock, Reference20 is exactly the same though
    fn make_ref21(category: &str, url: &str) -> Reference21 {
        serde_json::from_value(json!({
            "category": category,
            "summary": "x",
            "url": url
        }))
        .unwrap()
    }

    const FILENAME: &str = "example-company-2019-yh3234.json";
    const HTTPS_MATCH: &str = "https://example.com/example-company-2019-yh3234.json";
    const HTTPS_MATCH_WELL_KNOWN: &str = "https://example.com/.well-known/csaf/clear/example-company-2019-yh3234.json";
    const HTTP_MATCH: &str = "http://example.com/example-company-2019-yh3234.json";

    #[rstest]
    // empty
    #[case::no_references(None, 0)]
    // canonical URLs
    #[case::match_simple_path(
        Some(vec![make_ref21("self", HTTPS_MATCH)]),
        1
    )]
    #[case::match_well_known(
        Some(vec![make_ref21("self", HTTPS_MATCH_WELL_KNOWN)]),
        1
    )]
    #[case::userinfo_and_port_with_hostname(
        Some(vec![make_ref21(
        "self",
        "https://user:pass@example.com:443/example-company-2019-yh3234.json"
    )]),
        1
    )]
    #[case::ipv4_hostname(
        Some(vec![make_ref21(
        "self",
        "https://192.168.1.1/example-company-2019-yh3234.json"
    )]),
        1
    )]
    #[case::ipv6_hostname(
        Some(vec![make_ref21(
        "self",
        "https://[2001:db8::1]/example-company-2019-yh3234.json"
    )]),
        1
    )]
    #[case::ipvfuture_hostname(
        Some(vec![make_ref21(
        "self",
        "https://[v1.foo]/example-company-2019-yh3234.json"
    )]),
        1
    )]
    // non-canonical URLs
    #[case::reference_category_external(
        Some(vec![make_ref21("external", HTTPS_MATCH)]),
        0
    )]
    #[case::url_scheme_http(
        Some(vec![make_ref21("self", HTTP_MATCH)]),
        0
    )]
    #[case::url_scheme_ftp(
        Some(vec![make_ref21(
        "self",
        "ftp://example.com/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::wrong_filename(
        Some(vec![make_ref21(
        "self",
        "https://example.com/example-company-2019-yh3235.json"
    )]),
        0
    )]
    #[case::only_domain(
        Some(vec![make_ref21("self", "https://example.com")]),
        0
    )]
    #[case::only_domain_with_delim(
        Some(vec![make_ref21("self", "https://example.com/")]),
        0
    )]
    #[case::empty_authority(
        Some(vec![make_ref21(
        "self",
        "https:///example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::empty_authority_additional_path(
        Some(vec![make_ref21(
        "self",
        "https:////example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::filename_as_authority(
        Some(vec![make_ref21(
        "self",
        "https://example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::empty_hostname_with_query_before_slash(
        Some(vec![make_ref21(
        "self",
        "https://?next=/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::empty_hostname_with_fragment_before_slash(
        Some(vec![make_ref21(
        "self",
        "https://#/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::with_fragment(
        Some(vec![make_ref21(
        "self",
        "https://example.com/example-company-2019-yh3234.json#fragment"
    )]),
        0
    )]
    #[case::with_param(
        Some(vec![make_ref21(
        "self",
        "https://example.com/example-company-2019-yh3234.json?foo=1"
    )]),
        0
    )]
    #[case::userinfo_without_hostname(
        Some(vec![make_ref21(
        "self",
        "https://user:password@/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::port_without_hostname(
        Some(vec![make_ref21(
        "self",
        "https://:443/example-company-2019-yh3234.json"
    )]),
        0
    )]
    #[case::userinfo_and_port_without_hostname(
        Some(vec![make_ref21(
        "self",
        "https://user:password@:443/example-company-2019-yh3234.json"
    )]),
        0
    )]
    fn test_candidate_filtering(#[case] refs: Option<Vec<Reference21>>, #[case] expected_count: usize) {
        let refs_ref = refs.as_ref();
        assert_eq!(
            filter_canonical_url_candidates(refs_ref, FILENAME).count(),
            expected_count
        );
    }

    #[test]
    fn only_matching_refs_are_returned_from_mixed_list() {
        let refs = vec![
            make_ref21("self", HTTPS_MATCH),
            make_ref21("self", HTTP_MATCH),
            make_ref21("external", HTTPS_MATCH),
            make_ref21("self", HTTPS_MATCH_WELL_KNOWN),
        ];

        let result: Vec<_> = filter_canonical_url_candidates(Some(&refs), FILENAME).collect();

        assert_eq!(result, vec![HTTPS_MATCH, HTTPS_MATCH_WELL_KNOWN]);
    }
}

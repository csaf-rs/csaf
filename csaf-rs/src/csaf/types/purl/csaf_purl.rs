pub use crate::csaf::types::purl::purl_error::{PurlParseError, PurlParseErrorKind};
pub use crate::csaf::types::purl::valid_purl::ValidPurl;
use crate::schema::csaf2_0::schema::PackageUrlRepresentation as PackageUrlRepresentation20;
use crate::schema::csaf2_1::schema::PackageUrlRepresentation as PackageUrlRepresentation21;
use packageurl::PackageUrl;
use std::ops::Deref;
use std::str::FromStr;

/// Represents a parsed CSAF PURL that is either valid or invalid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsafPurl {
    /// A successfully parsed and validated PURL.
    Valid(ValidPurl),
    /// A PURL that failed parsing or validation.
    Invalid(PurlParseError),
}

impl CsafPurl {
    fn parse(purl_str: &str) -> CsafPurl {
        match PackageUrl::from_str(purl_str) {
            Ok(mut purl) => {
                let normalized_purl = purl.to_string();
                let base_without_qualifiers = purl.clear_qualifiers().to_string();
                CsafPurl::Valid(ValidPurl::new(
                    purl_str.to_owned(),
                    normalized_purl,
                    base_without_qualifiers,
                ))
            },
            Err(e) => CsafPurl::Invalid(PurlParseError::from_packageurl_error(purl_str, e)),
        }
    }
}

impl From<&PackageUrlRepresentation20> for CsafPurl {
    fn from(purl: &PackageUrlRepresentation20) -> Self {
        CsafPurl::parse(purl.deref())
    }
}

impl From<&PackageUrlRepresentation21> for CsafPurl {
    fn from(purl: &PackageUrlRepresentation21) -> Self {
        CsafPurl::parse(purl.deref())
    }
}

#[cfg(test)]
mod test_purl_regex_parsing {
    use crate::csaf::enums::csaf_version::CsafVersion;
    use crate::schema::csaf2_0::schema::PackageUrlRepresentation as PackageUrlRepresentation20;
    use crate::schema::csaf2_1::schema::PackageUrlRepresentation as PackageUrlRepresentation21;
    use rstest::rstest;
    use std::str::FromStr;

    /// Helper function for the CSAF 2.0 / CSAF 2.1 matrix test. We don't care about the values,
    /// just if the from_str passed / failed.
    fn parse_purl_regex(input: &str, version: &CsafVersion) -> Result<(), ()> {
        match version {
            CsafVersion::X20 => PackageUrlRepresentation20::from_str(input).map(drop).map_err(drop),
            CsafVersion::X21 => PackageUrlRepresentation21::from_str(input).map(drop).map_err(drop),
        }
    }

    #[rstest]
    #[case::missing_scheme("somepackage")]
    #[case::missing_scheme_encoded("pkg%3Amaven%2Flogging")]
    #[case::invalid_scheme("http://maven/logging@1.3.4")]
    #[case::double_slash_after_scheme("pkg://maven/logging@1.3.4")]
    #[case::missing_type("pkg:/somepackage")]
    #[case::invalid_type("pkg:ma%3Fen/somepackage")]
    /// Cases that should be prevented by the CSAF 2.0 and 2.1 regex.
    /// TODO: These also exist as 6.1.13 supplemental test cases. Once we did the return type refactor,
    /// we should move this test into the schema validation tests.
    fn test_invalid_purl_regex(
        #[case] input: &str,
        #[values(CsafVersion::X20, CsafVersion::X21)] version: CsafVersion,
    ) {
        assert!(parse_purl_regex(input, &version).is_err());
    }

    #[rstest]
    #[case::uppercase_type("pkg:Maven/com.example/logging@1.3.4")]
    #[case::plus_in_type("pkg:typ+e/somepackage")]
    /// PURLs that are valid under CSAF 2.0 regex but invalid under the stricter CSAF 2.1 regex.
    /// 2.0 allows uppercase letters and `+` in the type segment; 2.1 does not.
    /// TODO: Same as above, these are schema validation tests implemented here due to lack of schema
    /// validation testing "capability" so far.
    fn test_valid_20_invalid_21_purl_regex(#[case] input: &str) {
        assert!(parse_purl_regex(input, &CsafVersion::X20).is_ok());
        assert!(parse_purl_regex(input, &CsafVersion::X21).is_err());
    }
}

#[cfg(test)]
mod test_purl_full_pipeline {
    use crate::csaf::enums::csaf_version::CsafVersion;
    use crate::csaf::types::purl::{PurlParseError, PurlParseErrorKind};
    use crate::schema::csaf2_0::schema::PackageUrlRepresentation as PackageUrlRepresentation20;
    use crate::schema::csaf2_1::schema::PackageUrlRepresentation as PackageUrlRepresentation21;
    use rstest::rstest;
    use std::str::FromStr;

    use super::*;

    /// Helper function for the CSAF 2.0 / 2.1 matrix test.
    fn to_csaf_purl(purl_str: &str, version: &CsafVersion) -> CsafPurl {
        match version {
            CsafVersion::X20 => {
                let repr = PackageUrlRepresentation20::from_str(purl_str)
                    .expect("Expected purl to pass CSAF 2.0 regex validation, but it failed");
                CsafPurl::from(&repr)
            },
            CsafVersion::X21 => {
                let repr = PackageUrlRepresentation21::from_str(purl_str)
                    .expect("Expected purl to pass CSAF 2.1 regex validation, but it failed");
                CsafPurl::from(&repr)
            },
        }
    }

    #[rstest]
    #[case::missing_name_only_version("pkg:maven/@1.3.4", PurlParseErrorKind::MissingName)]
    #[case::type_prohibits_namespace_oci("pkg:oci/com.example/product-A@sha256%3Add134261219b2", PurlParseErrorKind::TypeProhibitsNamespace("oci".to_string()))]
    #[case::type_prohibits_namespace_cargo("pkg:cargo/somenamespace/somecrate", PurlParseErrorKind::TypeProhibitsNamespace("cargo".to_string()))]
    #[case::type_prohibits_namespace_nuget("pkg:nuget/somenamespace/somepackage", PurlParseErrorKind::TypeProhibitsNamespace("nuget".to_string()))]
    #[case::invalid_key_leading_digit("pkg:maven/somenamespace/lib?1stkey=value", PurlParseErrorKind::InvalidKey("1stkey".to_string()))]
    #[case::invalid_key_with_plus("pkg:maven/somenamespace/lib?bad+key=value", PurlParseErrorKind::InvalidKey("bad+key".to_string()))]
    #[case::invalid_namespace_encoded_slash("pkg:maven/somename%2Fspace/name", PurlParseErrorKind::InvalidNamespaceComponent("somename/space".to_string()))]
    #[case::invalid_subpath_encoded_slash("pkg:maven/somenamespace/name#seg%2Fment", PurlParseErrorKind::InvalidSubpathSegment("seg/ment".to_string()))]
    /// Invalid PURLs that pass the CSAF regex (stage 1) but fail `packageurl` parsing (stage 2).
    /// The regex is a flat structural check (basically `^pkg:TYPE/.+`) and cannot validate:
    /// - decomposed structure (empty name after splitting namespace/name)
    /// - type-specific semantic rules (namespace prohibition)
    /// - qualifier key validity
    /// - percent-decoding results (decoded `/` in namespace/subpath components)
    /// TODO: All of these are to be considered for 6.1.13 supplemental tests.
    fn test_invalid_purl(
        #[case] purl_str: &str,
        #[case] expected_error: PurlParseErrorKind,
        #[values(CsafVersion::X20, CsafVersion::X21)] version: CsafVersion,
    ) {
        let csaf_purl = to_csaf_purl(purl_str, &version);

        let expected = PurlParseError::new_for_test(purl_str, expected_error).into_validation_error(String::new());

        match csaf_purl {
            CsafPurl::Invalid(err) => {
                let actual = err.into_validation_error(String::new());
                assert_eq!(actual, expected);
            },
            CsafPurl::Valid(_) => {
                panic!("Expected purl to fail packageurl validation, but it passed")
            },
        }
    }

    #[rstest]
    // variants from 6.1.41 test data
    #[case::with_namespace_and_version("pkg:maven/com.example/logging@1.3.4")]
    #[case::with_namespace_only("pkg:maven/com.example/product-A")]
    #[case::with_qualifier("pkg:maven/com.example/product-A@1.3.4?classifier=sources")]
    #[case::with_rep_url("pkg:maven/com.example/product-A@1.3.4?repository_url=https://registry.example.com")]
    // variants from 6.2.42 test data
    #[case::deb_with_arch_i386("pkg:deb/debian/curl@7.88.1-10+deb12u12?arch=i386")]
    #[case::deb_with_arch_arm64("pkg:deb/debian/curl@8.13.0-5~bpo12+1+deb12u12?arch=arm64")]
    #[case::deb_with_arch_armel("pkg:deb/debian/curl@7.1.0-5?arch=armel")]
    #[case::deb_without_namespace("pkg:deb/curl@8.13.0-5~bpo12+1?arch=i386")]
    /// Test the happy path with some additional valid purls from the test data.
    fn test_valid_purl(#[case] input: &str, #[values(CsafVersion::X20, CsafVersion::X21)] version: CsafVersion) {
        let csaf_purl = to_csaf_purl(input, &version);
        match csaf_purl {
            CsafPurl::Valid(_) => {},
            CsafPurl::Invalid(err) => panic!(
                "Expected purl to pass packageurl validation, but it failed: {}",
                err.kind()
            ),
        }
    }
}

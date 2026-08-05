/// Defangs a URL by replacing dangerous characters to prevent accidental execution.
/// Replaces:
/// - `https://` with `hXXps[://]`
/// - `http://` with `hXXp[://]`
/// - `.` with `[.]`
pub fn defang_url(url: &str) -> String {
    url.replace("https://", "hXXps[://]")
        .replace("http://", "hXXp[://]")
        .replace(".", "[.]")
}


/// The reason a URL failed to resolve successfully.
#[derive(Debug, PartialEq)]
pub enum UrlResolutionFailure {
    /// The URL resolved, but with a status code the caller-supplied classifier rejected.
    FailedWithStatusCode(u16),
    /// The URL could not be resolved at all (network error, timeout, DNS failure, etc.)
    FailedWithError(String),
}

/// Checks whether a URL resolves by sending a HEAD request, classifying the resulting HTTP
/// status code via `is_success`.
///
/// `is_success` is applied to the returned status code to determine whether it should be
/// treated as `Ok(())` or [`UrlResolutionFailure::FailedWithStatusCode`].
/// If the request fails outright (no connection, network error, timeout, etc.), returns
/// [`UrlResolutionFailure::FailedWithError`].
///
/// Only available when the `external-connections` feature is enabled.
pub fn check_url_resolution(url: &str, is_success: impl Fn(u16) -> bool) -> Result<(), UrlResolutionFailure> {
    match get_status_code(url) {
        Ok(status_code) => {
            if is_success(status_code) {
                Ok(())
            } else {
                Err(UrlResolutionFailure::FailedWithStatusCode(status_code))
            }
        },
        Err(e) => Err(UrlResolutionFailure::FailedWithError(e)),
    }
}

/// Helper function to get the HTTP status code of a URL by sending a HEAD request.
///
/// The request will not follow any redirects.
/// Only available when the `external-connections` feature is enabled.
///
/// Returns the status code as `Ok(u16)`, or `Err` with a description of the failure
/// if there was no status code
fn get_status_code(url: &str) -> Result<u16, String> {
    match ureq::head(url)
        .config()
        .http_status_as_error(false)
        .max_redirects(0)
        .timeout_global(Some(std::time::Duration::from_secs(5)))
        .build()
        .call()
    {
        Ok(response) => Ok(response.status().as_u16()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case(101, "/switching")]
    #[case(200, "/success_200")]
    #[case(201, "/created")]
    #[case(204, "/no-content")]
    #[case(301, "/moved")]
    #[case(400, "/bad-request")]
    #[case(401, "/unauthorized")]
    #[case(403, "/forbidden")]
    #[case(404, "/not-found")]
    #[case(500, "/internal-error")]
    #[case(502, "/bad-gateway")]
    #[case(503, "/unavailable")]
    fn test_get_status_code_http_status(#[case] expected_status: u16, #[case] path: &str) {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::HEAD).path(path);
            then.status(expected_status);
        });

        let url = server.url(path);
        let status = get_status_code(&url);

        assert_eq!(status, Ok(expected_status));
        mock.assert();
    }

    #[test]
    fn test_get_status_code_network_error() {
        // Use a URL to a port that won't respond to simulate network error
        // httpmock doesn't mock the network layer, so we use port 0 on localhost,
        // which will never be occupied
        let url = "http://127.0.0.1:0/test";
        let status = get_status_code(url);

        // Should return Err for network errors
        assert!(status.is_err());
    }
}

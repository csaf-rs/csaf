/// Helper function to get the HTTP status code of a URL by sending a HEAD request.
///
/// The request will not follow any redirects.
/// Only available when the `external-connections` feature is enabled.
///
/// Returns the status code as `Ok(u16)`, or `Err` with a description of the failure
/// if there was no status code
pub(crate) fn get_status_code(url: &str) -> Result<u16, String> {
    // In test builds, callers must explicitly register a mocked outcome for every URL they
    // hit (see mock.rs) — either a pre-defined response (StatusCode/ConnectionFailure),
    // or Passthrough to let the real HTTP call proceed.
    // This panics on unmocked URLs so tests can't accidentally spam / don't depend
    // on real network / DNS access.
    #[cfg(test)]
    match super::url_mock::get_mock_response(url) {
        Some(super::url_mock::MockResponse::StatusCode(code)) => return Ok(code),
        Some(super::url_mock::MockResponse::ConnectionFailure) => {
            return Err("mocked connection failure".to_string());
        },
        Some(super::url_mock::MockResponse::Passthrough) => {},
        None => panic!("mocked response requested for an undefined url"),
    }

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

/// This test module tests the "production"-like get_status_code implementation via wiremock-rs.
/// We control both the requested and hosted url and the time of test execution.
///
/// ureq's blocking call is run on a dedicated thread via "spawn_blocking" so the
/// runtime can keep the mock server responsive. As the mock is thread-bound,
/// the mock also needs to be registered (and cleared) inside `spawn_blocking`
#[cfg(test)]
mod get_status_code_production_tests {
    use super::super::url_mock::{MockResponse, clear_mock_responses, set_mock_response};
    use super::*;
    use rstest::rstest;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

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
    #[tokio::test]
    async fn test_get_status_code_http_status(#[case] expected_status: u16, #[case] path_str: &str) {
        let server = MockServer::start().await;
        Mock::given(method("HEAD"))
            .and(path(path_str))
            .respond_with(ResponseTemplate::new(expected_status))
            .expect(1)
            .mount(&server)
            .await;

        let url = format!("{}{}", server.uri(), path_str);

        let status = tokio::task::spawn_blocking(move || {
            set_mock_response(&url, MockResponse::Passthrough);
            let result = get_status_code(&url);
            clear_mock_responses();
            result
        })
        .await
        .unwrap();

        assert_eq!(status, Ok(expected_status));
    }

    #[tokio::test]
    async fn test_get_status_code_network_error() {
        let server = MockServer::start().await;
        Mock::given(method("HEAD"))
            .respond_with_err(|_req: &wiremock::Request| {
                std::io::Error::new(std::io::ErrorKind::ConnectionReset, "simulated network error")
            })
            .mount(&server)
            .await;

        let status = tokio::task::spawn_blocking(move || {
            set_mock_response(&server.uri(), MockResponse::Passthrough);
            let result = get_status_code(&server.uri());
            clear_mock_responses();
            result
        })
        .await
        .unwrap();

        assert!(status.is_err());
    }
}

/// This test module tests the "test" get_status_code implementation via our custom mock.rs.
///
/// This assumes we control only the hosted url's and have no control over the requested url's.
#[cfg(test)]
mod get_status_code_mock_tests {
    use super::super::url_mock::{MockResponse, clear_mock_responses, set_mock_response};
    use super::*;

    #[test]
    fn test_get_status_code_returns_mocked_status_code() {
        set_mock_response("https://example.net", MockResponse::StatusCode(200));

        assert_eq!(get_status_code("https://example.net"), Ok(200));

        clear_mock_responses();
    }

    #[test]
    fn test_get_status_code_returns_mocked_connection_failure() {
        set_mock_response("https://example.invalid", MockResponse::ConnectionFailure);

        assert_eq!(
            get_status_code("https://example.invalid"),
            Err("mocked connection failure".to_string())
        );

        clear_mock_responses();
    }

    /// Confirms `MockResponse::Passthrough` lets the real HTTP call proceed for a specific
    /// registered URL.
    #[tokio::test]
    async fn test_get_status_code_passthrough_uses_real_http_call() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("HEAD"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let url = format!("{}/passthrough", server.uri());
        let status = tokio::task::spawn_blocking(move || {
            set_mock_response(&url, MockResponse::Passthrough);
            let result = get_status_code(&url);
            clear_mock_responses();
            result
        })
        .await
        .unwrap();

        assert_eq!(status, Ok(200));
    }

    #[test]
    #[should_panic(expected = "mocked response requested for an undefined url")]
    fn test_get_status_code_panics_for_unmocked_url() {
        let _ = get_status_code("https://example.unregistered");
    }
}

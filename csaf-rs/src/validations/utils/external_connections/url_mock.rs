//! Test-only mechanism to stub out the outcome of [`super::get_status_code::get_status_code`]
//! for specific URLs used in test fixtures.
//!
//! This is needed for tests that validate against upstream test cases
//! containing hardcoded external-pointing URLs (such as `https://example.invalid`). Since we can't
//! rewrite those URLs to point at a local mock server, we instead let tests register a pre-defined
//! outcome per URL, which `get_status_code` checks for and returns if present before making any real HTTP request.
//!
//! Mock state is stored in a [`thread_local`] rather than a process-wide global. `cargo test` runs
//! each `#[test]` function on its own thread by default, so this keeps a test's mocked responses
//! scoped precisely to that test and prevents leakage into other tests running concurrently.

use std::cell::RefCell;
use std::collections::HashMap;

/// A pre-defined outcome for a mocked URL.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MockResponse {
    /// Simulate a HEAD request that returned the given HTTP status code.
    StatusCode(u16),
    /// Simulate a network/connection failure (no status code available).
    ConnectionFailure,
    /// Don't stub the outcome; let the real HTTP call for this URL go through as normal.
    Passthrough,
}

thread_local! {
    static MOCK_RESPONSES: RefCell<HashMap<String, MockResponse>> = RefCell::new(HashMap::new());
}

/// Registers a canned response for `url`, overriding the real network call in
/// `get_status_code` for the remainder of the calling test (or until [`clear_mock_responses`] is
/// called).
pub(crate) fn set_mock_response(url: &str, response: MockResponse) {
    MOCK_RESPONSES.with_borrow_mut(|responses| responses.insert(url.to_string(), response));
}

/// Looks up a previously registered canned response for `url`, if any exists.
pub(crate) fn get_mock_response(url: &str) -> Option<MockResponse> {
    MOCK_RESPONSES.with_borrow(|responses| responses.get(url).cloned())
}

/// Clears all registered mock responses. Tests should call this once done to avoid leaking
/// state into other tests that happen to reuse the same thread.
#[allow(dead_code)]
pub(crate) fn clear_mock_responses() {
    MOCK_RESPONSES.with_borrow_mut(|responses| responses.clear());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_mock_response() {
        set_mock_response("https://example.net", MockResponse::StatusCode(200));
        set_mock_response("https://example.invalid", MockResponse::ConnectionFailure);

        assert_eq!(
            get_mock_response("https://example.net"),
            Some(MockResponse::StatusCode(200))
        );
        assert_eq!(
            get_mock_response("https://example.invalid"),
            Some(MockResponse::ConnectionFailure)
        );

        assert_eq!(get_mock_response("https://example.unregistered"), None);

        clear_mock_responses();

        assert_eq!(get_mock_response("https://example.net"), None);
        assert_eq!(get_mock_response("https://example.invalid"), None);
    }
}

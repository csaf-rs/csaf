pub(crate) mod get_status_code;
#[cfg(test)]
pub(crate) mod url_mock;

use get_status_code::get_status_code;

/// The reason a URL failed to resolve successfully.
#[derive(Debug, Clone, PartialEq)]
pub enum UrlResolutionFailure {
    /// The URL resolved, but with a status code the caller-supplied classifier rejected.
    FailedWithStatusCode(u16),
    /// The URL could not be resolved at all (network error, timeout, DNS failure, etc.)
    FailedWithError(String),
}

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

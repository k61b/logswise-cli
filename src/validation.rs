//! Simple validation utilities.

/// Validate if a URL is properly formatted.
pub fn validate_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Validate if an API key is not empty and has reasonable length.
pub fn validate_api_key(api_key: &str) -> bool {
    !api_key.trim().is_empty() && api_key.len() > 10
}

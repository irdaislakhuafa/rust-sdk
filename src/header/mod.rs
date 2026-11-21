// header module (port of go-sdk/header/header.go)
//
// Standard HTTP header keys and common header values used across the SDK.
// Mirrors the constants defined in `go-sdk/header/header.go`.

/// Header keys (lowercased, matching HTTP canonical form).
pub const KEY_REQUEST_ID: &str = "x-request-id";
pub const KEY_AUTHORIZATION: &str = "authorization";
pub const KEY_USER_AGENT: &str = "user-agent";
pub const KEY_CONTENT_TYPE: &str = "content-type";
pub const KEY_CONTENT_ACCEPT: &str = "accept";
pub const KEY_ACCEPT_LANGUAGE: &str = "accept-language";
pub const KEY_CACHE_CONTROL: &str = "cache-control";

/// Content types — specifying the payload in the request.
pub const CONTENT_TYPE_JSON: &str = "application/json";
pub const CONTENT_TYPE_XML: &str = "application/xml";
pub const CONTENT_TYPE_FORM: &str = "application/x-www-form-urlencoded";
pub const CONTENT_TYPE_HTML: &str = "text/html";
pub const CONTENT_TYPE_PDF: &str = "application/pdf";

/// Accepting media — specifying the types of requested media (in the response).
/// See: https://en.wikipedia.org/wiki/Content_negotiation
pub const MEDIA_TEXT_PLAIN: &str = "text/plain";
pub const MEDIA_TEXT_HTML: &str = "text/html";
pub const MEDIA_TEXT_CSV: &str = "text/csv";
pub const MEDIA_TEXT_XML: &str = "text/xml";

pub const MEDIA_IMAGE_GIF: &str = "image/gif";
pub const MEDIA_IMAGE_JPEG: &str = "image/jpeg";
pub const MEDIA_IMAGE_PNG: &str = "image/png";
pub const MEDIA_IMAGE_WEBP: &str = "image/webp";

/// Cache control directives.
pub const CACHE_CONTROL_NO_CACHE: &str = "no-cache";
pub const CACHE_CONTROL_NO_STORE: &str = "no-store";

/// Content encoding.
pub const CONTENT_ENCODING_UTF8: &str = "UTF-8";

#[cfg(test)]
mod header_test;

use super::*;
use crate::http::status::*;

#[test]
fn test_constants() {
    assert_eq!(ENGLISH, "EN");
    assert_eq!(INDONESIAN, "ID");
    assert_eq!(JAPANESE, "JP");
    assert_eq!(DEUSTCH, "DE");
}

#[test]
fn test_from_str() {
    assert_eq!(from_str("EN"), Some(ENGLISH));
    assert_eq!(from_str("ID"), Some(INDONESIAN));
    assert_eq!(from_str("XX"), None);
}

#[test]
fn test_http_status_text() {
    assert_eq!(http_status_text(ENGLISH, STATUS_OK), "OK");
    assert_eq!(
        http_status_text(INDONESIAN, STATUS_NOT_FOUND),
        "Tidak Ditemukan"
    );
    // JP fallback to EN
    assert_eq!(
        http_status_text(JAPANESE, STATUS_INTERNAL_SERVER_ERROR),
        "Internal Server Error"
    );
    // DE fallback to EN
    assert_eq!(
        http_status_text(DEUSTCH, STATUS_INTERNAL_SERVER_ERROR),
        "Internal Server Error"
    );
    // unknown code
    assert_eq!(http_status_text(ENGLISH, 999), "");
}

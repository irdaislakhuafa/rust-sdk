#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_header_keys() {
        assert_eq!(KEY_REQUEST_ID, "x-request-id");
        assert_eq!(KEY_AUTHORIZATION, "authorization");
        assert_eq!(KEY_USER_AGENT, "user-agent");
        assert_eq!(KEY_CONTENT_TYPE, "content-type");
        assert_eq!(KEY_CONTENT_ACCEPT, "accept");
        assert_eq!(KEY_ACCEPT_LANGUAGE, "accept-language");
        assert_eq!(KEY_CACHE_CONTROL, "cache-control");
    }

    #[test]
    fn test_content_types() {
        assert_eq!(CONTENT_TYPE_JSON, "application/json");
        assert_eq!(CONTENT_TYPE_XML, "application/xml");
        assert_eq!(CONTENT_TYPE_FORM, "application/x-www-form-urlencoded");
        assert_eq!(CONTENT_TYPE_HTML, "text/html");
        assert_eq!(CONTENT_TYPE_PDF, "application/pdf");
    }

    #[test]
    fn test_media_types() {
        assert_eq!(MEDIA_TEXT_PLAIN, "text/plain");
        assert_eq!(MEDIA_TEXT_HTML, "text/html");
        assert_eq!(MEDIA_TEXT_CSV, "text/csv");
        assert_eq!(MEDIA_TEXT_XML, "text/xml");
        assert_eq!(MEDIA_IMAGE_GIF, "image/gif");
        assert_eq!(MEDIA_IMAGE_JPEG, "image/jpeg");
        assert_eq!(MEDIA_IMAGE_PNG, "image/png");
        assert_eq!(MEDIA_IMAGE_WEBP, "image/webp");
    }

    #[test]
    fn test_cache_control() {
        assert_eq!(CACHE_CONTROL_NO_CACHE, "no-cache");
        assert_eq!(CACHE_CONTROL_NO_STORE, "no-store");
    }

    #[test]
    fn test_content_encoding() {
        assert_eq!(CONTENT_ENCODING_UTF8, "UTF-8");
    }

    #[test]
    fn test_header_keys_are_lowercase() {
        for key in [
            KEY_REQUEST_ID,
            KEY_AUTHORIZATION,
            KEY_USER_AGENT,
            KEY_CONTENT_TYPE,
            KEY_CONTENT_ACCEPT,
            KEY_ACCEPT_LANGUAGE,
            KEY_CACHE_CONTROL,
        ] {
            assert_eq!(key, key.to_lowercase(), "header key must be lowercase: {key}");
        }
    }
}

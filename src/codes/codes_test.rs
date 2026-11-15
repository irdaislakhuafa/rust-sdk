#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::language;

    #[test]
    fn test_compile_success_english() {
        let msg = compile(Code::Success, language::ENGLISH);
        assert_eq!(msg.status_code, 200);
        assert_eq!(msg.title, language::http_status_text(language::ENGLISH, 200).to_string());
        assert_eq!(msg.body, "Request successful");
    }

    #[test]
    fn test_compile_success_indonesian() {
        let msg = compile(Code::Success, language::INDONESIAN);
        assert_eq!(msg.status_code, 200);
        assert_eq!(msg.body, "Request berhasil");
    }

    #[test]
    fn test_compile_unauthorized_english() {
        let msg = compile(Code::Unauthorized, language::ENGLISH);
        assert_eq!(msg.status_code, 401);
        assert_eq!(msg.body, "Unauthorized access. You are not authorized to access this resource.");
    }

    #[test]
    fn test_compile_unauthorized_indonesian() {
        let msg = compile(Code::Unauthorized, language::INDONESIAN);
        assert_eq!(msg.status_code, 401);
        assert_eq!(msg.body, "Akses ditolak. Anda tidak memilik izin untuk mengakses sumber daya ini.");
    }

    #[test]
    fn test_compile_not_found() {
        let msg = compile(Code::NotFound, language::ENGLISH);
        assert_eq!(msg.status_code, 404);
        assert_eq!(msg.body, "Record doesn't exist. Please validate your input or contact the administrator.");
    }

    #[test]
    fn test_compile_conflict() {
        let msg = compile(Code::Conflict, language::ENGLISH);
        assert_eq!(msg.status_code, 409);
        assert_eq!(msg.body, "Record has existed. Please validate your input or contact the administrator.");
    }

    #[test]
    fn test_compile_internal_server_error() {
        let msg = compile(Code::InternalServerError, language::ENGLISH);
        assert_eq!(msg.status_code, 500);
        assert_eq!(msg.body, "Internal server error. Please contact the administrator.");
    }

    #[test]
    fn test_compile_fallback_unknown_code() {
        let msg = compile(Code::NoCode, language::ENGLISH);
        assert_eq!(msg.status_code, 200);
        assert_eq!(msg.body, "Request successful");
    }

    #[test]
    fn test_get_code_messages_success() {
        let msgs = get_code_messages(Code::Success);
        assert!(msgs.contains_key(&language::ENGLISH));
        assert!(msgs.contains_key(&language::INDONESIAN));
    }

    #[test]
    fn test_get_code_messages_not_found() {
        let msgs = get_code_messages(Code::NoCode);
        assert!(msgs.is_empty());
    }

    #[test]
    fn test_compile_fallback_language() {
        // If language not available, fallback to English Success
        let msg = compile(Code::Success, language::JAPANESE);
        assert_eq!(msg.status_code, 200);
        assert_eq!(msg.body, "Request successful");
    }
}
#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_is_between() {
        let code = Code::BadRequest;
        assert!(code.is_between(Code::BadRequest, Code::NotFound));
        assert!(!code.is_between(Code::NotFound, Code::Conflict));
    }

    #[test]
    fn test_is_one_of() {
        let code = Code::NotFound;
        let list = [Code::BadRequest, Code::NotFound, Code::Conflict];
        assert!(code.is_one_of(&list));
        assert!(!Code::Unauthorized.is_one_of(&list));
    }

    #[test]
    fn test_is_not_one_of() {
        let code = Code::NotFound;
        let list = [Code::BadRequest, Code::Conflict];
        assert!(code.is_not_one_of(&list));
        assert!(!Code::BadRequest.is_not_one_of(&list));
    }
}
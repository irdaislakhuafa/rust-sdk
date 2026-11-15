use super::*;
use crate::codes::Code;
use crate::language;

#[test]
fn test_app_error_creation_and_conversion() {
    let err = App::new(Code::BadRequest, "Bad Request", "Bad Request", None);
    assert_eq!(err.code, Code::BadRequest);
    assert_eq!(err.title, "Bad Request");

    let boxed: Box<dyn Error + Send + Sync> = Box::new(err);
    let converted = App::safe_convert(&*boxed).unwrap();
    assert_eq!(converted.code, Code::BadRequest);
    assert_eq!(converted.title, "Bad Request");
}

#[test]
fn test_get_caller_success() {
    let err = new_with_code(Code::BadRequest, "bad request");
    let res = get_caller(err);
    assert!(res.is_ok());
    let (file, line, message) = res.unwrap();
    assert_eq!(message, "bad request");
    assert!(!file.is_empty());
    assert!(line > 0);
    assert_eq!(line, 19);
}

#[test]
fn test_get_caller_failure() {
    let pure: Box<dyn Error + Send + Sync> =
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, "pure error"));
    let res = get_caller(pure);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "pure error");
}

#[test]
fn test_get_caller_opt_none() {
    let res = get_caller_opt(None);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        "failed to cast error to stacktrace"
    );
}

#[test]
fn test_compile_auth_failure() {
    let err = new_with_code(Code::AuthFailure, "authentication failure");
    let (status, app) = compile(err, language::ENGLISH);
    assert_eq!(status, 401);
    assert_eq!(app.code, Code::AuthFailure);
}

#[test]
fn test_compile_no_code() {
    let err = new_with_code(Code::NoCode, "no code");
    let (status, app) = compile(err, language::ENGLISH);
    assert_eq!(status, 500);
    assert_eq!(app.code, Code::NoCode);
    assert_eq!(app.title, "Service Error Not Defined");
    assert_eq!(app.body, "Unknown error. Please contact admin");
}

#[test]
fn test_get_code_on_stacktrace() {
    let err = new_with_code(Code::NotFound, "missing");
    assert_eq!(get_code(err.as_ref()), Code::NotFound);
}

#[test]
fn test_get_code_on_non_stacktrace() {
    let pure: Box<dyn Error + Send + Sync> =
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, "x"));
    assert_eq!(get_code(pure.as_ref()), Code::NoCode);
}

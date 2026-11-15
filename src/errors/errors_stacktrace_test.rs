use super::*;
use crate::codes::Code;

#[test]
fn test_stacktrace_error_message() {
    let st = Stacktrace {
        message: "failed to start".to_string(),
        cause: None,
        code: Code::NoCode,
        file: String::new(),
        function: String::new(),
        line: 0,
    };
    assert_eq!(st.error(), "failed to start");
    assert_eq!(st.to_string(), "failed to start");
}

#[test]
fn test_stacktrace_exit_code() {
    let st = Stacktrace {
        message: String::new(),
        cause: None,
        code: Code::NoCode,
        file: String::new(),
        function: String::new(),
        line: 0,
    };
    assert_eq!(st.exit_code(), 1);

    let st2 = Stacktrace {
        message: String::new(),
        cause: None,
        code: Code::AuthFailure,
        file: String::new(),
        function: String::new(),
        line: 0,
    };
    assert_eq!(st2.exit_code(), Code::AuthFailure.value() as i64);
}

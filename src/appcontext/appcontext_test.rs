use crate::appcontext::{self, Interface};
use crate::language;

#[test]
fn test_app_context_contract() {
    let now = std::time::SystemTime::now();
    let mut ctx = appcontext::AppContext::new();

    // assert the default value
    assert_eq!(ctx.get_accept_language(), language::ENGLISH);
    assert_eq!(ctx.get_request_id(), "");
    assert_eq!(ctx.get_service_version(), "");
    assert_eq!(ctx.get_user_agent(), "");
    assert_eq!(
        ctx.get_request_start_time(),
        std::time::SystemTime::UNIX_EPOCH
    );

    // assert setters and getters work
    ctx.set_accept_language(language::INDONESIAN)
        .set_request_id("this_is_request_id")
        .set_service_version("v0.0.1")
        .set_user_agent("linux")
        .set_request_start_time(now);

    assert_eq!(ctx.get_accept_language(), language::INDONESIAN);
    assert_eq!(ctx.get_request_id(), "this_is_request_id");
    assert_eq!(ctx.get_service_version(), "v0.0.1");
    assert_eq!(ctx.get_user_agent(), "linux");
    assert_eq!(ctx.get_request_start_time(), now);
}

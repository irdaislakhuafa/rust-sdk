use super::*;

#[test]
fn test_status_constants() {
    assert_eq!(STATUS_CONTINUE, 100);
    assert_eq!(STATUS_OK, 200);
    assert_eq!(STATUS_CREATED, 201);
    assert_eq!(STATUS_BAD_REQUEST, 400);
    assert_eq!(STATUS_NOT_FOUND, 404);
    assert_eq!(STATUS_TEAPOT, 418);
    assert_eq!(STATUS_UNPROCESSABLE_ENTITY, 422);
    assert_eq!(STATUS_TOO_MANY_REQUESTS, 429);
    assert_eq!(STATUS_INTERNAL_SERVER_ERROR, 500);
    assert_eq!(STATUS_NETWORK_AUTHENTICATION_REQUIRED, 511);
}

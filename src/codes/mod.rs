use crate::language::Language;
use crate::language;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum Code {
    NoCode = 0,
    Success = 1,

    // Common error codes
    InvalidValue,
    ContextDeadlineExceeded,
    ContextCanceled,
    InternalServerError,
    ServerUnavailable,
    NotImplemented,
    BadRequest,
    NotFound,
    Conflict,
    Unauthorized,
    TooManyRequest,
    RequestEntityTooLarge,
    Marshal,
    Unmarshal,

    // SQL error codes
    SqlStart,
    Sql,
    SqlInit,
    SqlBuilder,
    SqlTxBegin,
    SqlTxCommit,
    SqlTxRollback,
    SqlTxExec,
    SqlPrepareStmt,
    SqlRead,
    SqlRowScan,
    SqlRecordDoesNotExist,
    SqlUniqueConstraint,
    SqlConflict,
    SqlNoRowsAffected,
    SqlEnd,

    // Client error codes
    ClientStart,
    Client,
    ClientMarshal,
    ClientUnmarshal,
    ClientErrorOnRequest,
    ClientErrorOnReadBody,
    ClientEnd,

    // Auth error codes
    AuthStart,
    Auth,
    AuthRefreshTokenExpired,
    AuthAccessTokenExpired,
    AuthFailure,
    AuthInvalidToken,
    Forbidden,
    AuthEnd,

    // JSON encoding/decoding error codes
    JsonSchemaStart,
    JsonSchema,
    JsonSchemaInvalid,
    JsonSchemaNotFound,
    JsonStructInvalid,
    JsonRawInvalid,
    JsonValidationError,
    JsonMarshalError,
    JsonUnmarshalError,
    JsonSchemaEnd,

    // Storage error codes
    StorageStart,
    Storage,
    StorageNoFile,
    StorageGenerateUrlFailure,
    StorageNoClient,
    StorageDelFailure,
    StorageEnd,

    // JWT error codes
    JwtStart,
    Jwt,
    JwtInvalidMethod,
    JwtParseWithClaimsError,
    JwtInvalidClaimsType,
    JwtSignedStringError,
    JwtEnd,

    // GraphQL error codes
    GqlStart,
    Gql,
    GqlInvalidValue,
    GqlBuilder,
    GqlEnd,

    // Argon2 error codes
    Argon2Start,
    Argon2,
    Argon2InvalidEncodedHash,
    Argon2EncodeHashError,
    Argon2DecodeHashError,
    Argon2IncompatibleVersion,
    Argon2NotMatch,
    Argon2End,

    // Bcrypt error codes
    BcryptStart,
    Bcrypt,
    BcryptEnd,

    // AES 256 GCM error codes
    Aes256GcmStart,
    Aes256Gcm,
    Aes256GcmOpenError,
    Aes256GcmEnd,

    // SMTP error codes
    SmtpStart,
    Smtp,
    SmtpBadRequest,
    SmtpRequestTimeout,
    SmtpEnd,

    // Identiface error codes
    IdentifaceStart,
    Identiface,
    IdentifaceNoFaceDetected,
    IdentifaceFaceNotRecognized,
    IdentifaceMultipleFaceDetected,
    IdentifaceFaceNotRegistered,
    IdentifaceEnd,

    // String Template codes
    StrTemplateStart,
    StrTemplateInvalidFormat,
    StrTemplateExecuteErr,
    StrTemplateEnd,

    // Queue codes
    QueueEmpty,
    QueueFull,

    // Cache codes
    CacheStart,
    CacheKeyNotFound,
    CacheAddErr,
    CacheDelErr,
    CacheReadErr,
    CacheEnd,

    // Rate limiter codes
    RateLimiterStart,
    RateLimiterRead,
    RateLimiterExceeded,
    RateLimiterResetFailed,
    RateLimiterEnd,

    // Log codes
    LogStart,
    Log,
    LogInvalidLevel,
    LogInitFailure,
    LogWriteFailure,
    LogEnd,

    // End of sdk code
    End,
}

impl Code {
    pub fn value(&self) -> u64 {
        *self as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub status_code: u16,
    pub title: String,
    pub body: String,
}

pub trait CodeExt {
    fn is_between(&self, start_code: Code, end_code: Code) -> bool;
    fn is_one_of(&self, list_codes: &[Code]) -> bool;
    fn is_not_one_of(&self, list_codes: &[Code]) -> bool;
}

impl CodeExt for Code {
    fn is_between(&self, start_code: Code, end_code: Code) -> bool {
        self.value() >= start_code.value() && self.value() <= end_code.value()
    }

    fn is_one_of(&self, list_codes: &[Code]) -> bool {
        list_codes.contains(self)
    }

    fn is_not_one_of(&self, list_codes: &[Code]) -> bool {
        !self.is_one_of(list_codes)
    }
}

pub fn get_code_messages(code: Code) -> HashMap<language::Language, Message> {
    match code {
        Code::InvalidValue | Code::BadRequest | Code::Marshal | Code::Unmarshal | Code::JsonMarshalError | Code::JsonUnmarshalError => {
            get_messages(messages::MsgCode::ErrBadRequest)
        }
        Code::ContextDeadlineExceeded | Code::ContextCanceled => {
            get_messages(messages::MsgCode::ErrContextTimeout)
        }
        Code::InternalServerError | Code::Sql | Code::SqlInit | Code::SqlBuilder | Code::SqlTxBegin | Code::SqlTxCommit | Code::SqlTxRollback | Code::SqlTxExec | Code::SqlPrepareStmt | Code::SqlRead | Code::SqlRowScan | Code::ClientMarshal | Code::ClientUnmarshal | Code::ClientErrorOnRequest | Code::ClientErrorOnReadBody => {
            get_messages(messages::MsgCode::ErrInternalServerError)
        }
        Code::ServerUnavailable => {
            get_messages(messages::MsgCode::ErrServiceUnavailable)
        }
        Code::NotImplemented => {
            get_messages(messages::MsgCode::ErrNotImplemented)
        }
        Code::NotFound | Code::SqlRecordDoesNotExist | Code::SqlNoRowsAffected => {
            get_messages(messages::MsgCode::ErrNotFound)
        }
        Code::Conflict | Code::SqlUniqueConstraint | Code::SqlConflict => {
            get_messages(messages::MsgCode::ErrConflict)
        }
        Code::Unauthorized | Code::Auth | Code::AuthFailure => {
            get_messages(messages::MsgCode::ErrUnauthorized)
        }
        Code::TooManyRequest => {
            get_messages(messages::MsgCode::ErrTooManyRequest)
        }
        Code::RequestEntityTooLarge => {
            get_messages(messages::MsgCode::ErrRequestEntityTooLarge)
        }
        Code::AuthRefreshTokenExpired => {
            get_messages(messages::MsgCode::ErrRefreshTokenExpired)
        }
        Code::AuthAccessTokenExpired => {
            get_messages(messages::MsgCode::ErrAccessTokenExpired)
        }
        Code::AuthInvalidToken => {
            get_messages(messages::MsgCode::ErrInvalidToken)
        }
        Code::Forbidden => {
            get_messages(messages::MsgCode::ErrForbidden)
        }
        Code::Success => {
            get_messages(messages::MsgCode::SuccessDefault)
        }
        Code::LogInvalidLevel => {
            get_messages(messages::MsgCode::ErrBadRequest)
        }
        Code::LogInitFailure | Code::LogWriteFailure => {
            get_messages(messages::MsgCode::ErrInternalServerError)
        }
        _ => HashMap::new(),
    }
}

pub fn compile(code: Code, lang: Language) -> Message {
    let msgs = get_code_messages(code);
    if let Some(msg) = msgs.get(&lang) {
        return msg.clone();
    }

    let success_msgs = get_code_messages(Code::Success);
    if let Some(msg) = success_msgs.get(&language::ENGLISH) {
        return msg.clone();
    }

    Message {
        status_code: 200,
        title: String::new(),
        body: String::new(),
    }
}

pub mod messages;
pub use messages::*;
#[cfg(test)]
mod code_func_test;
#[cfg(test)]
mod codes_test;
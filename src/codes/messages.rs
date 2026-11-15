use crate::language::{self, Language};
use super::Message;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i64)]
pub enum MsgCode {
    SuccessDefault = 1,
    ErrBadRequest,
    ErrUnauthorized,
    ErrInvalidToken,
    ErrRefreshTokenExpired,
    ErrAccessTokenExpired,
    ErrForbidden,
    ErrNotFound,
    ErrContextTimeout,
    ErrConflict,
    ErrTooManyRequest,
    ErrRequestEntityTooLarge,
    ErrInternalServerError,
    ErrNotImplemented,
    ErrServiceUnavailable,
}

pub fn get_messages(msg_code: MsgCode) -> HashMap<Language, Message> {
    match msg_code {
        MsgCode::SuccessDefault => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 200,
                    title: language::http_status_text(language::ENGLISH, 200).to_string(),
                    body: "Request successful".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 200,
                    title: language::http_status_text(language::INDONESIAN, 200).to_string(),
                    body: "Request berhasil".to_string(),
                },
            );
            map
        }
        MsgCode::ErrBadRequest => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 400,
                    title: language::http_status_text(language::ENGLISH, 400).to_string(),
                    body: "Invalid input. Please validate your input.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 400,
                    title: language::http_status_text(language::INDONESIAN, 400).to_string(),
                    body: "Masukan data tidak valid. Mohon cek kembali masukan anda.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrUnauthorized => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::ENGLISH, 401).to_string(),
                    body: "Unauthorized access. You are not authorized to access this resource.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::INDONESIAN, 401).to_string(),
                    body: "Akses ditolak. Anda tidak memilik izin untuk mengakses sumber daya ini.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrInvalidToken => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::ENGLISH, 401).to_string(),
                    body: "Invalid token. Please renew your session by reloading.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::INDONESIAN, 401).to_string(),
                    body: "Token tidak valid. Mohon perbarui sesi anda dengan menggakses ulang laman.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrRefreshTokenExpired => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::ENGLISH, 401).to_string(),
                    body: "Session refresh token has expired. Please renew your session by reloading.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::INDONESIAN, 401).to_string(),
                    body: "Token pembaruan sudah tidak berlaku. Mohon perbarui sesi anda dengan mengakses ulang laman.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrAccessTokenExpired => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::ENGLISH, 401).to_string(),
                    body: "Session access token has expired. Please renew your session by reloading.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 401,
                    title: language::http_status_text(language::INDONESIAN, 401).to_string(),
                    body: "Token akses sudah tidak berlaku. Mohon perbarui sesi anda dengan mengakses ulang laman.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrForbidden => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 403,
                    title: language::http_status_text(language::ENGLISH, 403).to_string(),
                    body: "Forbidden. You don't have permission to access this resource.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 403,
                    title: language::http_status_text(language::INDONESIAN, 403).to_string(),
                    body: "Terlarang. Anda tidak memiliki izin untuk mengakses laman ini".to_string(),
                },
            );
            map
        }
        MsgCode::ErrNotFound => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 404,
                    title: language::http_status_text(language::ENGLISH, 404).to_string(),
                    body: "Record doesn't exist. Please validate your input or contact the administrator.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 404,
                    title: language::http_status_text(language::INDONESIAN, 404).to_string(),
                    body: "Data tidak ditemukan. Mohon cek kembali masukan anda atau hubungi administrator.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrContextTimeout => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 408,
                    title: language::http_status_text(language::ENGLISH, 408).to_string(),
                    body: "Request time has been exceeded.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 408,
                    title: language::http_status_text(language::INDONESIAN, 408).to_string(),
                    body: "Waktu permintaan habis.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrConflict => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 409,
                    title: language::http_status_text(language::ENGLISH, 409).to_string(),
                    body: "Record has existed. Please validate your input or contact the administrator.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 409,
                    title: language::http_status_text(language::INDONESIAN, 409).to_string(),
                    body: "Data sudah ada. Mohon cek kembali masukan anda atau hubungi administrator.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrTooManyRequest => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 429,
                    title: language::http_status_text(language::ENGLISH, 429).to_string(),
                    body: "Too many requests. Please wait and try again after a few seconds.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 429,
                    title: language::http_status_text(language::INDONESIAN, 429).to_string(),
                    body: "Terlalu banyak permintaan. Mohon tunggu dan coba lagi setelah beberapa saat.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrRequestEntityTooLarge => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 413,
                    title: language::http_status_text(language::ENGLISH, 413).to_string(),
                    body: "Your request entity too large. Please validate your input or contact the administrator.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 413,
                    title: language::http_status_text(language::INDONESIAN, 413).to_string(),
                    body: "Entitas permintaan Anda terlalu besar. Harap validasi input Anda atau hubungi administrator.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrInternalServerError => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 500,
                    title: language::http_status_text(language::ENGLISH, 500).to_string(),
                    body: "Internal server error. Please contact the administrator.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 500,
                    title: language::http_status_text(language::INDONESIAN, 500).to_string(),
                    body: "Terjadi kendala diserver. Mohon hubungi administrator.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrNotImplemented => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 501,
                    title: language::http_status_text(language::ENGLISH, 501).to_string(),
                    body: "Not Implemented. Please contact the administrator.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 501,
                    title: language::http_status_text(language::INDONESIAN, 501).to_string(),
                    body: "Layanan tidak tersedia. Mohon hubungi administrator.".to_string(),
                },
            );
            map
        }
        MsgCode::ErrServiceUnavailable => {
            let mut map = HashMap::new();
            map.insert(
                language::ENGLISH,
                Message {
                    status_code: 503,
                    title: language::http_status_text(language::ENGLISH, 503).to_string(),
                    body: "Service is unavailable. Please contact the administrator.".to_string(),
                },
            );
            map.insert(
                language::INDONESIAN,
                Message {
                    status_code: 503,
                    title: language::http_status_text(language::INDONESIAN, 503).to_string(),
                    body: "Layanan sedang tidak tersedia. Mohon hubungi administrator.".to_string(),
                },
            );
            map
        }
    }
}
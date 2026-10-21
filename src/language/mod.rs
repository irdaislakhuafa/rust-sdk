pub mod text_en;
pub mod text_id;

#[cfg(test)]
mod language_test;

pub type Language = &'static str;

pub const ENGLISH: Language = "EN";
pub const INDONESIAN: Language = "ID";
pub const JAPANESE: Language = "JP";
pub const DEUSTCH: Language = "DE";

pub fn from_str(s: &str) -> Option<Language> {
    match s {
        ENGLISH => Some(ENGLISH),
        INDONESIAN => Some(INDONESIAN),
        JAPANESE => Some(JAPANESE),
        DEUSTCH => Some(DEUSTCH),
        _ => None,
    }
}

pub fn http_status_text(lang: Language, code: u16) -> &'static str {
    match lang {
        INDONESIAN => text_id::status_text_id(code),
        _ => text_en::status_text_en(code),
    }
}

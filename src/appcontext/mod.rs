#[cfg(test)]
mod appcontext_test;

use crate::language;

/// Interface trait representing the context contract (mirroring Go's context.Context interface).
pub trait Interface {
    // getters
    fn get_accept_language(&self) -> language::Language;
    fn get_request_id(&self) -> &str;
    fn get_service_version(&self) -> &str;
    fn get_user_agent(&self) -> &str;
    fn get_request_start_time(&self) -> std::time::SystemTime;

    // setters
    fn set_accept_language(&mut self, lang: language::Language) -> &mut Self;
    fn set_request_id(&mut self, request_id: &str) -> &mut Self;
    fn set_service_version(&mut self, service_version: &str) -> &mut Self;
    fn set_user_agent(&mut self, user_agent: &str) -> &mut Self;
    fn set_request_start_time(&mut self, start_time: std::time::SystemTime) -> &mut Self;
}

/// Default implementation of the AppContext contract.
pub struct AppContext {
    accept_language: language::Language,
    request_id: String,
    service_version: String,
    user_agent: String,
    request_start_time: std::time::SystemTime,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            accept_language: language::ENGLISH,
            request_id: "".to_string(),
            service_version: "".to_string(),
            user_agent: "".to_string(),
            request_start_time: std::time::SystemTime::UNIX_EPOCH,
        }
    }
}

impl Interface for AppContext {
    fn get_accept_language(&self) -> language::Language {
        self.accept_language
    }

    fn get_request_id(&self) -> &str {
        &self.request_id
    }

    fn get_request_start_time(&self) -> std::time::SystemTime {
        self.request_start_time
    }

    fn get_service_version(&self) -> &str {
        &self.service_version
    }

    fn get_user_agent(&self) -> &str {
        &self.user_agent
    }

    fn set_accept_language(&mut self, lang: language::Language) -> &mut Self {
        self.accept_language = lang;
        self
    }

    fn set_request_id(&mut self, request_id: &str) -> &mut Self {
        self.request_id = request_id.to_string();
        self
    }

    fn set_request_start_time(&mut self, start_time: std::time::SystemTime) -> &mut Self {
        self.request_start_time = start_time;
        self
    }

    fn set_service_version(&mut self, service_version: &str) -> &mut Self {
        self.service_version = service_version.to_string();
        self
    }

    fn set_user_agent(&mut self, user_agent: &str) -> &mut Self {
        self.user_agent = user_agent.to_string();
        self
    }
}

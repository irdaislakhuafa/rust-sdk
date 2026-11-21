use crate::codes::Code;
use crate::language::Language;
use std::error::Error;
use std::fmt;

/// Convenience alias for a boxed, thread-safe error (common across SDK modules).
pub type SdkError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct Stacktrace {
    pub message: String,
    pub cause: Option<SdkError>,
    pub code: Code,
    pub file: String,
    pub function: String,
    pub line: usize,
}

impl Stacktrace {
    pub fn error(&self) -> &str {
        &self.message
    }

    pub fn exit_code(&self) -> i64 {
        if self.code == Code::NoCode {
            1
        } else {
            self.code.value() as i64
        }
    }
}

impl fmt::Display for Stacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for Stacktrace {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_ref()
            .map(|b| b.as_ref() as &(dyn Error + 'static))
    }
}

// `SdkError` (the `cause` field) does not implement
// `PartialEq`/`Eq`, so we cannot `#[derive(PartialEq, Eq)]` on `Stacktrace`.
// Compare the simple fields directly and the boxed error by pointer identity.
impl PartialEq for Stacktrace {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
            && self.code == other.code
            && self.file == other.file
            && self.line == other.line
            && match (&self.cause, &other.cause) {
                (Some(a), Some(b)) => std::ptr::eq(a.as_ref(), b.as_ref()),
                (None, None) => true,
                _ => false,
            }
    }
}
impl Eq for Stacktrace {}

#[derive(Debug)]
pub struct AppError {
    pub code: Code,
    pub title: String,
    pub body: String,
    pub sys: Option<SdkError>,
}

impl AppError {
    pub fn new(
        code: Code,
        title: impl Into<String>,
        body: impl Into<String>,
        sys: Option<SdkError>,
    ) -> Self {
        Self {
            code,
            title: title.into(),
            body: body.into(),
            sys,
        }
    }

    pub fn safe_convert<'a>(
        err: &'a (dyn Error + Send + Sync + 'static),
    ) -> Result<&'a AppError, String> {
        match err.downcast_ref::<AppError>() {
            Some(app_err) => Ok(app_err),
            None => Err(format!("Failed to convert error to AppError: {}", err)),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(sys) = &self.sys {
            write!(f, "{}", sys)
        } else {
            write!(f, "Error [{}]: {}", self.code.value(), self.title)
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.sys
            .as_ref()
            .map(|b| b.as_ref() as &(dyn Error + 'static))
    }
}

// Same as above: `SdkError` (the `sys` field) is not
// `PartialEq`/`Eq`, so `#[derive(PartialEq, Eq)]` is unavailable.
impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.title == other.title
            && self.body == other.body
            && match (&self.sys, &other.sys) {
                (Some(a), Some(b)) => std::ptr::eq(a.as_ref(), b.as_ref()),
                (None, None) => true,
                _ => false,
            }
    }
}
impl Eq for AppError {}

// ---- core functions ----

pub fn get_code(err: &(dyn Error + Send + Sync + 'static)) -> Code {
    if let Some(st) = err.downcast_ref::<Stacktrace>() {
        st.code
    } else {
        Code::NoCode
    }
}

fn create(
    cause: Option<SdkError>,
    code: Code,
    message: String,
    loc: &'static std::panic::Location<'static>,
) -> Stacktrace {
    let resolved_code = if code == Code::NoCode {
        cause
            .as_ref()
            .map(|e| get_code(e.as_ref()))
            .unwrap_or(Code::NoCode)
    } else {
        code
    };

    Stacktrace {
        message,
        cause,
        code: resolved_code,
        file: loc.file().to_string(),
        function: String::new(),
        line: loc.line() as usize,
    }
}

#[track_caller]
pub fn new_with_code(code: Code, message: impl Into<String>) -> SdkError {
    let loc = std::panic::Location::caller();
    Box::new(create(None, code, message.into(), loc))
}

#[track_caller]
pub fn get_caller(cause: Option<&SdkError>) -> Result<(String, usize, String), SdkError> {
    let loc = std::panic::Location::caller();
    match cause {
        None => Err(Box::new(create(
            None,
            Code::NoCode,
            "failed to cast error to stacktrace".to_string(),
            loc,
        ))),
        Some(c) => {
            if let Some(st) = c.downcast_ref::<Stacktrace>() {
                return Ok((st.file.clone(), st.line, st.message.clone()));
            }
            let msg = c.to_string();
            Err(Box::new(create(None, Code::NoCode, msg, loc)))
        }
    }
}

pub fn compile(err: SdkError, lang: Language) -> (u16, AppError) {
    let code = get_code(err.as_ref());
    let msgs = crate::codes::get_code_messages(code);

    if let Some(msg) = msgs.get(&lang) {
        return (
            msg.status_code,
            AppError {
                code,
                title: msg.title.clone(),
                body: msg.body.clone(),
                sys: Some(err),
            },
        );
    }

    (
        500,
        AppError {
            code,
            title: "Service Error Not Defined".to_string(),
            body: "Unknown error. Please contact admin".to_string(),
            sys: Some(err),
        },
    )
}

#[cfg(test)]
mod errors_stacktrace_test;
#[cfg(test)]
mod errors_test;

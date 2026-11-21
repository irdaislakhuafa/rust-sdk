// log level (port of go-sdk/log/levels.go)
//
// Rust-idiomatic: a proper enum with ordering instead of a string alias.
// Supports both string values (for config/JSON) and integer values (programmatic).

use crate::codes::Code;
use crate::errors::{SdkError, new_with_code};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
    Panic = 6,
}

impl Level {
    pub fn parse(s: &str) -> Result<Self, SdkError> {
        match s.trim().to_ascii_lowercase().as_str() {
            "trace" => Ok(Level::Trace),
            "debug" => Ok(Level::Debug),
            "info" => Ok(Level::Info),
            "warn" | "warning" => Ok(Level::Warn),
            "error" => Ok(Level::Error),
            "fatal" => Ok(Level::Fatal),
            "panic" => Ok(Level::Panic),
            other => Err(new_with_code(
                Code::LogInvalidLevel,
                format!("invalid log level: {}", other),
            )),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "trace",
            Level::Debug => "debug",
            Level::Info => "info",
            Level::Warn => "warn",
            Level::Error => "error",
            Level::Fatal => "fatal",
            Level::Panic => "panic",
        }
    }

    pub fn as_i8(&self) -> i8 {
        *self as i8
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i8(v: i8) -> Result<Self, SdkError> {
        match v {
            0 => Ok(Level::Trace),
            1 => Ok(Level::Debug),
            2 => Ok(Level::Info),
            3 => Ok(Level::Warn),
            4 => Ok(Level::Error),
            5 => Ok(Level::Fatal),
            6 => Ok(Level::Panic),
            _ => Err(new_with_code(
                Code::LogInvalidLevel,
                format!("invalid log level: {}", v),
            )),
        }
    }

    pub fn from_i32(v: i32) -> Result<Self, SdkError> {
        if v < i8::MIN as i32 || v > i8::MAX as i32 {
            return Err(new_with_code(
                Code::LogInvalidLevel,
                format!("invalid log level: {}", v),
            ));
        }
        Level::from_i8(v as i8)
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl serde::Serialize for Level {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Level {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let s = String::deserialize(de)?;
        Level::parse(&s).map_err(serde::de::Error::custom)
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_i8().cmp(&other.as_i8())
    }
}

impl TryFrom<i8> for Level {
    type Error = SdkError;
    fn try_from(v: i8) -> Result<Self, SdkError> {
        Level::from_i8(v)
    }
}

impl TryFrom<i32> for Level {
    type Error = SdkError;
    fn try_from(v: i32) -> Result<Self, SdkError> {
        Level::from_i32(v)
    }
}

impl From<Level> for i8 {
    fn from(l: Level) -> i8 {
        l.as_i8()
    }
}

impl From<Level> for i32 {
    fn from(l: Level) -> i32 {
        l.as_i32()
    }
}

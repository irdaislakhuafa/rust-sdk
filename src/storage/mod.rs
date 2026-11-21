// storage module (port of go-sdk/storage)

pub mod dummy;
pub mod helper;
pub mod minio;
pub mod types;

#[cfg(test)]
pub mod dummy_test;
#[cfg(test)]
pub mod helper_test;
#[cfg(test)]
pub mod minio_test;

pub use dummy::{DummyImpl, init_dummy};
pub use helper::*;
pub use minio::init_minio;
pub use types::*;

use crate::errors::SdkError;
pub fn init(cfg: Config) -> Result<Box<dyn Storage>, SdkError> {
    match cfg.storage_type {
        StorageType::Dummy => dummy::init_dummy(cfg),
        StorageType::Minio => minio::init_minio(cfg),
        _ => Err(crate::errors::new_with_code(
            crate::codes::Code::NotImplemented,
            format!(
                "storage type '{}' not implemented",
                cfg.storage_type.as_str()
            ),
        )),
    }
}

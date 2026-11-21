// Dummy storage implementation (port of go-sdk/storage/dummy.go)

use crate::errors::{new_with_code, SdkError};
use crate::storage::helper::normalize_dir;
use crate::storage::types::{Config, DelParams, PutParams, PutResult, Storage, UrlParams, UrlResult};

pub struct DummyImpl {
    pub cfg: Config,
}

pub fn init_dummy(cfg: Config) -> Result<Box<dyn Storage>, SdkError> {
    Ok(Box::new(DummyImpl { cfg }))
}

impl Storage for DummyImpl {
    fn put(&self, _ctx: (), mut params: PutParams) -> Result<PutResult, SdkError> {
        params.dir_name = normalize_dir(&params.dir_name);
        Ok(PutResult {
            dir_name: params.dir_name.clone(),
            file_name: params.file_name.clone(),
            full_path: format!("{}{}", params.dir_name, params.file_name),
            region: "dummy".to_string(),
            tag: "dummy".to_string(),
            size: params.file_size,
        })
    }

    fn del(&self, _ctx: (), _params: DelParams) -> Result<(), SdkError> {
        Ok(())
    }

    fn url(&self, _ctx: (), params: UrlParams) -> Result<UrlResult, SdkError> {
        Ok(UrlResult {
            scheme: "https".to_string(),
            host: "dummy.com".to_string(),
            full_path: params.full_path.clone(),
            full_url: format!("dummy.com/{}", params.full_path),
            raw_query: String::new(),
        })
    }
}




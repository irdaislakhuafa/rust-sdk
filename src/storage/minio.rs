// MinIO storage implementation (port of go-sdk/storage/minio.go)

use crate::codes::Code;
use crate::errors::{SdkError, new_with_code};
use crate::storage::helper::normalize_dir;
use crate::storage::types::{
    Config, DelParams, PutParams, PutResult, Storage, UrlParams, UrlResult,
};
use s3::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::io::Read;
use url::Url;

pub struct MinioImpl {
    bucket: Box<Bucket>,
    cfg: Config,
}

pub fn init_minio(cfg: Config) -> Result<Box<dyn Storage>, SdkError> {
    let mut cfg = cfg;
    cfg.parse_default();

    let scheme = if cfg.ssl { "https" } else { "http" };
    let host = cfg.host.trim();
    let port = cfg.port.trim();
    let endpoint = if port.is_empty() {
        format!("{}://{}", scheme, host)
    } else if port.starts_with(':') {
        format!("{}://{}{}", scheme, host, port)
    } else {
        format!("{}://{}:{}", scheme, host, port)
    };

    let credentials = Credentials::new(
        if cfg.access_key_id.is_empty() {
            None
        } else {
            Some(cfg.access_key_id.as_str())
        },
        if cfg.access_key_secret.is_empty() {
            None
        } else {
            Some(cfg.access_key_secret.as_str())
        },
        if cfg.token.is_empty() {
            None
        } else {
            Some(cfg.token.as_str())
        },
        None,
        None,
    )
    .map_err(|e| new_with_code(Code::StorageNoClient, e.to_string()))?;

    let region = Region::Custom {
        region: cfg.region.clone(),
        endpoint,
    };

    // Create bucket if it does not exist (best-effort; ignore "already exists").
    let bucket_for_check = Bucket::new(&cfg.base_dir, region.clone(), credentials.clone())
        .map_err(|e| new_with_code(Code::StorageNoClient, e.to_string()))?
        .with_path_style();
    let exists = bucket_for_check
        .exists()
        .map_err(|e| new_with_code(Code::StorageNoClient, e.to_string()))?;
    if !exists {
        let _ = Bucket::create(
            &cfg.base_dir,
            region.clone(),
            credentials.clone(),
            s3::BucketConfiguration::default(),
        );
    }

    let bucket = Bucket::new(&cfg.base_dir, region, credentials)
        .map_err(|e| new_with_code(Code::StorageNoClient, e.to_string()))?
        .with_path_style();

    Ok(Box::new(MinioImpl { bucket, cfg }))
}

impl Storage for MinioImpl {
    fn put(&self, _ctx: (), mut params: PutParams) -> Result<PutResult, SdkError> {
        params.dir_name = normalize_dir(&params.dir_name);
        let full_path = format!("{}{}", params.dir_name, params.file_name);

        let mut bytes = Vec::new();
        if let Some(mut reader) = params.file_buffer.take() {
            reader
                .read_to_end(&mut bytes)
                .map_err(|e| new_with_code(Code::StorageNoFile, e.to_string()))?;
        }

        let content_type = if params.content_type.is_empty() {
            "application/octet-stream"
        } else {
            &params.content_type
        };

        let response = self
            .bucket
            .put_object_with_content_type(&full_path, &bytes, content_type)
            .map_err(|e| new_with_code(Code::StorageNoFile, e.to_string()))?;

        let tag = response
            .headers()
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("etag"))
            .map(|(_, v)| v.clone())
            .unwrap_or_default();

        Ok(PutResult {
            dir_name: params.dir_name.clone(),
            file_name: params.file_name.clone(),
            full_path,
            region: self.cfg.region.clone(),
            tag,
            size: bytes.len() as i64,
        })
    }

    fn del(&self, _ctx: (), params: DelParams) -> Result<(), SdkError> {
        self.bucket
            .delete_object(&params.full_path)
            .map_err(|e| new_with_code(Code::StorageDelFailure, e.to_string()))?;
        Ok(())
    }

    fn url(&self, _ctx: (), params: UrlParams) -> Result<UrlResult, SdkError> {
        let mut params = params;
        params.parse_default();

        if params.full_path.is_empty() {
            return Ok(UrlResult {
                full_path: params.full_path.clone(),
                full_url: params.full_path.clone(),
                ..Default::default()
            });
        }

        if params.full_path.starts_with("http") {
            let parsed = Url::parse(&params.full_path)
                .map_err(|e| new_with_code(Code::BadRequest, e.to_string()))?;
            return Ok(UrlResult {
                scheme: parsed.scheme().to_string(),
                host: parsed.host_str().unwrap_or_default().to_string(),
                full_path: parsed.path().to_string(),
                full_url: params.full_path.clone(),
                raw_query: parsed.query().unwrap_or_default().to_string(),
            });
        }

        let expiry = if params.expire_duration.as_secs() == 0 {
            24 * 3600
        } else {
            params.expire_duration.as_secs() as u32
        };

        let presigned = self
            .bucket
            .presign_get(&params.full_path, expiry, None)
            .map_err(|e| new_with_code(Code::StorageGenerateUrlFailure, e.to_string()))?;

        let parsed = Url::parse(&presigned)
            .map_err(|e| new_with_code(Code::StorageGenerateUrlFailure, e.to_string()))?;

        Ok(UrlResult {
            scheme: parsed.scheme().to_string(),
            host: parsed.host_str().unwrap_or_default().to_string(),
            full_path: parsed.path().to_string(),
            full_url: presigned,
            raw_query: parsed.query().unwrap_or_default().to_string(),
        })
    }
}



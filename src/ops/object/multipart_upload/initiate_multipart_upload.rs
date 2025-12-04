use std::collections::HashMap;
use std::future::Future;

use heck::ToKebabCase;
use http::{HeaderMap, HeaderName, Method, header};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Storage class
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageClass {
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "IA")]
    InfrequentAccess,
    #[serde(rename = "Archive")]
    Archive,
    #[serde(rename = "ColdArchive")]
    ColdArchive,
    #[serde(rename = "DeepColdArchive")]
    DeepColdArchive,
}

impl Default for StorageClass {
    fn default() -> Self {
        Self::Standard
    }
}

impl AsRef<str> for StorageClass {
    fn as_ref(&self) -> &str {
        match self {
            StorageClass::Standard => "Standard",
            StorageClass::InfrequentAccess => "IA",
            StorageClass::Archive => "Archive",
            StorageClass::ColdArchive => "ColdArchive",
            StorageClass::DeepColdArchive => "DeepColdArchive",
        }
    }
}

/// Server-side encryption method
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServerSideEncryption {
    #[serde(rename = "AES256")]
    Aes256,
    #[serde(rename = "KMS")]
    Kms,
    #[serde(rename = "SM4")]
    Sm4,
}

impl AsRef<str> for ServerSideEncryption {
    fn as_ref(&self) -> &str {
        match self {
            ServerSideEncryption::Aes256 => "AES256",
            ServerSideEncryption::Kms => "KMS",
            ServerSideEncryption::Sm4 => "SM4",
        }
    }
}

/// Encoding type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EncodingType {
    #[serde(rename = "url")]
    Url,
}

impl AsRef<str> for EncodingType {
    fn as_ref(&self) -> &str {
        match self {
            EncodingType::Url => "url",
        }
    }
}

/// InitiateMultipartUpload request parameters
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct InitiateMultipartUploadParams {
    uploads: OnlyKeyField,
    pub encoding_type: Option<EncodingType>,
}

impl InitiateMultipartUploadParams {
    pub fn new() -> Self {
        Self {
            uploads: OnlyKeyField,
            encoding_type: None,
        }
    }

    pub fn encoding_type(mut self, encoding_type: EncodingType) -> Self {
        self.encoding_type = Some(encoding_type);
        self
    }
}

/// InitiateMultipartUpload request options
#[derive(Debug, Clone, Default)]
pub struct InitiateMultipartUploadOptions {
    /// Specify the web cache behavior of this object
    pub cache_control: Option<String>,
    /// Specify the name when this object is downloaded
    pub content_disposition: Option<String>,
    /// Specify the content encoding format when this object is downloaded
    pub content_encoding: Option<String>,
    /// Specify the content type of this object
    pub content_type: Option<String>,
    /// Expiration time
    pub expires: Option<String>,
    /// Whether to overwrite objects with the same name
    pub forbid_overwrite: Option<bool>,
    /// Server-side encryption method
    pub server_side_encryption: Option<ServerSideEncryption>,
    /// Encryption algorithm
    pub server_side_data_encryption: Option<String>,
    /// User master key managed by KMS
    pub server_side_encryption_key_id: Option<String>,
    /// Storage class
    pub storage_class: Option<StorageClass>,
    /// Object tags
    pub tagging: Option<String>,
    /// User-defined metadata
    pub user_meta: HashMap<String, String>,
}

impl InitiateMultipartUploadOptions {
    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        if let Some(cache_control) = self.cache_control {
            headers.insert(header::CACHE_CONTROL, cache_control.parse()?);
        }

        // Set content type
        if let Some(content_type) = self.content_type {
            headers.insert(header::CONTENT_TYPE, content_type.parse()?);
        }

        // Set content disposition
        if let Some(content_disposition) = self.content_disposition {
            headers.insert(header::CONTENT_DISPOSITION, content_disposition.parse()?);
        }

        // Set content encoding
        if let Some(content_encoding) = self.content_encoding {
            headers.insert(header::CONTENT_ENCODING, content_encoding.parse()?);
        }

        // Set Expiration time
        if let Some(expires) = self.expires {
            headers.insert(header::EXPIRES, expires.parse()?);
        }

        // Set whether to allow overwriting files with the same name
        if let Some(forbid_overwrite) = self.forbid_overwrite {
            headers.insert(
                HeaderName::from_static("x-oss-forbid-overwrite"),
                forbid_overwrite.to_string().parse()?,
            );
        }

        // Set Server-side encryption method
        if let Some(server_side_encryption) = self.server_side_encryption {
            headers.insert(
                HeaderName::from_static("x-oss-server-side-encryption"),
                server_side_encryption.as_ref().parse()?,
            );
        }

        // Set server-side data encryption algorithm
        if let Some(server_side_data_encryption) = self.server_side_data_encryption {
            headers.insert(
                HeaderName::from_static("x-oss-server-side-data-encryption"),
                server_side_data_encryption.parse()?,
            );
        }

        // Set KMS key ID
        if let Some(server_side_encryption_key_id) = self.server_side_encryption_key_id {
            headers.insert(
                HeaderName::from_static("x-oss-server-side-encryption-key-id"),
                server_side_encryption_key_id.parse()?,
            );
        }

        // Set Storage class
        if let Some(storage_class) = self.storage_class {
            headers.insert(HeaderName::from_static("x-oss-storage-class"), storage_class.as_ref().parse()?);
        }

        // Set Object tags
        if let Some(tagging) = self.tagging {
            headers.insert(HeaderName::from_static("x-oss-tagging"), tagging.parse()?);
        }

        // Set User-defined metadata
        for (key, value) in self.user_meta {
            let key = key.to_kebab_case().to_lowercase();
            let header_name = format!("x-oss-meta-{key}");
            headers.insert(HeaderName::from_bytes(header_name.as_bytes())?, value.parse()?);
        }

        Ok(headers)
    }
}

/// InitiateMultipartUpload response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitiateMultipartUploadResult {
    /// Bucket name
    pub bucket: String,
    /// Object name
    pub key: String,
    /// Unique ID identifying the multipart upload event
    pub upload_id: String,
    /// Encoding type
    pub encoding_type: Option<String>,
}

/// InitiateMultipartUpload operation
pub struct InitiateMultipartUpload {
    pub object_key: String,
    pub params: InitiateMultipartUploadParams,
    pub options: InitiateMultipartUploadOptions,
}

impl Ops for InitiateMultipartUpload {
    type Response = BodyResponseProcessor<InitiateMultipartUploadResult>;
    type Body = ZeroBody;
    type Query = InitiateMultipartUploadParams;

    fn prepare(self) -> Result<Prepared<InitiateMultipartUploadParams>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.object_key),
            query: Some(self.params),
            headers: Some(self.options.into_headers()?),
            body: Some(()),
            ..Default::default()
        })
    }
}

/// Trait for InitiateMultipartUpload operations
pub trait InitiateMultipartUploadOperations {
    /// Initialize multipart upload
    ///
    /// Official documentation: <https://www.alibabacloud.com/help/en/oss/developer-reference/initiatemultipartupload>
    fn initiate_multipart_upload(
        &self,
        object_key: impl Into<String>,
        options: Option<InitiateMultipartUploadOptions>,
    ) -> impl Future<Output = Result<InitiateMultipartUploadResult>>;
}

impl InitiateMultipartUploadOperations for Client {
    async fn initiate_multipart_upload(
        &self,
        object_key: impl Into<String>,
        options: Option<InitiateMultipartUploadOptions>,
    ) -> Result<InitiateMultipartUploadResult> {
        let ops = InitiateMultipartUpload {
            object_key: object_key.into(),
            params: InitiateMultipartUploadParams::new(),
            options: options.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

// =============================================================================
// Convenience builder and helper functions
// =============================================================================

/// InitiateMultipartUpload request builder
#[derive(Debug, Clone, Default)]
pub struct InitiateMultipartUploadRequestBuilder {
    options: InitiateMultipartUploadOptions,
}

impl InitiateMultipartUploadRequestBuilder {
    /// Create a new request builder
    pub fn new() -> Self {
        Self {
            options: InitiateMultipartUploadOptions::default(),
        }
    }

    /// Set cache control
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.options.cache_control = Some(cache_control.into());
        self
    }

    /// Set the name when downloading
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.options.content_disposition = Some(content_disposition.into());
        self
    }

    /// Set content encoding format
    pub fn content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.options.content_encoding = Some(content_encoding.into());
        self
    }

    /// Set content type
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.options.content_type = Some(content_type.into());
        self
    }

    /// Set expiration time
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.options.expires = Some(expires.into());
        self
    }

    /// Set whether to forbid overwriting objects with the same name
    pub fn forbid_overwrite(mut self, forbid: bool) -> Self {
        self.options.forbid_overwrite = Some(forbid);
        self
    }

    /// Set server-side encryption method
    pub fn server_side_encryption(mut self, encryption: ServerSideEncryption) -> Self {
        self.options.server_side_encryption = Some(encryption);
        self
    }

    /// Set encryption algorithm
    pub fn server_side_data_encryption(mut self, encryption: impl Into<String>) -> Self {
        self.options.server_side_data_encryption = Some(encryption.into());
        self
    }

    /// Set KMS key ID
    pub fn server_side_encryption_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.options.server_side_encryption_key_id = Some(key_id.into());
        self
    }

    /// Set storage class
    pub fn storage_class(mut self, storage_class: StorageClass) -> Self {
        self.options.storage_class = Some(storage_class);
        self
    }

    /// Set object tags
    pub fn tagging(mut self, tagging: impl Into<String>) -> Self {
        self.options.tagging = Some(tagging.into());
        self
    }

    /// Add user-defined metadata
    pub fn user_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.user_meta.insert(key.into(), value.into());
        self
    }

    /// Build request options
    pub fn build(self) -> InitiateMultipartUploadOptions {
        self.options
    }
}

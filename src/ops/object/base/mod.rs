mod copy_object;
mod delete_object;
mod get_object;
mod head_object;
mod put_object;

use serde::{Deserialize, Serialize};

/// OSS storage class
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all_fields = "lowercase")]
pub enum StorageClass {
    /// Standard storage
    #[default]
    Standard,
    /// Infrequent Access storage
    InfrequentAccess,
    /// Archive storage
    Archive,
    /// Cold Archive storage
    ColdArchive,
    /// Deep Cold Archive storage
    DeepColdArchive,
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

/// Server-side encryption type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServerSideEncryption {
    /// AES256 encryption
    #[serde(rename = "AES256")]
    Aes256,
    /// KMS encryption
    #[serde(rename = "KMS")]
    Kms,
    /// SM4 encryption
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

pub use copy_object::*;
pub use delete_object::*;
pub use get_object::*;
pub use head_object::*;
pub use put_object::*;

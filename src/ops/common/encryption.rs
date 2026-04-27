use serde::{Deserialize, Serialize};

/// Server-side encryption algorithm. Used via the `x-oss-server-side-encryption`
/// header on `PutObject`, `CopyObject`, `InitiateMultipartUpload`, and via XML
/// on the bucket encryption and KMS related operations.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ServerSideEncryption {
    /// AES-256 encryption managed by OSS.
    #[default]
    #[serde(rename = "AES256")]
    Aes256,
    /// Keys managed by Alibaba Cloud KMS.
    #[serde(rename = "KMS")]
    Kms,
    /// SM4 (China national cipher) encryption managed by OSS.
    #[serde(rename = "SM4")]
    Sm4,
}

impl ServerSideEncryption {
    /// The wire form used by OSS (what appears in XML and HTTP headers).
    pub fn as_str(&self) -> &'static str {
        match self {
            ServerSideEncryption::Aes256 => "AES256",
            ServerSideEncryption::Kms => "KMS",
            ServerSideEncryption::Sm4 => "SM4",
        }
    }
}

impl AsRef<str> for ServerSideEncryption {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_side_encryption_wire_names() {
        assert_eq!(serde_json::to_string(&ServerSideEncryption::Aes256).unwrap(), "\"AES256\"");
        assert_eq!(serde_json::to_string(&ServerSideEncryption::Kms).unwrap(), "\"KMS\"");
        assert_eq!(serde_json::to_string(&ServerSideEncryption::Sm4).unwrap(), "\"SM4\"");
    }

    #[test]
    fn server_side_encryption_round_trip() {
        for sse in [
            ServerSideEncryption::Aes256,
            ServerSideEncryption::Kms,
            ServerSideEncryption::Sm4,
        ] {
            let json = serde_json::to_string(&sse).unwrap();
            let back: ServerSideEncryption = serde_json::from_str(&json).unwrap();
            assert_eq!(sse, back);
        }
    }
}

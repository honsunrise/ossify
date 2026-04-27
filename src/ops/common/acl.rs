use serde::{Deserialize, Serialize};

/// Bucket canned ACL. Used by `PutBucketAcl`, `GetBucketAcl`, and the
/// `x-oss-acl` header on `PutBucket`.
///
/// Unlike object ACLs there is no `default` value for a bucket.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BucketAcl {
    /// Anonymous users can read and write objects in the bucket.
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
    /// Anonymous users can only read. Only the owner / authorized users can
    /// write.
    #[serde(rename = "public-read")]
    PublicRead,
    /// Only the owner / authorized users can read and write.
    #[default]
    #[serde(rename = "private")]
    Private,
}

impl BucketAcl {
    /// Wire form used by OSS (value of the `x-oss-acl` header and the
    /// `<Grant>` XML element).
    pub fn as_str(&self) -> &'static str {
        match self {
            BucketAcl::PublicReadWrite => "public-read-write",
            BucketAcl::PublicRead => "public-read",
            BucketAcl::Private => "private",
        }
    }
}

impl AsRef<str> for BucketAcl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_acl_wire_names() {
        assert_eq!(serde_json::to_string(&BucketAcl::PublicReadWrite).unwrap(), "\"public-read-write\"");
        assert_eq!(serde_json::to_string(&BucketAcl::PublicRead).unwrap(), "\"public-read\"");
        assert_eq!(serde_json::to_string(&BucketAcl::Private).unwrap(), "\"private\"");
    }

    #[test]
    fn bucket_acl_round_trip() {
        for acl in [BucketAcl::PublicReadWrite, BucketAcl::PublicRead, BucketAcl::Private] {
            let json = serde_json::to_string(&acl).unwrap();
            let back: BucketAcl = serde_json::from_str(&json).unwrap();
            assert_eq!(acl, back);
        }
    }
}

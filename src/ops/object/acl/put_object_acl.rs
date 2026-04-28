//! PutObjectACL: set the access permissions on an existing object.
//!
//! The ACL is passed via the `x-oss-object-acl` header, not in the body.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putobjectacl>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::ops::common::ObjectAcl;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// PutObjectACL query parameters: `?acl[&versionId=<id>]`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutObjectAclParams {
    pub(crate) acl: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PutObjectAclParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, v: impl Into<String>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

/// PutObjectACL operation.
pub struct PutObjectAcl {
    pub object_key: String,
    pub params: PutObjectAclParams,
    pub acl: ObjectAcl,
}

impl Ops for PutObjectAcl {
    type Response = EmptyResponseProcessor;
    type Body = ZeroBody;
    type Query = PutObjectAclParams;

    fn prepare(self) -> Result<Prepared<PutObjectAclParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-object-acl"), self.acl.as_str().parse()?);
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.object_key),
            query: Some(self.params),
            headers: Some(headers),
            body: Some(()),
            ..Default::default()
        })
    }
}

/// Trait for PutObjectACL operations.
pub trait PutObjectAclOperations {
    /// Set the ACL of an object.
    fn put_object_acl(
        &self,
        object_key: impl Into<String>,
        acl: ObjectAcl,
        params: Option<PutObjectAclParams>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutObjectAclOperations for Client {
    async fn put_object_acl(
        &self,
        object_key: impl Into<String>,
        acl: ObjectAcl,
        params: Option<PutObjectAclParams>,
    ) -> Result<()> {
        let ops = PutObjectAcl {
            object_key: object_key.into(),
            params: params.unwrap_or_default(),
            acl,
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params_default() {
        let q = crate::ser::to_string(&PutObjectAclParams::default()).unwrap();
        assert_eq!(q, "acl");
    }

    #[test]
    fn test_serialize_params_with_version() {
        let q = crate::ser::to_string(&PutObjectAclParams::new().version_id("v1")).unwrap();
        assert_eq!(q, "acl&versionId=v1");
    }

    #[test]
    fn test_prepare_header() {
        let p = PutObjectAcl {
            object_key: "foo.jpg".into(),
            params: PutObjectAclParams::new(),
            acl: ObjectAcl::PublicRead,
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::PUT);
        assert_eq!(p.key.as_deref(), Some("foo.jpg"));
        assert_eq!(p.headers.unwrap().get("x-oss-object-acl").unwrap(), "public-read");
    }
}

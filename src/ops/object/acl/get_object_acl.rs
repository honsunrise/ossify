//! GetObjectACL: query an object's ACL.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getobjectacl>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{ObjectAcl, Owner};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// GetObjectACL query parameters: `?acl[&versionId=<id>]`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectAclParams {
    pub(crate) acl: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetObjectAclParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, v: impl Into<String>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

/// `<AccessControlList>` element — contains only the `Grant` value.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct AccessControlList {
    /// Wire values are whitespace-padded occasionally; parse as String and
    /// let callers compare after trim.
    pub grant: String,
}

impl AccessControlList {
    /// Return the grant parsed as an [`ObjectAcl`], trimming whitespace.
    pub fn as_acl(&self) -> Option<ObjectAcl> {
        match self.grant.trim() {
            "default" => Some(ObjectAcl::Default),
            "private" => Some(ObjectAcl::Private),
            "public-read" => Some(ObjectAcl::PublicRead),
            "public-read-write" => Some(ObjectAcl::PublicReadWrite),
            _ => None,
        }
    }
}

/// GetObjectACL response body `<AccessControlPolicy>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "AccessControlPolicy", rename_all = "PascalCase")]
pub struct GetObjectAclResponse {
    pub owner: Owner,
    pub access_control_list: AccessControlList,
}

/// GetObjectACL operation.
pub struct GetObjectAcl {
    pub object_key: String,
    pub params: GetObjectAclParams,
}

impl Ops for GetObjectAcl {
    type Response = BodyResponseProcessor<GetObjectAclResponse>;
    type Body = NoneBody;
    type Query = GetObjectAclParams;

    fn prepare(self) -> Result<Prepared<GetObjectAclParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for GetObjectACL operations.
pub trait GetObjectAclOperations {
    fn get_object_acl(
        &self,
        object_key: impl Into<String>,
        params: Option<GetObjectAclParams>,
    ) -> impl Future<Output = Result<GetObjectAclResponse>>;
}

impl GetObjectAclOperations for Client {
    async fn get_object_acl(
        &self,
        object_key: impl Into<String>,
        params: Option<GetObjectAclParams>,
    ) -> Result<GetObjectAclResponse> {
        let ops = GetObjectAcl {
            object_key: object_key.into(),
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&GetObjectAclParams::new().version_id("v1")).unwrap();
        assert_eq!(q, "acl&versionId=v1");
    }

    #[test]
    fn test_deserialize_response() {
        let xml = r#"<?xml version="1.0" ?>
<AccessControlPolicy>
  <Owner>
    <ID>0022012****</ID>
    <DisplayName>0022012****</DisplayName>
  </Owner>
  <AccessControlList>
    <Grant>public-read </Grant>
  </AccessControlList>
</AccessControlPolicy>"#;
        let resp: GetObjectAclResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(resp.owner.id.as_deref(), Some("0022012****"));
        assert_eq!(resp.access_control_list.as_acl(), Some(ObjectAcl::PublicRead));
    }
}

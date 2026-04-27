//! GetBucketAcl: query the access control list of a bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketacl>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{BucketAcl, Owner};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketAclParams {
    acl: OnlyKeyField,
}

/// XML `<AccessControlList>` element.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct AccessControlList {
    /// The canned ACL value currently applied to the bucket.
    pub grant: BucketAcl,
}

/// Response body for [`GetBucketAcl`] (XML root `<AccessControlPolicy>`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccessControlPolicy {
    /// Bucket owner information.
    pub owner: Owner,
    /// The ACL applied to the bucket.
    pub access_control_list: AccessControlList,
}

/// The `GetBucketAcl` operation.
pub struct GetBucketAcl;

impl Ops for GetBucketAcl {
    type Response = BodyResponseProcessor<AccessControlPolicy>;
    type Body = NoneBody;
    type Query = GetBucketAclParams;

    fn prepare(self) -> Result<Prepared<GetBucketAclParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketAclParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketAclOps {
    /// Query the ACL of the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketacl>
    fn get_bucket_acl(&self) -> impl Future<Output = Result<AccessControlPolicy>>;
}

impl GetBucketAclOps for Client {
    async fn get_bucket_acl(&self) -> Result<AccessControlPolicy> {
        self.request(GetBucketAcl).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_adds_acl_subresource() {
        let q = crate::ser::to_string(&GetBucketAclParams::default()).unwrap();
        assert_eq!(q, "acl");
    }

    #[test]
    fn prepared_uses_get() {
        let prepared = GetBucketAcl.prepare().unwrap();
        assert_eq!(prepared.method, Method::GET);
        assert!(prepared.body.is_none());
    }

    #[test]
    fn parse_sample_response() {
        let xml = r#"<?xml version="1.0"?>
<AccessControlPolicy>
    <Owner>
        <ID>0022012****</ID>
        <DisplayName>user_example</DisplayName>
    </Owner>
    <AccessControlList>
        <Grant>public-read</Grant>
    </AccessControlList>
</AccessControlPolicy>"#;
        let parsed: AccessControlPolicy = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.owner.id.as_deref(), Some("0022012****"));
        assert_eq!(parsed.owner.display_name.as_deref(), Some("user_example"));
        assert_eq!(parsed.access_control_list.grant, BucketAcl::PublicRead);
    }

    #[test]
    fn parse_private_acl() {
        let xml = r#"<AccessControlPolicy>
    <Owner>
        <ID>x</ID>
        <DisplayName>x</DisplayName>
    </Owner>
    <AccessControlList>
        <Grant>private</Grant>
    </AccessControlList>
</AccessControlPolicy>"#;
        let parsed: AccessControlPolicy = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.access_control_list.grant, BucketAcl::Private);
    }

    #[test]
    fn parse_public_read_write_acl() {
        let xml = r#"<AccessControlPolicy>
    <Owner>
        <ID>x</ID>
        <DisplayName>x</DisplayName>
    </Owner>
    <AccessControlList>
        <Grant>public-read-write</Grant>
    </AccessControlList>
</AccessControlPolicy>"#;
        let parsed: AccessControlPolicy = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.access_control_list.grant, BucketAcl::PublicReadWrite);
    }
}

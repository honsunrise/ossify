//! GetBucketPolicyStatus.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketpolicystatus>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketPolicyStatusParams {
    #[serde(rename = "policyStatus")]
    policy_status: OnlyKeyField,
}

/// Response body for [`GetBucketPolicyStatus`] (XML root `<PolicyStatus>`).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatus {
    /// Whether the current bucket policy allows public access.
    pub is_public: bool,
}

pub struct GetBucketPolicyStatus;

impl Ops for GetBucketPolicyStatus {
    type Response = BodyResponseProcessor<PolicyStatus>;
    type Body = NoneBody;
    type Query = GetBucketPolicyStatusParams;

    fn prepare(self) -> Result<Prepared<GetBucketPolicyStatusParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketPolicyStatusParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketPolicyStatusOps {
    /// Query whether the bucket's policy allows public access.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketpolicystatus>
    fn get_bucket_policy_status(&self) -> impl Future<Output = Result<PolicyStatus>>;
}

impl GetBucketPolicyStatusOps for Client {
    async fn get_bucket_policy_status(&self) -> Result<PolicyStatus> {
        self.request(GetBucketPolicyStatus).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketPolicyStatusParams::default()).unwrap(),
            "policyStatus"
        );
    }

    #[test]
    fn parse_public() {
        let xml = r#"<PolicyStatus><IsPublic>true</IsPublic></PolicyStatus>"#;
        let parsed: PolicyStatus = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.is_public);
    }

    #[test]
    fn parse_private() {
        let xml = r#"<PolicyStatus><IsPublic>false</IsPublic></PolicyStatus>"#;
        let parsed: PolicyStatus = quick_xml::de::from_str(xml).unwrap();
        assert!(!parsed.is_public);
    }
}

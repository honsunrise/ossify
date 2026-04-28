//! GetVectorBucket: fetch vector bucket metadata.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getvectorbucket>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetVectorBucketParams {
    #[serde(rename = "bucketInfo")]
    bucket_info: OnlyKeyField,
}

/// Vector bucket metadata (uses PascalCase keys for compatibility with the
/// legacy OSS XML schema, even though the transport format here is JSON).
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VectorBucketInfo {
    /// The bucket's ARN, formatted as
    /// `acs:ossvector:<region>:<uid>:<bucket-name>`.
    pub name: String,
    pub creation_date: Option<String>,
    pub extranet_endpoint: Option<String>,
    pub intranet_endpoint: Option<String>,
    pub location: Option<String>,
    pub region: Option<String>,
}

/// Root object of the `GetVectorBucket` response.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetVectorBucketResult {
    pub bucket_info: VectorBucketInfo,
}

pub struct GetVectorBucket;

impl Ops for GetVectorBucket {
    type Response = BodyResponseProcessor<GetVectorBucketResult>;
    type Body = NoneBody;
    type Query = GetVectorBucketParams;

    fn prepare(self) -> Result<Prepared<GetVectorBucketParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetVectorBucketParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetVectorBucketOps {
    /// Fetch metadata of the current vector bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getvectorbucket>
    fn get_vector_bucket(&self) -> impl Future<Output = Result<GetVectorBucketResult>>;
}

impl GetVectorBucketOps for Client {
    async fn get_vector_bucket(&self) -> Result<GetVectorBucketResult> {
        self.request(GetVectorBucket).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetVectorBucketParams::default()).unwrap(), "bucketInfo");
    }

    #[test]
    fn parse_response() {
        let json = r#"{
          "BucketInfo": {
            "CreationDate": "2013-07-31T10:56:21.000Z",
            "ExtranetEndpoint": "cn-hangzhou.oss-vectors.aliyuncs.com",
            "IntranetEndpoint": "cn-hangzhou-internal.oss-vectors.aliyuncs.com",
            "Location": "oss-cn-hangzhou",
            "Name": "acs:ossvector:cn-shanghai:103735**********:examplebucket",
            "Region": "cn-hangzhou"
          }
        }"#;
        let parsed: GetVectorBucketResult = serde_json::from_str(json).unwrap();
        assert!(parsed.bucket_info.name.starts_with("acs:ossvector:"));
        assert_eq!(parsed.bucket_info.region.as_deref(), Some("cn-hangzhou"));
    }
}

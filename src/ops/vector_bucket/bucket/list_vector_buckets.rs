//! ListVectorBuckets: list vector buckets in the current account/region.
//!
//! This is an account-level operation (`USE_BUCKET=false`); the request
//! targets `{region}.oss-vectors.aliyuncs.com`.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listvectorbuckets>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::get_vector_bucket::VectorBucketInfo;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`ListVectorBuckets`].
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListVectorBucketsParams {
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub max_keys: Option<u32>,
}

impl ListVectorBucketsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.marker = Some(marker.into());
        self
    }

    pub fn max_keys(mut self, max_keys: u32) -> Self {
        self.max_keys = Some(max_keys);
        self
    }
}

/// Body of a `<ListAllMyBucketsResult>` payload returned as JSON.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyVectorBucketsResult {
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub max_keys: Option<u32>,
    #[serde(default)]
    pub is_truncated: bool,
    pub next_marker: Option<String>,
    #[serde(default)]
    pub buckets: Vec<VectorBucketInfo>,
}

/// Root object of the `ListVectorBuckets` response.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListVectorBucketsResult {
    pub list_all_my_buckets_result: ListAllMyVectorBucketsResult,
}

pub struct ListVectorBuckets {
    pub params: ListVectorBucketsParams,
}

impl Ops for ListVectorBuckets {
    type Response = BodyResponseProcessor<ListVectorBucketsResult>;
    type Body = NoneBody;
    type Query = ListVectorBucketsParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListVectorBucketsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListVectorBucketsOps {
    /// List vector buckets belonging to the current account/region.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listvectorbuckets>
    fn list_vector_buckets(
        &self,
        params: Option<ListVectorBucketsParams>,
    ) -> impl Future<Output = Result<ListVectorBucketsResult>>;
}

impl ListVectorBucketsOps for Client {
    async fn list_vector_buckets(
        &self,
        params: Option<ListVectorBucketsParams>,
    ) -> Result<ListVectorBucketsResult> {
        self.request(ListVectorBuckets {
            params: params.unwrap_or_default(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_defaults() {
        assert_eq!(crate::ser::to_string(&ListVectorBucketsParams::new()).unwrap(), "");
    }

    #[test]
    fn params_serialize_full() {
        let q = crate::ser::to_string(
            &ListVectorBucketsParams::new()
                .prefix("my")
                .marker("mybucket")
                .max_keys(10),
        )
        .unwrap();
        assert_eq!(q, "marker=mybucket&max-keys=10&prefix=my");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<ListVectorBuckets as Ops>::USE_BUCKET);
    }

    #[test]
    fn parse_response() {
        let json = r#"{
          "ListAllMyBucketsResult": {
            "Prefix": "my",
            "Marker": "mybucket",
            "MaxKeys": 10,
            "IsTruncated": true,
            "NextMarker": "mybucket10",
            "Buckets": [
              {
                "CreationDate": "2014-02-07T18:12:43.000Z",
                "ExtranetEndpoint": "cn-shanghai.oss-vectors.aliyuncs.com",
                "IntranetEndpoint": "cn-shanghai-internal.oss-vectors.aliyuncs.com",
                "Location": "oss-cn-shanghai",
                "Name": "acs:ossvector:cn-shanghai:103735**********:test-bucket-3",
                "Region": "cn-shanghai"
              }
            ]
          }
        }"#;
        let parsed: ListVectorBucketsResult = serde_json::from_str(json).unwrap();
        let r = parsed.list_all_my_buckets_result;
        assert!(r.is_truncated);
        assert_eq!(r.next_marker.as_deref(), Some("mybucket10"));
        assert_eq!(r.buckets.len(), 1);
        assert_eq!(r.buckets[0].region.as_deref(), Some("cn-shanghai"));
    }

    #[test]
    fn parse_response_empty() {
        let json = r#"{ "ListAllMyBucketsResult": { "Buckets": [] } }"#;
        let parsed: ListVectorBucketsResult = serde_json::from_str(json).unwrap();
        assert!(!parsed.list_all_my_buckets_result.is_truncated);
        assert!(parsed.list_all_my_buckets_result.buckets.is_empty());
    }
}

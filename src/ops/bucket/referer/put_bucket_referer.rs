//! PutBucketReferer.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketreferer>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketRefererParams {
    referer: OnlyKeyField,
}

/// `<RefererList>` / `<RefererBlacklist>` container. OSS wraps the repeated
/// `<Referer>` elements in one of these two containers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefererList {
    #[serde(rename = "Referer", default, skip_serializing_if = "Vec::is_empty")]
    pub referers: Vec<String>,
}

/// Root `<RefererConfiguration>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "RefererConfiguration", rename_all = "PascalCase")]
pub struct RefererConfiguration {
    pub allow_empty_referer: bool,
    pub allow_truncate_query_string: Option<bool>,
    pub truncate_path: Option<bool>,
    pub referer_list: Option<RefererList>,
    pub referer_blacklist: Option<RefererList>,
}

pub struct PutBucketReferer {
    pub config: RefererConfiguration,
}

impl Ops for PutBucketReferer {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<RefererConfiguration>;
    type Query = PutBucketRefererParams;

    fn prepare(self) -> Result<Prepared<PutBucketRefererParams, RefererConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketRefererParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketRefererOps {
    /// Configure the Referer whitelist / blacklist on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketreferer>
    fn put_bucket_referer(&self, config: RefererConfiguration) -> impl Future<Output = Result<()>>;
}

impl PutBucketRefererOps for Client {
    async fn put_bucket_referer(&self, config: RefererConfiguration) -> Result<()> {
        self.request(PutBucketReferer { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketRefererParams::default()).unwrap(), "referer");
    }

    #[test]
    fn round_trip_whitelist_and_blacklist() {
        let cfg = RefererConfiguration {
            allow_empty_referer: false,
            allow_truncate_query_string: Some(true),
            truncate_path: Some(true),
            referer_list: Some(RefererList {
                referers: vec!["http://www.aliyun.com".to_string()],
            }),
            referer_blacklist: Some(RefererList {
                referers: vec!["http://www.refuse.com".to_string()],
            }),
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        let back: RefererConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}

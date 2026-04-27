//! PutBucketLifecycle.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketlifecycle>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::LifecycleConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketLifecycleParams {
    lifecycle: OnlyKeyField,
}

/// Options for [`PutBucketLifecycle`].
#[derive(Debug, Clone, Default)]
pub struct PutBucketLifecycleOptions {
    /// When `true`, OSS allows rules whose prefixes overlap. Default `false`.
    ///
    /// Sent as the `x-oss-allow-same-action-overlap` request header.
    pub allow_same_action_overlap: Option<bool>,
}

impl PutBucketLifecycleOptions {
    fn into_headers(self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        if let Some(flag) = self.allow_same_action_overlap {
            headers.insert(
                HeaderName::from_static("x-oss-allow-same-action-overlap"),
                flag.to_string().parse()?,
            );
        }
        Ok(headers)
    }
}

/// The `PutBucketLifecycle` operation.
pub struct PutBucketLifecycle {
    pub config: LifecycleConfiguration,
    pub options: PutBucketLifecycleOptions,
}

impl Ops for PutBucketLifecycle {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<LifecycleConfiguration>;
    type Query = PutBucketLifecycleParams;

    fn prepare(self) -> Result<Prepared<PutBucketLifecycleParams, LifecycleConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketLifecycleParams::default()),
            headers: Some(self.options.into_headers()?),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketLifecycleOps {
    /// Configure (overwrite) the lifecycle rules of a bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketlifecycle>
    fn put_bucket_lifecycle(
        &self,
        config: LifecycleConfiguration,
        options: Option<PutBucketLifecycleOptions>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutBucketLifecycleOps for Client {
    async fn put_bucket_lifecycle(
        &self,
        config: LifecycleConfiguration,
        options: Option<PutBucketLifecycleOptions>,
    ) -> Result<()> {
        let ops = PutBucketLifecycle {
            config,
            options: options.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::{LifecycleExpiration, LifecycleRule, LifecycleRuleStatus};

    #[test]
    fn params_serialize_adds_lifecycle_subresource() {
        let q = crate::ser::to_string(&PutBucketLifecycleParams::default()).unwrap();
        assert_eq!(q, "lifecycle");
    }

    #[test]
    fn prepared_has_put_method_and_xml_body() {
        let config = LifecycleConfiguration::with_rules(vec![LifecycleRule {
            id: Some("r".to_string()),
            prefix: Some("log/".to_string()),
            status: LifecycleRuleStatus::Enabled,
            expiration: Some(LifecycleExpiration {
                days: Some(30),
                ..Default::default()
            }),
            ..Default::default()
        }]);
        let prepared = PutBucketLifecycle {
            config,
            options: PutBucketLifecycleOptions::default(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
        assert!(prepared.body.is_some());
    }

    #[test]
    fn options_into_headers_sets_allow_overlap() {
        let headers = PutBucketLifecycleOptions {
            allow_same_action_overlap: Some(true),
        }
        .into_headers()
        .unwrap();
        assert_eq!(headers.get("x-oss-allow-same-action-overlap").unwrap(), "true");
    }

    #[test]
    fn options_into_headers_empty_by_default() {
        let headers = PutBucketLifecycleOptions::default().into_headers().unwrap();
        assert!(headers.is_empty());
    }
}

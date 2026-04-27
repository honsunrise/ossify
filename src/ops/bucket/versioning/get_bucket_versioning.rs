//! GetBucketVersioning.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketversioning>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_versioning::VersioningConfiguration;
#[allow(unused_imports)]
pub use super::put_bucket_versioning::VersioningStatus;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketVersioningParams {
    versioning: OnlyKeyField,
}

pub struct GetBucketVersioning;

impl Ops for GetBucketVersioning {
    type Response = BodyResponseProcessor<VersioningConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketVersioningParams;

    fn prepare(self) -> Result<Prepared<GetBucketVersioningParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketVersioningParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketVersioningOps {
    /// Query the versioning state of the bucket.
    ///
    /// Returns a `VersioningConfiguration` with `status: None` if versioning
    /// has never been enabled.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketversioning>
    fn get_bucket_versioning(&self) -> impl Future<Output = Result<VersioningConfiguration>>;
}

impl GetBucketVersioningOps for Client {
    async fn get_bucket_versioning(&self) -> Result<VersioningConfiguration> {
        self.request(GetBucketVersioning).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketVersioningParams::default()).unwrap(),
            "versioning"
        );
    }

    #[test]
    fn parse_enabled() {
        let xml = r#"<VersioningConfiguration><Status>Enabled</Status></VersioningConfiguration>"#;
        let parsed: VersioningConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.status, Some(VersioningStatus::Enabled));
    }

    #[test]
    fn parse_empty() {
        // Bucket has never had versioning enabled: empty body.
        let xml = r#"<VersioningConfiguration/>"#;
        let parsed: VersioningConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.status, None);
    }
}

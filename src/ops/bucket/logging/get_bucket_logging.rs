//! GetBucketLogging.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketlogging>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_logging::BucketLoggingStatus;
#[allow(unused_imports)]
pub use super::put_bucket_logging::LoggingEnabled;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketLoggingParams {
    logging: OnlyKeyField,
}

pub struct GetBucketLogging;

impl Ops for GetBucketLogging {
    type Response = BodyResponseProcessor<BucketLoggingStatus>;
    type Body = NoneBody;
    type Query = GetBucketLoggingParams;

    fn prepare(self) -> Result<Prepared<GetBucketLoggingParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketLoggingParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketLoggingOps {
    /// Query the bucket access-logging configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketlogging>
    fn get_bucket_logging(&self) -> impl Future<Output = Result<BucketLoggingStatus>>;
}

impl GetBucketLoggingOps for Client {
    async fn get_bucket_logging(&self) -> Result<BucketLoggingStatus> {
        self.request(GetBucketLogging).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetBucketLoggingParams::default()).unwrap(), "logging");
    }

    #[test]
    fn parse_logging_enabled() {
        let xml = r#"<BucketLoggingStatus>
    <LoggingEnabled>
        <TargetBucket>mybucketlogs</TargetBucket>
        <TargetPrefix>mybucket-access_log/</TargetPrefix>
        <LoggingRole>AliyunOSSLoggingDefaultRole</LoggingRole>
    </LoggingEnabled>
</BucketLoggingStatus>"#;
        let parsed: BucketLoggingStatus = quick_xml::de::from_str(xml).unwrap();
        let le = parsed.logging_enabled.unwrap();
        assert_eq!(le.target_bucket, "mybucketlogs");
        assert_eq!(le.target_prefix.as_deref(), Some("mybucket-access_log/"));
    }

    #[test]
    fn parse_logging_disabled() {
        let xml = r#"<BucketLoggingStatus></BucketLoggingStatus>"#;
        let parsed: BucketLoggingStatus = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.logging_enabled.is_none());
    }
}

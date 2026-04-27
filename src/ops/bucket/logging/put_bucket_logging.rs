//! PutBucketLogging.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketlogging>

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
pub struct PutBucketLoggingParams {
    logging: OnlyKeyField,
}

/// `<LoggingEnabled>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoggingEnabled {
    pub target_bucket: String,
    pub target_prefix: Option<String>,
    pub logging_role: Option<String>,
}

/// Root `<BucketLoggingStatus>` element. `logging_enabled = None` disables
/// logging.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BucketLoggingStatus", rename_all = "PascalCase")]
pub struct BucketLoggingStatus {
    pub logging_enabled: Option<LoggingEnabled>,
}

pub struct PutBucketLogging {
    pub status: BucketLoggingStatus,
}

impl Ops for PutBucketLogging {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<BucketLoggingStatus>;
    type Query = PutBucketLoggingParams;

    fn prepare(self) -> Result<Prepared<PutBucketLoggingParams, BucketLoggingStatus>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketLoggingParams::default()),
            body: Some(self.status),
            ..Default::default()
        })
    }
}

pub trait PutBucketLoggingOps {
    /// Enable or update bucket access logging.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketlogging>
    fn put_bucket_logging(&self, status: BucketLoggingStatus) -> impl Future<Output = Result<()>>;
}

impl PutBucketLoggingOps for Client {
    async fn put_bucket_logging(&self, status: BucketLoggingStatus) -> Result<()> {
        self.request(PutBucketLogging { status }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketLoggingParams::default()).unwrap(), "logging");
    }

    #[test]
    fn body_round_trip_with_logging() {
        let status = BucketLoggingStatus {
            logging_enabled: Some(LoggingEnabled {
                target_bucket: "bucket-logs".to_string(),
                target_prefix: Some("access-log/".to_string()),
                logging_role: Some("AliyunOSSLoggingDefaultRole".to_string()),
            }),
        };
        let xml = quick_xml::se::to_string(&status).unwrap();
        assert!(xml.contains("<BucketLoggingStatus>"));
        assert!(xml.contains("<TargetBucket>bucket-logs</TargetBucket>"));
        let back: BucketLoggingStatus = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, status);
    }

    #[test]
    fn body_round_trip_disabled() {
        let status = BucketLoggingStatus::default();
        let xml = quick_xml::se::to_string(&status).unwrap();
        assert!(xml.contains("<BucketLoggingStatus"));
        let back: BucketLoggingStatus = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, status);
    }
}

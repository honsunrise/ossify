//! PutBucketAccessMonitor.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketaccessmonitor>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketAccessMonitorParams {
    accessmonitor: OnlyKeyField,
}

/// Access tracking status.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AccessMonitorStatus {
    #[default]
    Enabled,
    Disabled,
}

impl AccessMonitorStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessMonitorStatus::Enabled => "Enabled",
            AccessMonitorStatus::Disabled => "Disabled",
        }
    }
}

impl AsRef<str> for AccessMonitorStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Root `<AccessMonitorConfiguration>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AccessMonitorConfiguration", rename_all = "PascalCase")]
pub struct AccessMonitorConfiguration {
    pub status: AccessMonitorStatus,
}

pub struct PutBucketAccessMonitor {
    pub status: AccessMonitorStatus,
}

impl Ops for PutBucketAccessMonitor {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<AccessMonitorConfiguration>;
    type Query = PutBucketAccessMonitorParams;

    fn prepare(self) -> Result<Prepared<PutBucketAccessMonitorParams, AccessMonitorConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketAccessMonitorParams::default()),
            body: Some(AccessMonitorConfiguration { status: self.status }),
            ..Default::default()
        })
    }
}

pub trait PutBucketAccessMonitorOps {
    /// Enable or disable access tracking for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketaccessmonitor>
    fn put_bucket_access_monitor(&self, status: AccessMonitorStatus) -> impl Future<Output = Result<()>>;
}

impl PutBucketAccessMonitorOps for Client {
    async fn put_bucket_access_monitor(&self, status: AccessMonitorStatus) -> Result<()> {
        self.request(PutBucketAccessMonitor { status }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketAccessMonitorParams::default()).unwrap(),
            "accessmonitor"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = AccessMonitorConfiguration {
            status: AccessMonitorStatus::Enabled,
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Status>Enabled</Status>"));
        let back: AccessMonitorConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}

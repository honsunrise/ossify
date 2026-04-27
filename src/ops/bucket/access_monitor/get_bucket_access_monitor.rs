//! GetBucketAccessMonitor.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketaccessmonitor>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_access_monitor::AccessMonitorConfiguration;
#[allow(unused_imports)]
pub use super::put_bucket_access_monitor::AccessMonitorStatus;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketAccessMonitorParams {
    accessmonitor: OnlyKeyField,
}

pub struct GetBucketAccessMonitor;

impl Ops for GetBucketAccessMonitor {
    type Response = BodyResponseProcessor<AccessMonitorConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketAccessMonitorParams;

    fn prepare(self) -> Result<Prepared<GetBucketAccessMonitorParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketAccessMonitorParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketAccessMonitorOps {
    /// Query the bucket access-tracking status.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketaccessmonitor>
    fn get_bucket_access_monitor(&self) -> impl Future<Output = Result<AccessMonitorConfiguration>>;
}

impl GetBucketAccessMonitorOps for Client {
    async fn get_bucket_access_monitor(&self) -> Result<AccessMonitorConfiguration> {
        self.request(GetBucketAccessMonitor).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketAccessMonitorParams::default()).unwrap(),
            "accessmonitor"
        );
    }

    #[test]
    fn parse_response() {
        let xml = r#"<AccessMonitorConfiguration><Status>Disabled</Status></AccessMonitorConfiguration>"#;
        let parsed: AccessMonitorConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.status, AccessMonitorStatus::Disabled);
    }
}

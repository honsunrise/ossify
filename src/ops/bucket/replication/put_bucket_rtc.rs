//! PutBucketRTC: enable or disable Replication Time Control on an existing rule.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketrtc>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::{Rtc, RtcStatus};
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketRtcParams {
    rtc: OnlyKeyField,
}

/// Request body: `<ReplicationRule>` with RTC and rule ID.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "ReplicationRule", rename_all = "PascalCase")]
pub struct PutBucketRtcBody {
    #[serde(rename = "RTC")]
    pub rtc: Rtc,
    #[serde(rename = "ID")]
    pub id: String,
}

impl PutBucketRtcBody {
    pub fn new(rule_id: impl Into<String>, status: RtcStatus) -> Self {
        Self {
            rtc: Rtc { status },
            id: rule_id.into(),
        }
    }
}

pub struct PutBucketRtc {
    pub body: PutBucketRtcBody,
}

impl Ops for PutBucketRtc {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PutBucketRtcBody>;
    type Query = PutBucketRtcParams;

    fn prepare(self) -> Result<Prepared<PutBucketRtcParams, PutBucketRtcBody>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketRtcParams::default()),
            body: Some(self.body),
            ..Default::default()
        })
    }
}

pub trait PutBucketRtcOps {
    /// Enable or disable Replication Time Control for an existing CRR rule.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketrtc>
    fn put_bucket_rtc(
        &self,
        rule_id: impl Into<String>,
        status: RtcStatus,
    ) -> impl Future<Output = Result<()>>;
}

impl PutBucketRtcOps for Client {
    async fn put_bucket_rtc(&self, rule_id: impl Into<String>, status: RtcStatus) -> Result<()> {
        self.request(PutBucketRtc {
            body: PutBucketRtcBody::new(rule_id, status),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketRtcParams::default()).unwrap(), "rtc");
    }

    #[test]
    fn body_serialize() {
        let body = PutBucketRtcBody::new("rule-1", RtcStatus::Enabled);
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert!(xml.contains("<RTC>"));
        assert!(xml.contains("<Status>enabled</Status>"));
        assert!(xml.contains("<ID>rule-1</ID>"));
    }

    #[test]
    fn prepared_uses_put() {
        let prepared = PutBucketRtc {
            body: PutBucketRtcBody::new("r", RtcStatus::Disabled),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
    }
}

//! UpdateBucketAntiDDosInfo.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/updatebucketantiddosinfo>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

pub use super::init_bucket_anti_ddos_info::InitBucketAntiDDosInfoBody as UpdateBucketAntiDDosInfoBody;
use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::{AntiDdosCnames, AntiDdosStatus};
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateBucketAntiDDosInfoParams {
    #[serde(rename = "antiDDos")]
    anti_ddos: OnlyKeyField,
}

pub struct UpdateBucketAntiDDosInfo {
    pub instance_id: String,
    pub status: AntiDdosStatus,
    pub domains: Vec<String>,
}

impl Ops for UpdateBucketAntiDDosInfo {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<UpdateBucketAntiDDosInfoBody>;
    type Query = UpdateBucketAntiDDosInfoParams;

    fn prepare(self) -> Result<Prepared<UpdateBucketAntiDDosInfoParams, UpdateBucketAntiDDosInfoBody>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-defender-instance"), self.instance_id.parse()?);
        headers.insert(HeaderName::from_static("x-oss-defender-status"), self.status.as_str().parse()?);
        let cnames = if self.domains.is_empty() {
            None
        } else {
            Some(AntiDdosCnames {
                domains: self.domains,
            })
        };
        Ok(Prepared {
            method: Method::POST,
            query: Some(UpdateBucketAntiDDosInfoParams::default()),
            headers: Some(headers),
            body: Some(UpdateBucketAntiDDosInfoBody { cnames }),
            ..Default::default()
        })
    }
}

pub trait UpdateBucketAntiDDosInfoOps {
    /// Update the Anti-DDoS status or protected domain list of a bucket.
    ///
    /// - For `Init`, at least one domain must be supplied.
    /// - For `Defending`, domains are optional.
    /// - For `HaltDefending`, domains should be omitted.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/updatebucketantiddosinfo>
    fn update_bucket_anti_ddos_info(
        &self,
        instance_id: impl Into<String>,
        status: AntiDdosStatus,
        domains: Vec<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl UpdateBucketAntiDDosInfoOps for Client {
    async fn update_bucket_anti_ddos_info(
        &self,
        instance_id: impl Into<String>,
        status: AntiDdosStatus,
        domains: Vec<String>,
    ) -> Result<()> {
        self.request(UpdateBucketAntiDDosInfo {
            instance_id: instance_id.into(),
            status,
            domains,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&UpdateBucketAntiDDosInfoParams::default()).unwrap();
        assert_eq!(q, "antiDDos");
    }

    #[test]
    fn prepared_sets_headers() {
        let prepared = UpdateBucketAntiDDosInfo {
            instance_id: "id1".to_string(),
            status: AntiDdosStatus::Defending,
            domains: vec!["abc1.example.cn".to_string()],
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::POST);
        let h = prepared.headers.as_ref().unwrap();
        assert_eq!(h.get("x-oss-defender-instance").unwrap(), "id1");
        assert_eq!(h.get("x-oss-defender-status").unwrap(), "Defending");
        let body = prepared.body.as_ref().unwrap();
        let xml = quick_xml::se::to_string(body).unwrap();
        assert!(xml.contains("<Domain>abc1.example.cn</Domain>"));
    }
}

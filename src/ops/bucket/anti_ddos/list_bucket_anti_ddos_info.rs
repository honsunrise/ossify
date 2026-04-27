//! ListBucketAntiDDosInfo.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketantiddosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::AntiDdosListConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBucketAntiDDosInfoParams {
    #[serde(rename = "bucketAntiDDos")]
    bucket_anti_ddos: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "max-keys", skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<u32>,
}

impl ListBucketAntiDDosInfoParams {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct ListBucketAntiDDosInfo {
    pub marker: Option<String>,
    pub max_keys: Option<u32>,
}

impl Ops for ListBucketAntiDDosInfo {
    type Response = BodyResponseProcessor<AntiDdosListConfiguration>;
    type Body = NoneBody;
    type Query = ListBucketAntiDDosInfoParams;

    fn prepare(self) -> Result<Prepared<ListBucketAntiDDosInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(ListBucketAntiDDosInfoParams {
                bucket_anti_ddos: OnlyKeyField,
                marker: self.marker,
                max_keys: self.max_keys,
            }),
            ..Default::default()
        })
    }
}

pub trait ListBucketAntiDDosInfoOps {
    /// List the Anti-DDoS instances that protect this bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketantiddosinfo>
    fn list_bucket_anti_ddos_info(
        &self,
        marker: Option<String>,
        max_keys: Option<u32>,
    ) -> impl Future<Output = Result<AntiDdosListConfiguration>>;
}

impl ListBucketAntiDDosInfoOps for Client {
    async fn list_bucket_anti_ddos_info(
        &self,
        marker: Option<String>,
        max_keys: Option<u32>,
    ) -> Result<AntiDdosListConfiguration> {
        self.request(ListBucketAntiDDosInfo { marker, max_keys }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_default() {
        let q = crate::ser::to_string(&ListBucketAntiDDosInfoParams::default()).unwrap();
        assert_eq!(q, "bucketAntiDDos");
    }

    #[test]
    fn params_serialize_with_pagination() {
        let q = crate::ser::to_string(&ListBucketAntiDDosInfoParams {
            bucket_anti_ddos: OnlyKeyField,
            marker: Some("abc".to_string()),
            max_keys: Some(50),
        })
        .unwrap();
        assert_eq!(q, "bucketAntiDDos&marker=abc&max-keys=50");
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<AntiDDOSListConfiguration>
  <Marker>nextMarker</Marker>
  <IsTruncated>true</IsTruncated>
  <AntiDDOSConfiguration>
    <InstanceId>cbcac8d2-4f75-4d6d-9f2e-c3447f73****</InstanceId>
    <Owner>114893010724****</Owner>
    <Bucket>examplebucket</Bucket>
    <Ctime>1626769503</Ctime>
    <Mtime>1626769840</Mtime>
    <ActiveTime>1626769845</ActiveTime>
    <Status>Defending</Status>
    <Type>AntiDDosPremimum</Type>
    <Cnames>
      <Domain>abc1.example.cn</Domain>
      <Domain>abc2.example.cn</Domain>
    </Cnames>
  </AntiDDOSConfiguration>
</AntiDDOSListConfiguration>"#;
        let parsed: AntiDdosListConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.marker.as_deref(), Some("nextMarker"));
        assert_eq!(parsed.is_truncated, Some(true));
        assert_eq!(parsed.configurations.len(), 1);
        let cfg = &parsed.configurations[0];
        assert_eq!(cfg.bucket.as_deref(), Some("examplebucket"));
        assert_eq!(cfg.kind, Some(crate::ops::common::AntiDdosType::AntiDdosPremimum));
        let cnames = cfg.cnames.as_ref().unwrap();
        assert_eq!(cnames.domains.len(), 2);
        assert_eq!(cnames.domains[0], "abc1.example.cn");
    }
}

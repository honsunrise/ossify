//! ListAccessPoints.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listaccesspoints>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{AccessPointNetworkOrigin, AccessPointStatus, VpcConfiguration};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAccessPointsParams {
    #[serde(rename = "accessPoint")]
    access_point: OnlyKeyField,
    #[serde(rename = "max-keys", skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<u32>,
    #[serde(rename = "continuation-token", skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

/// Summary of a single access point inside `<AccessPoints>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AccessPoint", rename_all = "PascalCase")]
pub struct AccessPointSummary {
    pub bucket: String,
    pub access_point_name: String,
    pub alias: String,
    pub network_origin: AccessPointNetworkOrigin,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
    pub status: AccessPointStatus,
}

/// `<AccessPoints>` wrapper.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AccessPoints")]
pub struct AccessPointsList {
    #[serde(rename = "AccessPoint", default)]
    pub access_points: Vec<AccessPointSummary>,
}

/// Response body: `<ListAccessPointsResult>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ListAccessPointsResult", rename_all = "PascalCase")]
pub struct ListAccessPointsResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_points: Option<AccessPointsList>,
}

pub struct ListAccessPoints {
    pub max_keys: Option<u32>,
    pub continuation_token: Option<String>,
}

impl Ops for ListAccessPoints {
    type Response = BodyResponseProcessor<ListAccessPointsResult>;
    type Body = NoneBody;
    type Query = ListAccessPointsParams;

    fn prepare(self) -> Result<Prepared<ListAccessPointsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(ListAccessPointsParams {
                access_point: OnlyKeyField,
                max_keys: self.max_keys,
                continuation_token: self.continuation_token,
            }),
            ..Default::default()
        })
    }
}

pub trait ListAccessPointsOps {
    /// List bucket-level access points. Issue the request against a user-level
    /// host (no bucket) to receive user-level access points instead.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listaccesspoints>
    fn list_access_points(
        &self,
        max_keys: Option<u32>,
        continuation_token: Option<String>,
    ) -> impl Future<Output = Result<ListAccessPointsResult>>;
}

impl ListAccessPointsOps for Client {
    async fn list_access_points(
        &self,
        max_keys: Option<u32>,
        continuation_token: Option<String>,
    ) -> Result<ListAccessPointsResult> {
        self.request(ListAccessPoints {
            max_keys,
            continuation_token,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_default() {
        let q = crate::ser::to_string(&ListAccessPointsParams::default()).unwrap();
        assert_eq!(q, "accessPoint");
    }

    #[test]
    fn params_serialize_with_pagination() {
        let q = crate::ser::to_string(&ListAccessPointsParams {
            access_point: OnlyKeyField,
            max_keys: Some(10),
            continuation_token: Some("abc".to_string()),
        })
        .unwrap();
        assert_eq!(q, "accessPoint&continuation-token=abc&max-keys=10");
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAccessPointsResult>
  <IsTruncated>true</IsTruncated>
  <NextContinuationToken>abc</NextContinuationToken>
  <AccountId>111933544165****</AccountId>
  <AccessPoints>
    <AccessPoint>
      <Bucket>oss-example</Bucket>
      <AccessPointName>ap-01</AccessPointName>
      <Alias>ap-01-ossalias</Alias>
      <NetworkOrigin>vpc</NetworkOrigin>
      <VpcConfiguration>
        <VpcId>vpc-t4nlw426y44rd3iq4****</VpcId>
      </VpcConfiguration>
      <Status>enable</Status>
    </AccessPoint>
  </AccessPoints>
</ListAccessPointsResult>"#;
        let parsed: ListAccessPointsResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.is_truncated, Some(true));
        assert_eq!(parsed.next_continuation_token.as_deref(), Some("abc"));
        let aps = parsed.access_points.unwrap();
        assert_eq!(aps.access_points.len(), 1);
        assert_eq!(aps.access_points[0].access_point_name, "ap-01");
        assert_eq!(aps.access_points[0].status, AccessPointStatus::Enable);
    }
}

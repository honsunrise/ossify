//! ListAccessPointsForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listaccesspointsforobjectprocess>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::AccessPointForObjectProcessSummary;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAccessPointsForObjectProcessParams {
    #[serde(rename = "accessPointForObjectProcess")]
    access_point_for_object_process: OnlyKeyField,
    #[serde(rename = "max-keys", skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<u32>,
    #[serde(rename = "continuation-token", skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

/// `<AccessPointsForObjectProcess>` wrapper.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AccessPointsForObjectProcess")]
pub struct AccessPointsForObjectProcessList {
    #[serde(rename = "AccessPointForObjectProcess", default)]
    pub access_points: Vec<AccessPointForObjectProcessSummary>,
}

/// Response body: `<ListAccessPointsForObjectProcessResult>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ListAccessPointsForObjectProcessResult", rename_all = "PascalCase")]
pub struct ListAccessPointsForObjectProcessResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_points_for_object_process: Option<AccessPointsForObjectProcessList>,
}

pub struct ListAccessPointsForObjectProcess {
    pub max_keys: Option<u32>,
    pub continuation_token: Option<String>,
}

impl Ops for ListAccessPointsForObjectProcess {
    type Response = BodyResponseProcessor<ListAccessPointsForObjectProcessResult>;
    type Body = NoneBody;
    type Query = ListAccessPointsForObjectProcessParams;

    fn prepare(self) -> Result<Prepared<ListAccessPointsForObjectProcessParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(ListAccessPointsForObjectProcessParams {
                access_point_for_object_process: OnlyKeyField,
                max_keys: self.max_keys,
                continuation_token: self.continuation_token,
            }),
            ..Default::default()
        })
    }
}

pub trait ListAccessPointsForObjectProcessOps {
    /// List Object FC access points.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listaccesspointsforobjectprocess>
    fn list_access_points_for_object_process(
        &self,
        max_keys: Option<u32>,
        continuation_token: Option<String>,
    ) -> impl Future<Output = Result<ListAccessPointsForObjectProcessResult>>;
}

impl ListAccessPointsForObjectProcessOps for Client {
    async fn list_access_points_for_object_process(
        &self,
        max_keys: Option<u32>,
        continuation_token: Option<String>,
    ) -> Result<ListAccessPointsForObjectProcessResult> {
        self.request(ListAccessPointsForObjectProcess {
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
        let q = crate::ser::to_string(&ListAccessPointsForObjectProcessParams::default()).unwrap();
        assert_eq!(q, "accessPointForObjectProcess");
    }

    #[test]
    fn params_serialize_paginated() {
        let q = crate::ser::to_string(&ListAccessPointsForObjectProcessParams {
            access_point_for_object_process: OnlyKeyField,
            max_keys: Some(10),
            continuation_token: Some("abc".to_string()),
        })
        .unwrap();
        assert_eq!(q, "accessPointForObjectProcess&continuation-token=abc&max-keys=10");
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAccessPointsForObjectProcessResult>
  <IsTruncated>true</IsTruncated>
  <NextContinuationToken>abc</NextContinuationToken>
  <AccountId>111933544165****</AccountId>
  <AccessPointsForObjectProcess>
    <AccessPointForObjectProcess>
      <AccessPointNameForObjectProcess>fc-ap-01</AccessPointNameForObjectProcess>
      <AccessPointForObjectProcessAlias>fc-ap-01-xyz-opapalias</AccessPointForObjectProcessAlias>
      <AccessPointName>fc-01</AccessPointName>
      <Status>enable</Status>
    </AccessPointForObjectProcess>
  </AccessPointsForObjectProcess>
</ListAccessPointsForObjectProcessResult>"#;
        let parsed: ListAccessPointsForObjectProcessResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.is_truncated, Some(true));
        let aps = parsed.access_points_for_object_process.unwrap();
        assert_eq!(aps.access_points.len(), 1);
        assert_eq!(aps.access_points[0].access_point_name_for_object_process, "fc-ap-01");
    }
}

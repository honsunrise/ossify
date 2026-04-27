//! GetAccessPointForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointforobjectprocess>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{AccessPointStatus, ObjectFcEndpoints};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointForObjectProcessParams {
    #[serde(rename = "accessPointForObjectProcess")]
    access_point_for_object_process: OnlyKeyField,
}

/// Block Public Access descriptor attached to an Object FC access point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PublicAccessBlockConfiguration", rename_all = "PascalCase")]
pub struct ObjectFcPublicAccessBlockConfiguration {
    pub block_public_access: bool,
}

/// Response body: `<GetAccessPointForObjectProcessResult>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GetAccessPointForObjectProcessResult", rename_all = "PascalCase")]
pub struct GetAccessPointForObjectProcessResult {
    pub access_point_name_for_object_process: String,
    pub access_point_for_object_process_alias: String,
    pub access_point_name: String,
    pub account_id: String,
    pub access_point_for_object_process_arn: String,
    pub creation_date: String,
    pub status: AccessPointStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<ObjectFcEndpoints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<ObjectFcPublicAccessBlockConfiguration>,
}

pub struct GetAccessPointForObjectProcess {
    pub fc_ap_name: String,
}

impl Ops for GetAccessPointForObjectProcess {
    type Response = BodyResponseProcessor<GetAccessPointForObjectProcessResult>;
    type Body = NoneBody;
    type Query = GetAccessPointForObjectProcessParams;

    fn prepare(self) -> Result<Prepared<GetAccessPointForObjectProcessParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetAccessPointForObjectProcessParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait GetAccessPointForObjectProcessOps {
    /// Retrieve basic information about an Object FC access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointforobjectprocess>
    fn get_access_point_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> impl Future<Output = Result<GetAccessPointForObjectProcessResult>>;
}

impl GetAccessPointForObjectProcessOps for Client {
    async fn get_access_point_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> Result<GetAccessPointForObjectProcessResult> {
        self.request(GetAccessPointForObjectProcess {
            fc_ap_name: fc_ap_name.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetAccessPointForObjectProcessParams::default()).unwrap(),
            "accessPointForObjectProcess"
        );
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<GetAccessPointForObjectProcessResult>
  <AccessPointNameForObjectProcess>fc-ap-01</AccessPointNameForObjectProcess>
  <AccessPointForObjectProcessAlias>fc-ap-01-xyz-opapalias</AccessPointForObjectProcessAlias>
  <AccessPointName>ap-01</AccessPointName>
  <AccountId>111933544165****</AccountId>
  <AccessPointForObjectProcessArn>acs:oss:cn-qingdao:111933544165****:accesspointforobjectprocess/fc-ap-01</AccessPointForObjectProcessArn>
  <CreationDate>1626769503</CreationDate>
  <Status>enable</Status>
  <Endpoints>
    <PublicEndpoint>fc-ap-01.public</PublicEndpoint>
    <InternalEndpoint>fc-ap-01.internal</InternalEndpoint>
  </Endpoints>
  <PublicAccessBlockConfiguration>
    <BlockPublicAccess>true</BlockPublicAccess>
  </PublicAccessBlockConfiguration>
</GetAccessPointForObjectProcessResult>"#;
        let parsed: GetAccessPointForObjectProcessResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.access_point_name_for_object_process, "fc-ap-01");
        assert_eq!(parsed.status, AccessPointStatus::Enable);
        assert!(parsed.endpoints.is_some());
        assert!(
            parsed
                .public_access_block_configuration
                .unwrap()
                .block_public_access
        );
    }
}

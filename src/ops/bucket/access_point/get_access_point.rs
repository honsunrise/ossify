//! GetAccessPoint.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspoint>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{AccessPointNetworkOrigin, AccessPointStatus, VpcConfiguration};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointParams {
    #[serde(rename = "accessPoint")]
    access_point: OnlyKeyField,
}

/// Endpoints of the access point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Endpoints", rename_all = "PascalCase")]
pub struct AccessPointEndpoints {
    pub public_endpoint: String,
    pub internal_endpoint: String,
}

/// Block Public Access configuration attached to an access point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PublicAccessBlockConfiguration", rename_all = "PascalCase")]
pub struct AccessPointPublicAccessBlockConfiguration {
    pub block_public_access: bool,
}

/// Response body: `<GetAccessPointResult>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GetAccessPointResult", rename_all = "PascalCase")]
pub struct GetAccessPointResult {
    pub access_point_name: String,
    pub bucket: String,
    pub account_id: String,
    pub network_origin: AccessPointNetworkOrigin,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
    pub access_point_arn: String,
    pub creation_date: String,
    pub alias: String,
    pub status: AccessPointStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<AccessPointEndpoints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<AccessPointPublicAccessBlockConfiguration>,
}

pub struct GetAccessPoint {
    pub name: String,
}

impl Ops for GetAccessPoint {
    type Response = BodyResponseProcessor<GetAccessPointResult>;
    type Body = NoneBody;
    type Query = GetAccessPointParams;

    fn prepare(self) -> Result<Prepared<GetAccessPointParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-access-point-name"), self.name.parse()?);
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetAccessPointParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait GetAccessPointOps {
    /// Retrieve the full configuration of a named access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspoint>
    fn get_access_point(&self, name: impl Into<String>)
    -> impl Future<Output = Result<GetAccessPointResult>>;
}

impl GetAccessPointOps for Client {
    async fn get_access_point(&self, name: impl Into<String>) -> Result<GetAccessPointResult> {
        self.request(GetAccessPoint { name: name.into() }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetAccessPointParams::default()).unwrap(), "accessPoint");
    }

    #[test]
    fn prepared_sets_header() {
        let p = GetAccessPoint {
            name: "ap-01".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(
            p.headers
                .as_ref()
                .unwrap()
                .get("x-oss-access-point-name")
                .unwrap(),
            "ap-01"
        );
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<GetAccessPointResult>
  <AccessPointName>ap-01</AccessPointName>
  <Bucket>oss-example</Bucket>
  <AccountId>111933544165xxxx</AccountId>
  <NetworkOrigin>vpc</NetworkOrigin>
  <VpcConfiguration>
    <VpcId>vpc-t4nlw426y44rd3iq4xxxx</VpcId>
  </VpcConfiguration>
  <AccessPointArn>arn:acs:oss:cn-hangzhou:111933544165xxxx:accesspoint/ap-01</AccessPointArn>
  <CreationDate>1626769503</CreationDate>
  <Alias>ap-01-ossalias</Alias>
  <Status>enable</Status>
  <Endpoints>
    <PublicEndpoint>ap-01.oss-cn-hangzhou.oss-accesspoint.aliyuncs.com</PublicEndpoint>
    <InternalEndpoint>ap-01.oss-cn-hangzhou-internal.oss-accesspoint.aliyuncs.com</InternalEndpoint>
  </Endpoints>
  <PublicAccessBlockConfiguration>
    <BlockPublicAccess>true</BlockPublicAccess>
  </PublicAccessBlockConfiguration>
</GetAccessPointResult>"#;
        let parsed: GetAccessPointResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.access_point_name, "ap-01");
        assert_eq!(parsed.network_origin, AccessPointNetworkOrigin::Vpc);
        assert_eq!(parsed.status, AccessPointStatus::Enable);
        assert!(parsed.endpoints.is_some());
        assert!(
            parsed
                .public_access_block_configuration
                .as_ref()
                .unwrap()
                .block_public_access
        );
    }
}

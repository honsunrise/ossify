//! CreateAccessPoint.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createaccesspoint>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::{AccessPointNetworkOrigin, VpcConfiguration};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessPointParams {
    #[serde(rename = "accessPoint")]
    access_point: OnlyKeyField,
}

/// Request body: `<CreateAccessPointConfiguration>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CreateAccessPointConfiguration", rename_all = "PascalCase")]
pub struct CreateAccessPointConfiguration {
    pub access_point_name: String,
    pub network_origin: AccessPointNetworkOrigin,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

/// Response body: `<CreateAccessPointResult>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CreateAccessPointResult", rename_all = "PascalCase")]
pub struct CreateAccessPointResult {
    pub access_point_arn: String,
    pub alias: String,
}

pub struct CreateAccessPoint {
    pub config: CreateAccessPointConfiguration,
}

impl Ops for CreateAccessPoint {
    type Response = BodyResponseProcessor<CreateAccessPointResult>;
    type Body = XMLBody<CreateAccessPointConfiguration>;
    type Query = CreateAccessPointParams;

    fn prepare(self) -> Result<Prepared<CreateAccessPointParams, CreateAccessPointConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(CreateAccessPointParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait CreateAccessPointOps {
    /// Create an access point for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createaccesspoint>
    fn create_access_point(
        &self,
        config: CreateAccessPointConfiguration,
    ) -> impl Future<Output = Result<CreateAccessPointResult>>;
}

impl CreateAccessPointOps for Client {
    async fn create_access_point(
        &self,
        config: CreateAccessPointConfiguration,
    ) -> Result<CreateAccessPointResult> {
        self.request(CreateAccessPoint { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&CreateAccessPointParams::default()).unwrap(),
            "accessPoint"
        );
    }

    #[test]
    fn body_serializes() {
        let cfg = CreateAccessPointConfiguration {
            access_point_name: "ap-01".to_string(),
            network_origin: AccessPointNetworkOrigin::Vpc,
            vpc_configuration: Some(VpcConfiguration {
                vpc_id: "vpc-abc".to_string(),
            }),
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<CreateAccessPointConfiguration>"));
        assert!(xml.contains("<AccessPointName>ap-01</AccessPointName>"));
        assert!(xml.contains("<NetworkOrigin>vpc</NetworkOrigin>"));
        assert!(xml.contains("<VpcId>vpc-abc</VpcId>"));
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CreateAccessPointResult>
  <AccessPointArn>acs:oss:cn-hangzhou:128364106451xxxx:accesspoint/ap-01</AccessPointArn>
  <Alias>ap-01-45ee7945007a2f0bcb595f63e2215cxxxx-ossalias</Alias>
</CreateAccessPointResult>"#;
        let parsed: CreateAccessPointResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            parsed.access_point_arn,
            "acs:oss:cn-hangzhou:128364106451xxxx:accesspoint/ap-01"
        );
        assert_eq!(parsed.alias, "ap-01-45ee7945007a2f0bcb595f63e2215cxxxx-ossalias");
    }
}

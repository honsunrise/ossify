//! CreateAccessPointForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createaccesspointforobjectprocess>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::ObjectProcessConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessPointForObjectProcessParams {
    #[serde(rename = "accessPointForObjectProcess")]
    access_point_for_object_process: OnlyKeyField,
}

/// Request body: `<CreateAccessPointForObjectProcessConfiguration>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    rename = "CreateAccessPointForObjectProcessConfiguration",
    rename_all = "PascalCase"
)]
pub struct CreateAccessPointForObjectProcessConfiguration {
    pub access_point_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_process_configuration: Option<ObjectProcessConfiguration>,
}

/// Response body: `<CreateAccessPointForObjectProcessResult>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CreateAccessPointForObjectProcessResult", rename_all = "PascalCase")]
pub struct CreateAccessPointForObjectProcessResult {
    pub access_point_for_object_process_arn: String,
    pub access_point_for_object_process_alias: String,
}

pub struct CreateAccessPointForObjectProcess {
    pub fc_ap_name: String,
    pub config: CreateAccessPointForObjectProcessConfiguration,
}

impl Ops for CreateAccessPointForObjectProcess {
    type Response = BodyResponseProcessor<CreateAccessPointForObjectProcessResult>;
    type Body = XMLBody<CreateAccessPointForObjectProcessConfiguration>;
    type Query = CreateAccessPointForObjectProcessParams;

    fn prepare(
        self,
    ) -> Result<
        Prepared<CreateAccessPointForObjectProcessParams, CreateAccessPointForObjectProcessConfiguration>,
    > {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::PUT,
            query: Some(CreateAccessPointForObjectProcessParams::default()),
            headers: Some(headers),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait CreateAccessPointForObjectProcessOps {
    /// Create an Object FC access point backed by a Function Compute function.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createaccesspointforobjectprocess>
    fn create_access_point_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
        config: CreateAccessPointForObjectProcessConfiguration,
    ) -> impl Future<Output = Result<CreateAccessPointForObjectProcessResult>>;
}

impl CreateAccessPointForObjectProcessOps for Client {
    async fn create_access_point_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
        config: CreateAccessPointForObjectProcessConfiguration,
    ) -> Result<CreateAccessPointForObjectProcessResult> {
        self.request(CreateAccessPointForObjectProcess {
            fc_ap_name: fc_ap_name.into(),
            config,
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
            crate::ser::to_string(&CreateAccessPointForObjectProcessParams::default()).unwrap(),
            "accessPointForObjectProcess"
        );
    }

    #[test]
    fn body_serializes_bare() {
        let cfg = CreateAccessPointForObjectProcessConfiguration {
            access_point_name: "ap-01".to_string(),
            object_process_configuration: None,
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        eprintln!("XML: {xml}");
        assert!(xml.contains("<CreateAccessPointForObjectProcessConfiguration>"));
        assert!(xml.contains("<AccessPointName>ap-01</AccessPointName>"));
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CreateAccessPointForObjectProcessResult>
  <AccessPointForObjectProcessArn>acs:oss:cn-qingdao:119335441657143:accesspointforobjectprocess/fc-ap-01</AccessPointForObjectProcessArn>
  <AccessPointForObjectProcessAlias>fc-ap-01-xyz-opapalias</AccessPointForObjectProcessAlias>
</CreateAccessPointForObjectProcessResult>"#;
        let parsed: CreateAccessPointForObjectProcessResult = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.access_point_for_object_process_arn.contains("fc-ap-01"));
        assert!(parsed.access_point_for_object_process_alias.contains("opapalias"));
    }
}

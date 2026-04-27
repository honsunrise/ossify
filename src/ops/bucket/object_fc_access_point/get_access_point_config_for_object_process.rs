//! GetAccessPointConfigForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointconfigforobjectprocess>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

pub use super::get_access_point_for_object_process::ObjectFcPublicAccessBlockConfiguration;
use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::ObjectProcessConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointConfigForObjectProcessParams {
    #[serde(rename = "accessPointConfigForObjectProcess")]
    access_point_config_for_object_process: OnlyKeyField,
}

/// Response body: `<GetAccessPointConfigForObjectProcessResult>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    rename = "GetAccessPointConfigForObjectProcessResult",
    rename_all = "PascalCase"
)]
pub struct GetAccessPointConfigForObjectProcessResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_process_configuration: Option<ObjectProcessConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<ObjectFcPublicAccessBlockConfiguration>,
}

pub struct GetAccessPointConfigForObjectProcess {
    pub fc_ap_name: String,
}

impl Ops for GetAccessPointConfigForObjectProcess {
    type Response = BodyResponseProcessor<GetAccessPointConfigForObjectProcessResult>;
    type Body = NoneBody;
    type Query = GetAccessPointConfigForObjectProcessParams;

    fn prepare(self) -> Result<Prepared<GetAccessPointConfigForObjectProcessParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetAccessPointConfigForObjectProcessParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait GetAccessPointConfigForObjectProcessOps {
    /// Retrieve the current configuration of an Object FC access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointconfigforobjectprocess>
    fn get_access_point_config_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> impl Future<Output = Result<GetAccessPointConfigForObjectProcessResult>>;
}

impl GetAccessPointConfigForObjectProcessOps for Client {
    async fn get_access_point_config_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> Result<GetAccessPointConfigForObjectProcessResult> {
        self.request(GetAccessPointConfigForObjectProcess {
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
            crate::ser::to_string(&GetAccessPointConfigForObjectProcessParams::default()).unwrap(),
            "accessPointConfigForObjectProcess"
        );
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<GetAccessPointConfigForObjectProcessResult>
  <ObjectProcessConfiguration>
    <AllowedFeatures/>
    <TransformationConfigurations>
      <TransformationConfiguration>
        <Actions>
          <Action>getobject</Action>
        </Actions>
        <ContentTransformation>
          <FunctionCompute>
            <FunctionAssumeRoleArn>arn-role</FunctionAssumeRoleArn>
            <FunctionArn>arn-fn</FunctionArn>
          </FunctionCompute>
        </ContentTransformation>
      </TransformationConfiguration>
    </TransformationConfigurations>
  </ObjectProcessConfiguration>
  <PublicAccessBlockConfiguration>
    <BlockPublicAccess>true</BlockPublicAccess>
  </PublicAccessBlockConfiguration>
</GetAccessPointConfigForObjectProcessResult>"#;
        let parsed: GetAccessPointConfigForObjectProcessResult = quick_xml::de::from_str(xml).unwrap();
        let opc = parsed.object_process_configuration.unwrap();
        let tc = opc.transformation_configurations.unwrap();
        assert_eq!(tc.configurations.len(), 1);
        assert_eq!(
            tc.configurations[0]
                .content_transformation
                .function_compute
                .function_arn,
            "arn-fn"
        );
        assert!(
            parsed
                .public_access_block_configuration
                .unwrap()
                .block_public_access
        );
    }
}

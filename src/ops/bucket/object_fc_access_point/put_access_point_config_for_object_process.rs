//! PutAccessPointConfigForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointconfigforobjectprocess>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

pub use super::get_access_point_for_object_process::ObjectFcPublicAccessBlockConfiguration;
use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::ObjectProcessConfiguration;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAccessPointConfigForObjectProcessParams {
    #[serde(rename = "accessPointConfigForObjectProcess")]
    access_point_config_for_object_process: OnlyKeyField,
}

/// Request body: `<PutAccessPointConfigForObjectProcessConfiguration>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    rename = "PutAccessPointConfigForObjectProcessConfiguration",
    rename_all = "PascalCase"
)]
pub struct PutAccessPointConfigForObjectProcessConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_process_configuration: Option<ObjectProcessConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<ObjectFcPublicAccessBlockConfiguration>,
}

pub struct PutAccessPointConfigForObjectProcess {
    pub fc_ap_name: String,
    pub config: PutAccessPointConfigForObjectProcessConfiguration,
}

impl Ops for PutAccessPointConfigForObjectProcess {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PutAccessPointConfigForObjectProcessConfiguration>;
    type Query = PutAccessPointConfigForObjectProcessParams;

    fn prepare(
        self,
    ) -> Result<
        Prepared<
            PutAccessPointConfigForObjectProcessParams,
            PutAccessPointConfigForObjectProcessConfiguration,
        >,
    > {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutAccessPointConfigForObjectProcessParams::default()),
            headers: Some(headers),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutAccessPointConfigForObjectProcessOps {
    /// Change the configuration of an Object FC access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointconfigforobjectprocess>
    fn put_access_point_config_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
        config: PutAccessPointConfigForObjectProcessConfiguration,
    ) -> impl Future<Output = Result<()>>;
}

impl PutAccessPointConfigForObjectProcessOps for Client {
    async fn put_access_point_config_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
        config: PutAccessPointConfigForObjectProcessConfiguration,
    ) -> Result<()> {
        self.request(PutAccessPointConfigForObjectProcess {
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
            crate::ser::to_string(&PutAccessPointConfigForObjectProcessParams::default()).unwrap(),
            "accessPointConfigForObjectProcess"
        );
    }

    #[test]
    fn body_serializes() {
        let cfg = PutAccessPointConfigForObjectProcessConfiguration {
            object_process_configuration: None,
            public_access_block_configuration: Some(ObjectFcPublicAccessBlockConfiguration {
                block_public_access: true,
            }),
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<BlockPublicAccess>true</BlockPublicAccess>"));
    }
}

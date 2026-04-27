//! GetAccessPointPolicyForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointpolicyforobjectprocess>

use std::future::Future;

use bytes::Bytes;
use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BinaryResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointPolicyForObjectProcessParams {
    #[serde(rename = "accessPointPolicyForObjectProcess")]
    access_point_policy_for_object_process: OnlyKeyField,
}

pub struct GetAccessPointPolicyForObjectProcess {
    pub fc_ap_name: String,
}

impl Ops for GetAccessPointPolicyForObjectProcess {
    type Response = BinaryResponseProcessor;
    type Body = NoneBody;
    type Query = GetAccessPointPolicyForObjectProcessParams;

    fn prepare(self) -> Result<Prepared<GetAccessPointPolicyForObjectProcessParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetAccessPointPolicyForObjectProcessParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait GetAccessPointPolicyForObjectProcessOps {
    /// Retrieve the raw JSON policy document attached to an Object FC access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointpolicyforobjectprocess>
    fn get_access_point_policy_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> impl Future<Output = Result<Bytes>>;
}

impl GetAccessPointPolicyForObjectProcessOps for Client {
    async fn get_access_point_policy_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> Result<Bytes> {
        self.request(GetAccessPointPolicyForObjectProcess {
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
            crate::ser::to_string(&GetAccessPointPolicyForObjectProcessParams::default()).unwrap(),
            "accessPointPolicyForObjectProcess"
        );
    }

    #[test]
    fn method_is_get() {
        let p = GetAccessPointPolicyForObjectProcess {
            fc_ap_name: "fc-ap-01".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::GET);
    }
}

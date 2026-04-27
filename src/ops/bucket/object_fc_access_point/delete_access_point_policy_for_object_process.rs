//! DeleteAccessPointPolicyForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointpolicyforobjectprocess>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointPolicyForObjectProcessParams {
    #[serde(rename = "accessPointPolicyForObjectProcess")]
    access_point_policy_for_object_process: OnlyKeyField,
}

pub struct DeleteAccessPointPolicyForObjectProcess {
    pub fc_ap_name: String,
}

impl Ops for DeleteAccessPointPolicyForObjectProcess {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteAccessPointPolicyForObjectProcessParams;

    fn prepare(self) -> Result<Prepared<DeleteAccessPointPolicyForObjectProcessParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteAccessPointPolicyForObjectProcessParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait DeleteAccessPointPolicyForObjectProcessOps {
    /// Delete the policy attached to an Object FC access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointpolicyforobjectprocess>
    fn delete_access_point_policy_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteAccessPointPolicyForObjectProcessOps for Client {
    async fn delete_access_point_policy_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> Result<()> {
        self.request(DeleteAccessPointPolicyForObjectProcess {
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
            crate::ser::to_string(&DeleteAccessPointPolicyForObjectProcessParams::default()).unwrap(),
            "accessPointPolicyForObjectProcess"
        );
    }

    #[test]
    fn method_is_delete() {
        let p = DeleteAccessPointPolicyForObjectProcess {
            fc_ap_name: "fc-ap-01".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::DELETE);
    }
}

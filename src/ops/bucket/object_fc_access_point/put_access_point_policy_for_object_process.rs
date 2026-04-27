//! PutAccessPointPolicyForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointpolicyforobjectprocess>

use std::future::Future;

use bytes::Bytes;
use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::BytesBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAccessPointPolicyForObjectProcessParams {
    #[serde(rename = "accessPointPolicyForObjectProcess")]
    access_point_policy_for_object_process: OnlyKeyField,
}

pub struct PutAccessPointPolicyForObjectProcess {
    pub fc_ap_name: String,
    pub policy: Bytes,
}

impl Ops for PutAccessPointPolicyForObjectProcess {
    type Response = EmptyResponseProcessor;
    type Body = BytesBody;
    type Query = PutAccessPointPolicyForObjectProcessParams;

    fn prepare(self) -> Result<Prepared<PutAccessPointPolicyForObjectProcessParams, (Bytes, &'static str)>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutAccessPointPolicyForObjectProcessParams::default()),
            headers: Some(headers),
            body: Some((self.policy, "application/json")),
            ..Default::default()
        })
    }
}

pub trait PutAccessPointPolicyForObjectProcessOps {
    /// Attach a JSON access policy to an Object FC access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointpolicyforobjectprocess>
    fn put_access_point_policy_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
        policy: impl Into<Bytes>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutAccessPointPolicyForObjectProcessOps for Client {
    async fn put_access_point_policy_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
        policy: impl Into<Bytes>,
    ) -> Result<()> {
        self.request(PutAccessPointPolicyForObjectProcess {
            fc_ap_name: fc_ap_name.into(),
            policy: policy.into(),
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
            crate::ser::to_string(&PutAccessPointPolicyForObjectProcessParams::default()).unwrap(),
            "accessPointPolicyForObjectProcess"
        );
    }

    #[test]
    fn prepared_uses_put_with_json() {
        let p = PutAccessPointPolicyForObjectProcess {
            fc_ap_name: "fc-ap-01".to_string(),
            policy: Bytes::from_static(br#"{}"#),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::PUT);
        let (_, ct) = p.body.unwrap();
        assert_eq!(ct, "application/json");
    }
}

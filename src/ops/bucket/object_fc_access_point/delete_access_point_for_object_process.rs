//! DeleteAccessPointForObjectProcess.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointforobjectprocess>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointForObjectProcessParams {
    #[serde(rename = "accessPointForObjectProcess")]
    access_point_for_object_process: OnlyKeyField,
}

pub struct DeleteAccessPointForObjectProcess {
    pub fc_ap_name: String,
}

impl Ops for DeleteAccessPointForObjectProcess {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteAccessPointForObjectProcessParams;

    fn prepare(self) -> Result<Prepared<DeleteAccessPointForObjectProcessParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-oss-access-point-for-object-process-name"),
            self.fc_ap_name.parse()?,
        );
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteAccessPointForObjectProcessParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait DeleteAccessPointForObjectProcessOps {
    /// Delete an Object FC access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointforobjectprocess>
    fn delete_access_point_for_object_process(
        &self,
        fc_ap_name: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteAccessPointForObjectProcessOps for Client {
    async fn delete_access_point_for_object_process(&self, fc_ap_name: impl Into<String>) -> Result<()> {
        self.request(DeleteAccessPointForObjectProcess {
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
            crate::ser::to_string(&DeleteAccessPointForObjectProcessParams::default()).unwrap(),
            "accessPointForObjectProcess"
        );
    }

    #[test]
    fn method_and_header() {
        let p = DeleteAccessPointForObjectProcess {
            fc_ap_name: "fc-ap-01".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::DELETE);
        assert_eq!(
            p.headers
                .as_ref()
                .unwrap()
                .get("x-oss-access-point-for-object-process-name")
                .unwrap(),
            "fc-ap-01"
        );
    }
}

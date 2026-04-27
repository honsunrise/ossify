//! UpdateUserAntiDDosInfo.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/updateuserantiddosinfo>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::ops::common::AntiDdosStatus;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserAntiDDosInfoParams {
    #[serde(rename = "antiDDos")]
    anti_ddos: OnlyKeyField,
}

pub struct UpdateUserAntiDDosInfo {
    pub instance_id: String,
    pub status: AntiDdosStatus,
}

impl Ops for UpdateUserAntiDDosInfo {
    const USE_BUCKET: bool = false;

    type Response = EmptyResponseProcessor;
    type Body = ZeroBody;
    type Query = UpdateUserAntiDDosInfoParams;

    fn prepare(self) -> Result<Prepared<UpdateUserAntiDDosInfoParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-defender-instance"), self.instance_id.parse()?);
        headers.insert(HeaderName::from_static("x-oss-defender-status"), self.status.as_str().parse()?);
        Ok(Prepared {
            method: Method::POST,
            query: Some(UpdateUserAntiDDosInfoParams::default()),
            headers: Some(headers),
            body: Some(()),
            ..Default::default()
        })
    }
}

pub trait UpdateUserAntiDDosInfoOps {
    /// Update the status of an account-level Anti-DDoS instance.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/updateuserantiddosinfo>
    fn update_user_anti_ddos_info(
        &self,
        instance_id: impl Into<String>,
        status: AntiDdosStatus,
    ) -> impl Future<Output = Result<()>>;
}

impl UpdateUserAntiDDosInfoOps for Client {
    async fn update_user_anti_ddos_info(
        &self,
        instance_id: impl Into<String>,
        status: AntiDdosStatus,
    ) -> Result<()> {
        self.request(UpdateUserAntiDDosInfo {
            instance_id: instance_id.into(),
            status,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&UpdateUserAntiDDosInfoParams::default()).unwrap();
        assert_eq!(q, "antiDDos");
    }

    #[test]
    fn prepared_sets_headers() {
        let prepared = UpdateUserAntiDDosInfo {
            instance_id: "cbcac8d2-4f75-4d6d-9f2e-c3447f73****".to_string(),
            status: AntiDdosStatus::HaltDefending,
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::POST);
        let h = prepared.headers.as_ref().unwrap();
        assert_eq!(
            h.get("x-oss-defender-instance").unwrap(),
            "cbcac8d2-4f75-4d6d-9f2e-c3447f73****"
        );
        assert_eq!(h.get("x-oss-defender-status").unwrap(), "HaltDefending");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<UpdateUserAntiDDosInfo as Ops>::USE_BUCKET);
    }
}

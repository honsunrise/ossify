//! InitUserAntiDDosInfo.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/inituserantiddosinfo>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct InitUserAntiDDosInfoParams {
    #[serde(rename = "antiDDos")]
    anti_ddos: OnlyKeyField,
}

/// Parsed from the `x-oss-defender-instance` response header.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct InitUserAntiDDosInfoResponse {
    #[serde(rename = "x-oss-defender-instance")]
    pub defender_instance: String,
}

pub struct InitUserAntiDDosInfo;

impl Ops for InitUserAntiDDosInfo {
    const USE_BUCKET: bool = false;

    type Response = HeaderResponseProcessor<InitUserAntiDDosInfoResponse>;
    type Body = ZeroBody;
    type Query = InitUserAntiDDosInfoParams;

    fn prepare(self) -> Result<Prepared<InitUserAntiDDosInfoParams>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(InitUserAntiDDosInfoParams::default()),
            body: Some(()),
            ..Default::default()
        })
    }
}

pub trait InitUserAntiDDosInfoOps {
    /// Create an OSS Anti-DDoS instance for the current account.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/inituserantiddosinfo>
    fn init_user_anti_ddos_info(&self) -> impl Future<Output = Result<InitUserAntiDDosInfoResponse>>;
}

impl InitUserAntiDDosInfoOps for Client {
    async fn init_user_anti_ddos_info(&self) -> Result<InitUserAntiDDosInfoResponse> {
        self.request(InitUserAntiDDosInfo).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&InitUserAntiDDosInfoParams::default()).unwrap();
        assert_eq!(q, "antiDDos");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<InitUserAntiDDosInfo as Ops>::USE_BUCKET);
    }

    #[test]
    fn method_is_put() {
        let p = InitUserAntiDDosInfo.prepare().unwrap();
        assert_eq!(p.method, Method::PUT);
    }
}

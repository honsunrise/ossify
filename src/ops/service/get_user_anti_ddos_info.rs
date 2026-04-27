//! GetUserAntiDDosInfo.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getuserantiddosinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::AntiDdosListConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserAntiDDosInfoParams {
    #[serde(rename = "antiDDos")]
    anti_ddos: OnlyKeyField,
}

pub struct GetUserAntiDDosInfo;

impl Ops for GetUserAntiDDosInfo {
    const USE_BUCKET: bool = false;

    type Response = BodyResponseProcessor<AntiDdosListConfiguration>;
    type Body = NoneBody;
    type Query = GetUserAntiDDosInfoParams;

    fn prepare(self) -> Result<Prepared<GetUserAntiDDosInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetUserAntiDDosInfoParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetUserAntiDDosInfoOps {
    /// List all account-level Anti-DDoS instances.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getuserantiddosinfo>
    fn get_user_anti_ddos_info(&self) -> impl Future<Output = Result<AntiDdosListConfiguration>>;
}

impl GetUserAntiDDosInfoOps for Client {
    async fn get_user_anti_ddos_info(&self) -> Result<AntiDdosListConfiguration> {
        self.request(GetUserAntiDDosInfo).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetUserAntiDDosInfoParams::default()).unwrap();
        assert_eq!(q, "antiDDos");
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<GetUserAntiDDosInfo as Ops>::USE_BUCKET);
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<AntiDDOSListConfiguration>
    <AntiDDOSConfiguration>
        <InstanceId>cbcac8d2-4f75-4d6d-9f2e-c3447f73****</InstanceId>
        <Owner>114893010724****</Owner>
        <Ctime>12345667</Ctime>
        <Mtime>12345667</Mtime>
        <ActiveTime>12345680</ActiveTime>
        <Status>Init</Status>
    </AntiDDOSConfiguration>
</AntiDDOSListConfiguration>"#;
        let parsed: AntiDdosListConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.configurations.len(), 1);
        let cfg = &parsed.configurations[0];
        assert_eq!(cfg.instance_id, "cbcac8d2-4f75-4d6d-9f2e-c3447f73****");
        assert_eq!(cfg.owner.as_deref(), Some("114893010724****"));
        assert_eq!(cfg.status, Some(crate::ops::common::AntiDdosStatus::Init));
    }
}

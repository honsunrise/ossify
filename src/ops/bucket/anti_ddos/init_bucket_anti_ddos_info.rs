//! InitBucketAntiDDosInfo.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/initbucketantiddosinfo>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::{AntiDdosCnames, AntiDdosType};
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct InitBucketAntiDDosInfoParams {
    #[serde(rename = "antiDDos")]
    anti_ddos: OnlyKeyField,
}

/// Request body: `<AntiDDOSConfiguration><Cnames><Domain>…</Domain></Cnames></AntiDDOSConfiguration>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AntiDDOSConfiguration", rename_all = "PascalCase")]
pub struct InitBucketAntiDDosInfoBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnames: Option<AntiDdosCnames>,
}

pub struct InitBucketAntiDDosInfo {
    pub instance_id: String,
    pub kind: AntiDdosType,
    pub domains: Vec<String>,
}

impl Ops for InitBucketAntiDDosInfo {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<InitBucketAntiDDosInfoBody>;
    type Query = InitBucketAntiDDosInfoParams;

    fn prepare(self) -> Result<Prepared<InitBucketAntiDDosInfoParams, InitBucketAntiDDosInfoBody>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-defender-instance"), self.instance_id.parse()?);
        headers.insert(HeaderName::from_static("x-oss-defender-type"), self.kind.as_str().parse()?);
        let cnames = if self.domains.is_empty() {
            None
        } else {
            Some(AntiDdosCnames {
                domains: self.domains,
            })
        };
        Ok(Prepared {
            method: Method::PUT,
            query: Some(InitBucketAntiDDosInfoParams::default()),
            headers: Some(headers),
            body: Some(InitBucketAntiDDosInfoBody { cnames }),
            ..Default::default()
        })
    }
}

pub trait InitBucketAntiDDosInfoOps {
    /// Initialise an Anti-DDoS instance for the bucket, optionally whitelisting
    /// custom domain names to be protected.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/initbucketantiddosinfo>
    fn init_bucket_anti_ddos_info(
        &self,
        instance_id: impl Into<String>,
        kind: AntiDdosType,
        domains: Vec<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl InitBucketAntiDDosInfoOps for Client {
    async fn init_bucket_anti_ddos_info(
        &self,
        instance_id: impl Into<String>,
        kind: AntiDdosType,
        domains: Vec<String>,
    ) -> Result<()> {
        self.request(InitBucketAntiDDosInfo {
            instance_id: instance_id.into(),
            kind,
            domains,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&InitBucketAntiDDosInfoParams::default()).unwrap();
        assert_eq!(q, "antiDDos");
    }

    #[test]
    fn prepared_sets_headers_and_body() {
        let prepared = InitBucketAntiDDosInfo {
            instance_id: "cbcac8d2-4f75-4d6d-9f2e-c3447f73****".to_string(),
            kind: AntiDdosType::AntiDdosPremimum,
            domains: vec!["abc1.example.cn".to_string(), "abc2.example.cn".to_string()],
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
        let h = prepared.headers.as_ref().unwrap();
        assert_eq!(
            h.get("x-oss-defender-instance").unwrap(),
            "cbcac8d2-4f75-4d6d-9f2e-c3447f73****"
        );
        assert_eq!(h.get("x-oss-defender-type").unwrap(), "AntiDDosPremimum");
        let body = prepared.body.as_ref().unwrap();
        let xml = quick_xml::se::to_string(body).unwrap();
        assert!(xml.contains("<AntiDDOSConfiguration>"));
        assert!(xml.contains("<Domain>abc1.example.cn</Domain>"));
        assert!(xml.contains("<Domain>abc2.example.cn</Domain>"));
    }

    #[test]
    fn body_without_domains_has_no_cnames() {
        let prepared = InitBucketAntiDDosInfo {
            instance_id: "id".to_string(),
            kind: AntiDdosType::AntiDdosPremimum,
            domains: vec![],
        }
        .prepare()
        .unwrap();
        let body = prepared.body.as_ref().unwrap();
        assert!(body.cnames.is_none());
        let xml = quick_xml::se::to_string(body).unwrap();
        assert!(!xml.contains("Cnames"));
    }
}

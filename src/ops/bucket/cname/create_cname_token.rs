//! CreateCnameToken.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createcnametoken>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct CreateCnameTokenParams {
    cname: OnlyKeyField,
    comp: String,
}

impl Default for CreateCnameTokenParams {
    fn default() -> Self {
        Self {
            cname: OnlyKeyField,
            comp: "token".to_string(),
        }
    }
}

/// Request body: `<BucketCnameConfiguration><Cname><Domain>...</Domain></Cname></BucketCnameConfiguration>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BucketCnameConfiguration", rename_all = "PascalCase")]
pub struct CreateCnameTokenBody {
    pub cname: CnameDomain,
}

/// `<Cname>` container that holds just the domain name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Cname", rename_all = "PascalCase")]
pub struct CnameDomain {
    pub domain: String,
}

/// Response body: `<CnameToken>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CnameToken", rename_all = "PascalCase")]
pub struct CnameToken {
    pub bucket: String,
    pub cname: String,
    pub token: String,
    pub expire_time: String,
}

pub struct CreateCnameToken {
    pub domain: String,
}

impl Ops for CreateCnameToken {
    type Response = BodyResponseProcessor<CnameToken>;
    type Body = XMLBody<CreateCnameTokenBody>;
    type Query = CreateCnameTokenParams;

    fn prepare(self) -> Result<Prepared<CreateCnameTokenParams, CreateCnameTokenBody>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(CreateCnameTokenParams::default()),
            body: Some(CreateCnameTokenBody {
                cname: CnameDomain { domain: self.domain },
            }),
            ..Default::default()
        })
    }
}

pub trait CreateCnameTokenOps {
    /// Create a CNAME token used to verify ownership of a custom domain.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createcnametoken>
    fn create_cname_token(&self, domain: impl Into<String>) -> impl Future<Output = Result<CnameToken>>;
}

impl CreateCnameTokenOps for Client {
    async fn create_cname_token(&self, domain: impl Into<String>) -> Result<CnameToken> {
        self.request(CreateCnameToken {
            domain: domain.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&CreateCnameTokenParams::default()).unwrap();
        assert_eq!(q, "cname&comp=token");
    }

    #[test]
    fn body_serializes() {
        let body = CreateCnameTokenBody {
            cname: CnameDomain {
                domain: "example.com".to_string(),
            },
        };
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert!(xml.contains("<BucketCnameConfiguration>"));
        assert!(xml.contains("<Cname>"));
        assert!(xml.contains("<Domain>example.com</Domain>"));
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CnameToken>
  <Bucket>examplebucket</Bucket>
  <Cname>example.com</Cname>
  <Token>be1d49d863dea9ffeff3df7d6455****</Token>
  <ExpireTime>Wed, 23 Feb 2022 21:16:37 GMT</ExpireTime>
</CnameToken>"#;
        let parsed: CnameToken = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.bucket, "examplebucket");
        assert_eq!(parsed.cname, "example.com");
        assert_eq!(parsed.token, "be1d49d863dea9ffeff3df7d6455****");
        assert_eq!(parsed.expire_time, "Wed, 23 Feb 2022 21:16:37 GMT");
    }
}

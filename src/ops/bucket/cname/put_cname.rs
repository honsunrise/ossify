//! PutCname.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putcname>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct PutCnameParams {
    cname: OnlyKeyField,
    comp: String,
}

impl Default for PutCnameParams {
    fn default() -> Self {
        Self {
            cname: OnlyKeyField,
            comp: "add".to_string(),
        }
    }
}

/// Optional TLS certificate configuration attached to a CNAME binding.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "CertificateConfiguration", rename_all = "PascalCase")]
pub struct CertificateConfiguration {
    #[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "PreviousCertId", skip_serializing_if = "Option::is_none")]
    pub previous_cert_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_certificate: Option<bool>,
}

/// `<Cname>` container used by PutCname: domain + optional certificate configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Cname", rename_all = "PascalCase")]
pub struct PutCnameEntry {
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_configuration: Option<CertificateConfiguration>,
}

/// Root `<BucketCnameConfiguration>` body used by PutCname.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BucketCnameConfiguration", rename_all = "PascalCase")]
pub struct PutCnameBody {
    pub cname: PutCnameEntry,
}

pub struct PutCname {
    pub domain: String,
    pub certificate: Option<CertificateConfiguration>,
}

impl Ops for PutCname {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PutCnameBody>;
    type Query = PutCnameParams;

    fn prepare(self) -> Result<Prepared<PutCnameParams, PutCnameBody>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(PutCnameParams::default()),
            body: Some(PutCnameBody {
                cname: PutCnameEntry {
                    domain: self.domain,
                    certificate_configuration: self.certificate,
                },
            }),
            ..Default::default()
        })
    }
}

pub trait PutCnameOps {
    /// Map a custom domain name to the bucket, optionally binding a TLS certificate.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putcname>
    fn put_cname(
        &self,
        domain: impl Into<String>,
        certificate: Option<CertificateConfiguration>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutCnameOps for Client {
    async fn put_cname(
        &self,
        domain: impl Into<String>,
        certificate: Option<CertificateConfiguration>,
    ) -> Result<()> {
        self.request(PutCname {
            domain: domain.into(),
            certificate,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&PutCnameParams::default()).unwrap();
        assert_eq!(q, "cname&comp=add");
    }

    #[test]
    fn body_without_certificate() {
        let body = PutCnameBody {
            cname: PutCnameEntry {
                domain: "example.com".to_string(),
                certificate_configuration: None,
            },
        };
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert!(xml.contains("<BucketCnameConfiguration>"));
        assert!(xml.contains("<Domain>example.com</Domain>"));
        assert!(!xml.contains("CertificateConfiguration"));
    }

    #[test]
    fn body_with_certificate() {
        let body = PutCnameBody {
            cname: PutCnameEntry {
                domain: "example.com".to_string(),
                certificate_configuration: Some(CertificateConfiguration {
                    cert_id: Some("493****-cn-hangzhou".to_string()),
                    force: Some(true),
                    ..Default::default()
                }),
            },
        };
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert!(xml.contains("<CertId>493****-cn-hangzhou</CertId>"));
        assert!(xml.contains("<Force>true</Force>"));
    }

    #[test]
    fn body_delete_certificate() {
        let body = PutCnameBody {
            cname: PutCnameEntry {
                domain: "example.com".to_string(),
                certificate_configuration: Some(CertificateConfiguration {
                    delete_certificate: Some(true),
                    ..Default::default()
                }),
            },
        };
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert!(xml.contains("<DeleteCertificate>true</DeleteCertificate>"));
    }
}

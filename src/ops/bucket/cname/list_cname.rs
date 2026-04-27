//! ListCname.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listcname>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCnameParams {
    cname: OnlyKeyField,
}

/// A mapped CNAME record returned by ListCname.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Cname", rename_all = "PascalCase")]
pub struct ListedCname {
    pub domain: String,
    pub last_modified: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<ListedCnameCertificate>,
}

/// Certificate descriptor attached to a mapped CNAME record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Certificate", rename_all = "PascalCase")]
pub struct ListedCnameCertificate {
    /// Certificate source. Valid values: `CAS`, `Upload`.
    #[serde(rename = "Type")]
    pub kind: String,
    #[serde(rename = "CertId")]
    pub cert_id: String,
    pub status: String,
    pub creation_date: String,
    pub fingerprint: String,
    pub valid_start_date: String,
    pub valid_end_date: String,
}

/// Response body: `<ListCnameResult>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ListCnameResult", rename_all = "PascalCase")]
pub struct ListCnameResult {
    pub bucket: String,
    pub owner: String,
    #[serde(rename = "Cname", default)]
    pub cnames: Vec<ListedCname>,
}

pub struct ListCname;

impl Ops for ListCname {
    type Response = BodyResponseProcessor<ListCnameResult>;
    type Body = NoneBody;
    type Query = ListCnameParams;

    fn prepare(self) -> Result<Prepared<ListCnameParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(ListCnameParams::default()),
            ..Default::default()
        })
    }
}

pub trait ListCnameOps {
    /// List every CNAME record mapped to the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listcname>
    fn list_cname(&self) -> impl Future<Output = Result<ListCnameResult>>;
}

impl ListCnameOps for Client {
    async fn list_cname(&self) -> Result<ListCnameResult> {
        self.request(ListCname).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&ListCnameParams::default()).unwrap();
        assert_eq!(q, "cname");
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListCnameResult>
  <Bucket>targetbucket</Bucket>
  <Owner>testowner</Owner>
  <Cname>
    <Domain>example.com</Domain>
    <LastModified>2021-09-15T02:35:07.000Z</LastModified>
    <Status>Enabled</Status>
    <Certificate>
      <Type>CAS</Type>
      <CertId>493****-cn-hangzhou</CertId>
      <Status>Enabled</Status>
      <CreationDate>Wed, 15 Sep 2021 02:35:06 GMT</CreationDate>
      <Fingerprint>DE:01:CF:EC:7C:A7:98:CB:D8:6E:FB:1D:97:EB:A9:64:1D:4E:**:**</Fingerprint>
      <ValidStartDate>Wed, 12 Apr 2023 10:14:51 GMT</ValidStartDate>
      <ValidEndDate>Mon, 4 May 2048 10:14:51 GMT</ValidEndDate>
    </Certificate>
  </Cname>
  <Cname>
    <Domain>example.org</Domain>
    <LastModified>2021-09-15T02:34:58.000Z</LastModified>
    <Status>Enabled</Status>
  </Cname>
</ListCnameResult>"#;
        let parsed: ListCnameResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.bucket, "targetbucket");
        assert_eq!(parsed.owner, "testowner");
        assert_eq!(parsed.cnames.len(), 2);
        assert_eq!(parsed.cnames[0].domain, "example.com");
        let cert = parsed.cnames[0].certificate.as_ref().unwrap();
        assert_eq!(cert.kind, "CAS");
        assert_eq!(cert.cert_id, "493****-cn-hangzhou");
        assert!(parsed.cnames[1].certificate.is_none());
    }
}

//! DeleteCname.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletecname>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::create_cname_token::{CnameDomain, CreateCnameTokenBody as DeleteCnameBody};
use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteCnameParams {
    cname: OnlyKeyField,
    comp: String,
}

impl Default for DeleteCnameParams {
    fn default() -> Self {
        Self {
            cname: OnlyKeyField,
            comp: "delete".to_string(),
        }
    }
}

pub struct DeleteCname {
    pub domain: String,
}

impl Ops for DeleteCname {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<DeleteCnameBody>;
    type Query = DeleteCnameParams;

    fn prepare(self) -> Result<Prepared<DeleteCnameParams, DeleteCnameBody>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(DeleteCnameParams::default()),
            body: Some(DeleteCnameBody {
                cname: CnameDomain { domain: self.domain },
            }),
            ..Default::default()
        })
    }
}

pub trait DeleteCnameOps {
    /// Delete the CNAME record mapped to this bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletecname>
    fn delete_cname(&self, domain: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteCnameOps for Client {
    async fn delete_cname(&self, domain: impl Into<String>) -> Result<()> {
        self.request(DeleteCname {
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
        let q = crate::ser::to_string(&DeleteCnameParams::default()).unwrap();
        assert_eq!(q, "cname&comp=delete");
    }

    #[test]
    fn body_serializes() {
        let body = DeleteCnameBody {
            cname: CnameDomain {
                domain: "example.com".to_string(),
            },
        };
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert!(xml.contains("<BucketCnameConfiguration>"));
        assert!(xml.contains("<Domain>example.com</Domain>"));
    }
}

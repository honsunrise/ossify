//! GetCnameToken.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getcnametoken>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::create_cname_token::CnameToken;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetCnameTokenParams {
    comp: String,
    pub cname: String,
}

impl GetCnameTokenParams {
    pub fn new(cname: impl Into<String>) -> Self {
        Self {
            comp: "token".to_string(),
            cname: cname.into(),
        }
    }
}

pub struct GetCnameToken {
    pub domain: String,
}

impl Ops for GetCnameToken {
    type Response = BodyResponseProcessor<CnameToken>;
    type Body = NoneBody;
    type Query = GetCnameTokenParams;

    fn prepare(self) -> Result<Prepared<GetCnameTokenParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetCnameTokenParams::new(self.domain)),
            ..Default::default()
        })
    }
}

pub trait GetCnameTokenOps {
    /// Retrieve the CNAME token for a custom domain.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getcnametoken>
    fn get_cname_token(&self, domain: impl Into<String>) -> impl Future<Output = Result<CnameToken>>;
}

impl GetCnameTokenOps for Client {
    async fn get_cname_token(&self, domain: impl Into<String>) -> Result<CnameToken> {
        self.request(GetCnameToken {
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
        let q = crate::ser::to_string(&GetCnameTokenParams::new("example.com")).unwrap();
        assert_eq!(q, "cname=example.com&comp=token");
    }

    #[test]
    fn method_is_get() {
        let p = GetCnameToken {
            domain: "example.com".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::GET);
    }
}

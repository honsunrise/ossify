//! DeleteAccessPoint.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspoint>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointParams {
    #[serde(rename = "accessPoint")]
    access_point: OnlyKeyField,
}

pub struct DeleteAccessPoint {
    pub name: String,
}

impl Ops for DeleteAccessPoint {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteAccessPointParams;

    fn prepare(self) -> Result<Prepared<DeleteAccessPointParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-access-point-name"), self.name.parse()?);
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteAccessPointParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait DeleteAccessPointOps {
    /// Delete an access point by name.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspoint>
    fn delete_access_point(&self, name: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteAccessPointOps for Client {
    async fn delete_access_point(&self, name: impl Into<String>) -> Result<()> {
        self.request(DeleteAccessPoint { name: name.into() }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&DeleteAccessPointParams::default()).unwrap(),
            "accessPoint"
        );
    }

    #[test]
    fn method_and_header() {
        let p = DeleteAccessPoint {
            name: "ap-01".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::DELETE);
        assert_eq!(
            p.headers
                .as_ref()
                .unwrap()
                .get("x-oss-access-point-name")
                .unwrap(),
            "ap-01"
        );
    }
}

//! DeleteAccessPointPolicy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointpolicy>

use std::future::Future;

use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointPolicyParams {
    #[serde(rename = "accessPointPolicy")]
    access_point_policy: OnlyKeyField,
}

pub struct DeleteAccessPointPolicy {
    pub name: String,
}

impl Ops for DeleteAccessPointPolicy {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteAccessPointPolicyParams;

    fn prepare(self) -> Result<Prepared<DeleteAccessPointPolicyParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-access-point-name"), self.name.parse()?);
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteAccessPointPolicyParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait DeleteAccessPointPolicyOps {
    /// Delete the policy associated with an access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointpolicy>
    fn delete_access_point_policy(&self, name: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteAccessPointPolicyOps for Client {
    async fn delete_access_point_policy(&self, name: impl Into<String>) -> Result<()> {
        self.request(DeleteAccessPointPolicy { name: name.into() }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&DeleteAccessPointPolicyParams::default()).unwrap(),
            "accessPointPolicy"
        );
    }

    #[test]
    fn method_and_header() {
        let p = DeleteAccessPointPolicy {
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

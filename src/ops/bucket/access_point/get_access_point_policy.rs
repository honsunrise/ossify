//! GetAccessPointPolicy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointpolicy>

use std::future::Future;

use bytes::Bytes;
use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BinaryResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointPolicyParams {
    #[serde(rename = "accessPointPolicy")]
    access_point_policy: OnlyKeyField,
}

pub struct GetAccessPointPolicy {
    pub name: String,
}

impl Ops for GetAccessPointPolicy {
    type Response = BinaryResponseProcessor;
    type Body = NoneBody;
    type Query = GetAccessPointPolicyParams;

    fn prepare(self) -> Result<Prepared<GetAccessPointPolicyParams>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-access-point-name"), self.name.parse()?);
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetAccessPointPolicyParams::default()),
            headers: Some(headers),
            ..Default::default()
        })
    }
}

pub trait GetAccessPointPolicyOps {
    /// Retrieve the raw JSON policy document attached to an access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointpolicy>
    fn get_access_point_policy(&self, name: impl Into<String>) -> impl Future<Output = Result<Bytes>>;
}

impl GetAccessPointPolicyOps for Client {
    async fn get_access_point_policy(&self, name: impl Into<String>) -> Result<Bytes> {
        self.request(GetAccessPointPolicy { name: name.into() }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetAccessPointPolicyParams::default()).unwrap(),
            "accessPointPolicy"
        );
    }

    #[test]
    fn prepared_sets_header() {
        let p = GetAccessPointPolicy {
            name: "ap-01".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::GET);
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

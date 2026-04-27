//! PutAccessPointPolicy.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointpolicy>

use std::future::Future;

use bytes::Bytes;
use http::{HeaderMap, HeaderName, Method};
use serde::Serialize;

use crate::body::BytesBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAccessPointPolicyParams {
    #[serde(rename = "accessPointPolicy")]
    access_point_policy: OnlyKeyField,
}

pub struct PutAccessPointPolicy {
    pub name: String,
    pub policy: Bytes,
}

impl Ops for PutAccessPointPolicy {
    type Response = EmptyResponseProcessor;
    type Body = BytesBody;
    type Query = PutAccessPointPolicyParams;

    fn prepare(self) -> Result<Prepared<PutAccessPointPolicyParams, (Bytes, &'static str)>> {
        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-oss-access-point-name"), self.name.parse()?);
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutAccessPointPolicyParams::default()),
            headers: Some(headers),
            body: Some((self.policy, "application/json")),
            ..Default::default()
        })
    }
}

pub trait PutAccessPointPolicyOps {
    /// Attach a JSON IAM-like policy document to an access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putaccesspointpolicy>
    fn put_access_point_policy(
        &self,
        name: impl Into<String>,
        policy: impl Into<Bytes>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutAccessPointPolicyOps for Client {
    async fn put_access_point_policy(&self, name: impl Into<String>, policy: impl Into<Bytes>) -> Result<()> {
        self.request(PutAccessPointPolicy {
            name: name.into(),
            policy: policy.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutAccessPointPolicyParams::default()).unwrap(),
            "accessPointPolicy"
        );
    }

    #[test]
    fn prepared_sets_header_and_json_body() {
        let prepared = PutAccessPointPolicy {
            name: "ap-01".to_string(),
            policy: Bytes::from_static(br#"{"Version":"1"}"#),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
        assert_eq!(
            prepared
                .headers
                .as_ref()
                .unwrap()
                .get("x-oss-access-point-name")
                .unwrap(),
            "ap-01"
        );
        let (_, ct) = prepared.body.unwrap();
        assert_eq!(ct, "application/json");
    }
}

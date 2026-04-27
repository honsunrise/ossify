//! DeleteAccessPointPublicAccessBlock.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointpublicaccessblock>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteAccessPointPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
    #[serde(rename = "x-oss-access-point-name")]
    pub access_point_name: String,
}

impl DeleteAccessPointPublicAccessBlockParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            public_access_block: OnlyKeyField,
            access_point_name: name.into(),
        }
    }
}

pub struct DeleteAccessPointPublicAccessBlock {
    pub name: String,
}

impl Ops for DeleteAccessPointPublicAccessBlock {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteAccessPointPublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<DeleteAccessPointPublicAccessBlockParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteAccessPointPublicAccessBlockParams::new(self.name)),
            ..Default::default()
        })
    }
}

pub trait DeleteAccessPointPublicAccessBlockOps {
    /// Delete the Block Public Access configuration of an access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteaccesspointpublicaccessblock>
    fn delete_access_point_public_access_block(
        &self,
        name: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl DeleteAccessPointPublicAccessBlockOps for Client {
    async fn delete_access_point_public_access_block(&self, name: impl Into<String>) -> Result<()> {
        self.request(DeleteAccessPointPublicAccessBlock { name: name.into() })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&DeleteAccessPointPublicAccessBlockParams::new("ap-01")).unwrap();
        assert_eq!(q, "publicAccessBlock&x-oss-access-point-name=ap-01");
    }
}

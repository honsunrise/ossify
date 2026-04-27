//! GetAccessPointPublicAccessBlock.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointpublicaccessblock>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::PublicAccessBlockConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetAccessPointPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
    #[serde(rename = "x-oss-access-point-name")]
    pub access_point_name: String,
}

impl GetAccessPointPublicAccessBlockParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            public_access_block: OnlyKeyField,
            access_point_name: name.into(),
        }
    }
}

pub struct GetAccessPointPublicAccessBlock {
    pub name: String,
}

impl Ops for GetAccessPointPublicAccessBlock {
    type Response = BodyResponseProcessor<PublicAccessBlockConfiguration>;
    type Body = NoneBody;
    type Query = GetAccessPointPublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<GetAccessPointPublicAccessBlockParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetAccessPointPublicAccessBlockParams::new(self.name)),
            ..Default::default()
        })
    }
}

pub trait GetAccessPointPublicAccessBlockOps {
    /// Retrieve the Block Public Access configuration of an access point.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getaccesspointpublicaccessblock>
    fn get_access_point_public_access_block(
        &self,
        name: impl Into<String>,
    ) -> impl Future<Output = Result<PublicAccessBlockConfiguration>>;
}

impl GetAccessPointPublicAccessBlockOps for Client {
    async fn get_access_point_public_access_block(
        &self,
        name: impl Into<String>,
    ) -> Result<PublicAccessBlockConfiguration> {
        self.request(GetAccessPointPublicAccessBlock { name: name.into() })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetAccessPointPublicAccessBlockParams::new("ap-01")).unwrap();
        assert_eq!(q, "publicAccessBlock&x-oss-access-point-name=ap-01");
    }
}

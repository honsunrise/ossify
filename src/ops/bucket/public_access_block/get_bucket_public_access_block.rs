//! GetBucketPublicAccessBlock.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketpublicaccessblock>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::PublicAccessBlockConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
}

pub struct GetBucketPublicAccessBlock;

impl Ops for GetBucketPublicAccessBlock {
    type Response = BodyResponseProcessor<PublicAccessBlockConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketPublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<GetBucketPublicAccessBlockParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketPublicAccessBlockParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketPublicAccessBlockOps {
    /// Retrieve the bucket-level Block Public Access configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketpublicaccessblock>
    fn get_bucket_public_access_block(&self) -> impl Future<Output = Result<PublicAccessBlockConfiguration>>;
}

impl GetBucketPublicAccessBlockOps for Client {
    async fn get_bucket_public_access_block(&self) -> Result<PublicAccessBlockConfiguration> {
        self.request(GetBucketPublicAccessBlock).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketPublicAccessBlockParams::default()).unwrap(),
            "publicAccessBlock"
        );
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<PublicAccessBlockConfiguration><BlockPublicAccess>true</BlockPublicAccess></PublicAccessBlockConfiguration>"#;
        let parsed: PublicAccessBlockConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.block_public_access);
    }
}

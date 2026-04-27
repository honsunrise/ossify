//! GetPublicAccessBlock — query the global Block Public Access configuration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getpublicaccessblock>

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
pub struct GetPublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
}

pub struct GetPublicAccessBlock;

impl Ops for GetPublicAccessBlock {
    const USE_BUCKET: bool = false;

    type Response = BodyResponseProcessor<PublicAccessBlockConfiguration>;
    type Body = NoneBody;
    type Query = GetPublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<GetPublicAccessBlockParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetPublicAccessBlockParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetPublicAccessBlockOps {
    /// Retrieve the global Block Public Access configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getpublicaccessblock>
    fn get_public_access_block(&self) -> impl Future<Output = Result<PublicAccessBlockConfiguration>>;
}

impl GetPublicAccessBlockOps for Client {
    async fn get_public_access_block(&self) -> Result<PublicAccessBlockConfiguration> {
        self.request(GetPublicAccessBlock).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetPublicAccessBlockParams::default()).unwrap(),
            "publicAccessBlock"
        );
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<GetPublicAccessBlock as Ops>::USE_BUCKET);
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<PublicAccessBlockConfiguration>
  <BlockPublicAccess>true</BlockPublicAccess>
</PublicAccessBlockConfiguration>"#;
        let parsed: PublicAccessBlockConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.block_public_access);
    }
}

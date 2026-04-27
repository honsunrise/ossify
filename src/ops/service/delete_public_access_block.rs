//! DeletePublicAccessBlock — delete the global Block Public Access configuration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletepublicaccessblock>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePublicAccessBlockParams {
    #[serde(rename = "publicAccessBlock")]
    public_access_block: OnlyKeyField,
}

pub struct DeletePublicAccessBlock;

impl Ops for DeletePublicAccessBlock {
    const USE_BUCKET: bool = false;

    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeletePublicAccessBlockParams;

    fn prepare(self) -> Result<Prepared<DeletePublicAccessBlockParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeletePublicAccessBlockParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeletePublicAccessBlockOps {
    /// Delete the global Block Public Access configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletepublicaccessblock>
    fn delete_public_access_block(&self) -> impl Future<Output = Result<()>>;
}

impl DeletePublicAccessBlockOps for Client {
    async fn delete_public_access_block(&self) -> Result<()> {
        self.request(DeletePublicAccessBlock).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&DeletePublicAccessBlockParams::default()).unwrap(),
            "publicAccessBlock"
        );
    }

    #[test]
    fn use_bucket_is_false() {
        const _: () = assert!(!<DeletePublicAccessBlock as Ops>::USE_BUCKET);
    }
}

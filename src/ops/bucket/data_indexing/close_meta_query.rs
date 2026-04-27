//! CloseMetaQuery.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/closemetaquery>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct CloseMetaQueryParams {
    #[serde(rename = "metaQuery")]
    meta_query: OnlyKeyField,
    comp: String,
}

impl Default for CloseMetaQueryParams {
    fn default() -> Self {
        Self {
            meta_query: OnlyKeyField,
            comp: "delete".to_string(),
        }
    }
}

pub struct CloseMetaQuery;

impl Ops for CloseMetaQuery {
    type Response = EmptyResponseProcessor;
    type Body = ZeroBody;
    type Query = CloseMetaQueryParams;

    fn prepare(self) -> Result<Prepared<CloseMetaQueryParams>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(CloseMetaQueryParams::default()),
            body: Some(()),
            ..Default::default()
        })
    }
}

pub trait CloseMetaQueryOps {
    /// Disable the metadata-index library (asynchronously).
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/closemetaquery>
    fn close_meta_query(&self) -> impl Future<Output = Result<()>>;
}

impl CloseMetaQueryOps for Client {
    async fn close_meta_query(&self) -> Result<()> {
        self.request(CloseMetaQuery).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&CloseMetaQueryParams::default()).unwrap();
        assert_eq!(q, "comp=delete&metaQuery");
    }
}

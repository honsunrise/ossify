//! DeleteBucketTags.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebuckettags>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Query for deleting all tags (`?tagging`).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketTagsParams {
    tagging: OnlyKeyField,
}

/// Query for deleting a specific set of tag keys (`?tagging=k1,k2`).
#[derive(Debug, Clone, Serialize)]
pub struct DeleteBucketTagsKeysParams {
    pub tagging: String,
}

/// The `DeleteBucketTags` operation.
///
/// With `keys = None`, all tags are deleted. With `keys = Some(...)`, only
/// the listed tag keys are removed.
pub struct DeleteBucketTags {
    pub keys: Option<Vec<String>>,
}

impl Ops for DeleteBucketTags {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    // We pick a query struct that emits either the bare `?tagging` form or
    // the `?tagging=k1,k2` form depending on `keys`.
    type Query = DeleteBucketTagsQuery;

    fn prepare(self) -> Result<Prepared<DeleteBucketTagsQuery>> {
        let query = match self.keys {
            None => DeleteBucketTagsQuery::All(DeleteBucketTagsParams::default()),
            Some(keys) => DeleteBucketTagsQuery::Keys(DeleteBucketTagsKeysParams {
                tagging: keys.join(","),
            }),
        };
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(query),
            ..Default::default()
        })
    }
}

/// Tagged union of the two possible query strings.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DeleteBucketTagsQuery {
    All(DeleteBucketTagsParams),
    Keys(DeleteBucketTagsKeysParams),
}

pub trait DeleteBucketTagsOps {
    /// Delete the tags configured on the bucket.
    ///
    /// Pass `None` to delete every tag, or `Some(keys)` to delete only the
    /// listed keys.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletebuckettags>
    fn delete_bucket_tags(&self, keys: Option<Vec<String>>) -> impl Future<Output = Result<()>>;
}

impl DeleteBucketTagsOps for Client {
    async fn delete_bucket_tags(&self, keys: Option<Vec<String>>) -> Result<()> {
        self.request(DeleteBucketTags { keys }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_all_serializes_bare_tagging() {
        let q = crate::ser::to_string(&DeleteBucketTagsParams::default()).unwrap();
        assert_eq!(q, "tagging");
    }

    #[test]
    fn params_keys_serialize() {
        let q = crate::ser::to_string(&DeleteBucketTagsKeysParams {
            tagging: "k1,k2".to_string(),
        })
        .unwrap();
        assert_eq!(q, "tagging=k1%2Ck2");
    }

    #[test]
    fn prepared_all() {
        let prepared = DeleteBucketTags { keys: None }.prepare().unwrap();
        assert_eq!(prepared.method, Method::DELETE);
        let q = crate::ser::to_string(prepared.query.as_ref().unwrap()).unwrap();
        assert_eq!(q, "tagging");
    }

    #[test]
    fn prepared_keys() {
        let prepared = DeleteBucketTags {
            keys: Some(vec!["a".to_string(), "b".to_string()]),
        }
        .prepare()
        .unwrap();
        let q = crate::ser::to_string(prepared.query.as_ref().unwrap()).unwrap();
        assert_eq!(q, "tagging=a%2Cb");
    }
}

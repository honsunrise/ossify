//! CleanRestoredObject operation.
//!
//! Ends the restored state of a Cold Archive / Deep Cold Archive object early,
//! stopping further temporary-replica storage billing.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/cleanrestoredobject>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// CleanRestoredObject query parameters (`?cleanRestoredObject`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanRestoredObjectParams {
    pub(crate) clean_restored_object: OnlyKeyField,
}

impl CleanRestoredObjectParams {
    pub fn new() -> Self {
        Self::default()
    }
}

/// CleanRestoredObject operation.
pub struct CleanRestoredObject {
    pub object_key: String,
    pub params: CleanRestoredObjectParams,
}

impl Ops for CleanRestoredObject {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = CleanRestoredObjectParams;

    fn prepare(self) -> Result<Prepared<CleanRestoredObjectParams>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for CleanRestoredObject operations.
pub trait CleanRestoredObjectOperations {
    fn clean_restored_object(&self, object_key: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl CleanRestoredObjectOperations for Client {
    async fn clean_restored_object(&self, object_key: impl Into<String>) -> Result<()> {
        let ops = CleanRestoredObject {
            object_key: object_key.into(),
            params: CleanRestoredObjectParams::new(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&CleanRestoredObjectParams::new()).unwrap();
        assert_eq!(q, "cleanRestoredObject");
    }

    #[test]
    fn test_prepare_key_method() {
        let prepared = CleanRestoredObject {
            object_key: "foo.jpg".into(),
            params: CleanRestoredObjectParams::new(),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::POST);
        assert_eq!(prepared.key.as_deref(), Some("foo.jpg"));
    }
}

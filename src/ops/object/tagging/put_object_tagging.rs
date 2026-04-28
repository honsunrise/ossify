//! PutObjectTagging: add or update the tag set on an object.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putobjecttagging>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::Tagging;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// PutObjectTagging query parameters: `?tagging[&versionId=<id>]`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutObjectTaggingParams {
    pub(crate) tagging: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PutObjectTaggingParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, v: impl Into<String>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

/// PutObjectTagging operation.
pub struct PutObjectTagging {
    pub object_key: String,
    pub params: PutObjectTaggingParams,
    pub body: Tagging,
}

impl Ops for PutObjectTagging {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<Tagging>;
    type Query = PutObjectTaggingParams;

    fn prepare(self) -> Result<Prepared<PutObjectTaggingParams, Tagging>> {
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.object_key),
            query: Some(self.params),
            body: Some(self.body),
            ..Default::default()
        })
    }
}

/// Trait for PutObjectTagging operations.
pub trait PutObjectTaggingOperations {
    fn put_object_tagging(
        &self,
        object_key: impl Into<String>,
        tagging: Tagging,
        params: Option<PutObjectTaggingParams>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutObjectTaggingOperations for Client {
    async fn put_object_tagging(
        &self,
        object_key: impl Into<String>,
        tagging: Tagging,
        params: Option<PutObjectTaggingParams>,
    ) -> Result<()> {
        let ops = PutObjectTagging {
            object_key: object_key.into(),
            params: params.unwrap_or_default(),
            body: tagging,
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::{Tag, TagSet, Tagging};

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&PutObjectTaggingParams::new().version_id("v1")).unwrap();
        assert_eq!(q, "tagging&versionId=v1");
    }

    #[test]
    fn test_serialize_body() {
        let tagging = Tagging {
            tag_set: TagSet {
                tags: vec![
                    Tag {
                        key: "a".into(),
                        value: "1".into(),
                    },
                    Tag {
                        key: "b".into(),
                        value: "2".into(),
                    },
                ],
            },
        };
        let xml = quick_xml::se::to_string(&tagging).unwrap();
        assert!(xml.contains("<Tag><Key>a</Key><Value>1</Value></Tag>"));
        assert!(xml.contains("<Tag><Key>b</Key><Value>2</Value></Tag>"));
    }
}

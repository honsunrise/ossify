//! GetObjectTagging: query the tag set on an object.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getobjecttagging>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::Tagging;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// GetObjectTagging query parameters.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectTaggingParams {
    pub(crate) tagging: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetObjectTaggingParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version_id(mut self, v: impl Into<String>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

/// GetObjectTagging operation.
pub struct GetObjectTagging {
    pub object_key: String,
    pub params: GetObjectTaggingParams,
}

impl Ops for GetObjectTagging {
    type Response = BodyResponseProcessor<Tagging>;
    type Body = NoneBody;
    type Query = GetObjectTaggingParams;

    fn prepare(self) -> Result<Prepared<GetObjectTaggingParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.object_key),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for GetObjectTagging operations.
pub trait GetObjectTaggingOperations {
    fn get_object_tagging(
        &self,
        object_key: impl Into<String>,
        params: Option<GetObjectTaggingParams>,
    ) -> impl Future<Output = Result<Tagging>>;
}

impl GetObjectTaggingOperations for Client {
    async fn get_object_tagging(
        &self,
        object_key: impl Into<String>,
        params: Option<GetObjectTaggingParams>,
    ) -> Result<Tagging> {
        let ops = GetObjectTagging {
            object_key: object_key.into(),
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params_default() {
        let q = crate::ser::to_string(&GetObjectTaggingParams::default()).unwrap();
        assert_eq!(q, "tagging");
    }

    #[test]
    fn test_deserialize_response() {
        let xml = r#"<Tagging>
  <TagSet>
    <Tag><Key>a</Key><Value>1</Value></Tag>
    <Tag><Key>b</Key><Value>2</Value></Tag>
  </TagSet>
</Tagging>"#;
        let parsed: Tagging = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.tag_set.tags.len(), 2);
        assert_eq!(parsed.tag_set.tags[0].key, "a");
    }
}

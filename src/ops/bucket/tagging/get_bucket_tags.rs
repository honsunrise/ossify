//! GetBucketTags.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbuckettags>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::Tagging;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketTagsParams {
    tagging: OnlyKeyField,
}

pub struct GetBucketTags;

impl Ops for GetBucketTags {
    type Response = BodyResponseProcessor<Tagging>;
    type Body = NoneBody;
    type Query = GetBucketTagsParams;

    fn prepare(self) -> Result<Prepared<GetBucketTagsParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketTagsParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketTagsOps {
    /// Query the tags configured on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbuckettags>
    fn get_bucket_tags(&self) -> impl Future<Output = Result<Tagging>>;
}

impl GetBucketTagsOps for Client {
    async fn get_bucket_tags(&self) -> Result<Tagging> {
        self.request(GetBucketTags).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetBucketTagsParams::default()).unwrap(), "tagging");
    }

    #[test]
    fn parse_tagging_response() {
        let xml = r#"<Tagging>
  <TagSet>
    <Tag><Key>testa</Key><Value>value1</Value></Tag>
    <Tag><Key>testb</Key><Value>value2</Value></Tag>
  </TagSet>
</Tagging>"#;
        let parsed: Tagging = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.tag_set.tags.len(), 2);
        assert_eq!(parsed.tag_set.tags[1].key, "testb");
    }
}

//! PutBucketTags.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbuckettags>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::{Tag, Tagging};
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketTagsParams {
    tagging: OnlyKeyField,
}

pub struct PutBucketTags {
    pub tagging: Tagging,
}

impl Ops for PutBucketTags {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<Tagging>;
    type Query = PutBucketTagsParams;

    fn prepare(self) -> Result<Prepared<PutBucketTagsParams, Tagging>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketTagsParams::default()),
            body: Some(self.tagging),
            ..Default::default()
        })
    }
}

pub trait PutBucketTagsOps {
    /// Add or modify tags on a bucket. Existing tags with matching keys are
    /// overwritten.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbuckettags>
    fn put_bucket_tags(&self, tags: Vec<Tag>) -> impl Future<Output = Result<()>>;
}

impl PutBucketTagsOps for Client {
    async fn put_bucket_tags(&self, tags: Vec<Tag>) -> Result<()> {
        self.request(PutBucketTags {
            tagging: Tagging::new(tags),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketTagsParams::default()).unwrap(), "tagging");
    }

    #[test]
    fn prepared_body_contains_tags() {
        let prepared = PutBucketTags {
            tagging: Tagging::new(vec![Tag::new("k", "v")]),
        }
        .prepare()
        .unwrap();
        assert_eq!(prepared.method, Method::PUT);
        let xml = quick_xml::se::to_string(prepared.body.as_ref().unwrap()).unwrap();
        assert!(xml.contains("<Key>k</Key>"));
    }
}

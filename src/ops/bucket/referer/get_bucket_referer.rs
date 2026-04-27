//! GetBucketReferer.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreferer>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_referer::RefererConfiguration;
#[allow(unused_imports)]
pub use super::put_bucket_referer::RefererList;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketRefererParams {
    referer: OnlyKeyField,
}

pub struct GetBucketReferer;

impl Ops for GetBucketReferer {
    type Response = BodyResponseProcessor<RefererConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketRefererParams;

    fn prepare(self) -> Result<Prepared<GetBucketRefererParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketRefererParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketRefererOps {
    /// Retrieve the Referer configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketreferer>
    fn get_bucket_referer(&self) -> impl Future<Output = Result<RefererConfiguration>>;
}

impl GetBucketRefererOps for Client {
    async fn get_bucket_referer(&self) -> Result<RefererConfiguration> {
        self.request(GetBucketReferer).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&GetBucketRefererParams::default()).unwrap(), "referer");
    }

    #[test]
    fn parse_empty_list() {
        let xml = r#"<RefererConfiguration>
  <AllowEmptyReferer>true</AllowEmptyReferer>
  <RefererList/>
</RefererConfiguration>"#;
        let parsed: RefererConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.allow_empty_referer);
        assert_eq!(
            parsed
                .referer_list
                .as_ref()
                .map(|r| r.referers.len())
                .unwrap_or(0),
            0
        );
    }

    #[test]
    fn parse_with_both_lists() {
        let xml = r#"<RefererConfiguration>
  <AllowEmptyReferer>false</AllowEmptyReferer>
  <AllowTruncateQueryString>true</AllowTruncateQueryString>
  <TruncatePath>true</TruncatePath>
  <RefererList>
    <Referer>http://a.com</Referer>
    <Referer>http://b.com</Referer>
  </RefererList>
  <RefererBlacklist>
    <Referer>http://c.com</Referer>
  </RefererBlacklist>
</RefererConfiguration>"#;
        let parsed: RefererConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert!(!parsed.allow_empty_referer);
        assert_eq!(parsed.allow_truncate_query_string, Some(true));
        assert_eq!(parsed.referer_list.unwrap().referers.len(), 2);
        assert_eq!(parsed.referer_blacklist.unwrap().referers, vec!["http://c.com".to_string()]);
    }
}

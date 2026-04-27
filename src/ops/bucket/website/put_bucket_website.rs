//! PutBucketWebsite.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketwebsite>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketWebsiteParams {
    website: OnlyKeyField,
}

/// `<IndexDocument>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexDocument {
    pub suffix: String,
    pub support_sub_dir: Option<bool>,
    /// 0 / 1 / 2 per the PutBucketWebsite reference.
    #[serde(rename = "Type")]
    pub type_: Option<u32>,
}

/// `<ErrorDocument>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorDocument {
    pub key: String,
    pub http_status: Option<u32>,
}

/// `<Condition>` container inside a `<RoutingRule>`.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRuleCondition {
    pub key_prefix_equals: Option<String>,
    pub http_error_code_returned_equals: Option<u32>,
    pub key_suffix_equals: Option<String>,
    #[serde(rename = "IncludeHeader", default, skip_serializing_if = "Vec::is_empty")]
    pub include_headers: Vec<IncludeHeader>,
}

/// `<IncludeHeader>` entry inside `<Condition>`.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncludeHeader {
    pub key: String,
    pub equals: Option<String>,
}

/// `<Set>` entry inside `<MirrorHeaders>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MirrorSetHeader {
    pub key: String,
    pub value: String,
}

/// `<MirrorHeaders>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MirrorHeaders {
    pub pass_all: Option<bool>,
    #[serde(rename = "Pass", default, skip_serializing_if = "Vec::is_empty")]
    pub pass: Vec<String>,
    #[serde(rename = "Remove", default, skip_serializing_if = "Vec::is_empty")]
    pub remove: Vec<String>,
    #[serde(rename = "Set", default, skip_serializing_if = "Vec::is_empty")]
    pub set: Vec<MirrorSetHeader>,
}

/// `<Redirect>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRuleRedirect {
    pub redirect_type: Option<String>,
    pub pass_query_string: Option<bool>,
    #[serde(rename = "MirrorURL")]
    pub mirror_url: Option<String>,
    pub mirror_pass_query_string: Option<bool>,
    pub mirror_follow_redirect: Option<bool>,
    pub mirror_check_md5: Option<bool>,
    pub mirror_headers: Option<MirrorHeaders>,
    pub protocol: Option<String>,
    pub host_name: Option<String>,
    pub replace_key_prefix_with: Option<String>,
    pub enable_replace_prefix: Option<bool>,
    pub replace_key_with: Option<String>,
    pub http_redirect_code: Option<u32>,
}

/// `<RoutingRule>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRule {
    pub rule_number: u32,
    pub condition: RoutingRuleCondition,
    pub redirect: RoutingRuleRedirect,
}

/// `<RoutingRules>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoutingRules {
    #[serde(rename = "RoutingRule", default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RoutingRule>,
}

/// Root `<WebsiteConfiguration>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "WebsiteConfiguration", rename_all = "PascalCase")]
pub struct WebsiteConfiguration {
    pub index_document: Option<IndexDocument>,
    pub error_document: Option<ErrorDocument>,
    pub routing_rules: Option<RoutingRules>,
}

pub struct PutBucketWebsite {
    pub config: WebsiteConfiguration,
}

impl Ops for PutBucketWebsite {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<WebsiteConfiguration>;
    type Query = PutBucketWebsiteParams;

    fn prepare(self) -> Result<Prepared<PutBucketWebsiteParams, WebsiteConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketWebsiteParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutBucketWebsiteOps {
    /// Enable static website hosting on the bucket, optionally with redirect /
    /// mirroring-based back-to-origin rules.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketwebsite>
    fn put_bucket_website(&self, config: WebsiteConfiguration) -> impl Future<Output = Result<()>>;
}

impl PutBucketWebsiteOps for Client {
    async fn put_bucket_website(&self, config: WebsiteConfiguration) -> Result<()> {
        self.request(PutBucketWebsite { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&PutBucketWebsiteParams::default()).unwrap(), "website");
    }

    #[test]
    fn parse_minimal_config() {
        let xml = r#"<WebsiteConfiguration>
  <IndexDocument>
    <Suffix>index.html</Suffix>
  </IndexDocument>
  <ErrorDocument>
    <Key>error.html</Key>
    <HttpStatus>404</HttpStatus>
  </ErrorDocument>
</WebsiteConfiguration>"#;
        let parsed: WebsiteConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.index_document.as_ref().unwrap().suffix, "index.html");
        assert_eq!(parsed.error_document.as_ref().unwrap().http_status, Some(404));
    }

    #[test]
    fn parse_routing_rules() {
        let xml = r#"<WebsiteConfiguration>
  <IndexDocument><Suffix>index.html</Suffix></IndexDocument>
  <RoutingRules>
    <RoutingRule>
      <RuleNumber>1</RuleNumber>
      <Condition>
        <KeyPrefixEquals>abc/</KeyPrefixEquals>
        <HttpErrorCodeReturnedEquals>404</HttpErrorCodeReturnedEquals>
      </Condition>
      <Redirect>
        <RedirectType>Mirror</RedirectType>
        <MirrorURL>http://example.com/</MirrorURL>
        <MirrorHeaders>
          <PassAll>true</PassAll>
          <Pass>h1</Pass>
          <Set><Key>k</Key><Value>v</Value></Set>
        </MirrorHeaders>
      </Redirect>
    </RoutingRule>
  </RoutingRules>
</WebsiteConfiguration>"#;
        let parsed: WebsiteConfiguration = quick_xml::de::from_str(xml).unwrap();
        let rules = parsed.routing_rules.unwrap().rules;
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].rule_number, 1);
        let mh = rules[0].redirect.mirror_headers.as_ref().unwrap();
        assert_eq!(mh.pass_all, Some(true));
        assert_eq!(mh.pass, vec!["h1".to_string()]);
        assert_eq!(mh.set[0].key, "k");
    }
}

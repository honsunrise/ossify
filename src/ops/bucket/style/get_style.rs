//! GetStyle.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getstyle>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetStyleParams {
    style: OnlyKeyField,
    #[serde(rename = "styleName")]
    pub style_name: String,
}

impl GetStyleParams {
    pub fn new(style_name: impl Into<String>) -> Self {
        Self {
            style: OnlyKeyField,
            style_name: style_name.into(),
        }
    }
}

/// Response body: `<Style>` with all metadata fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Style", rename_all = "PascalCase")]
pub struct StyleInfo {
    pub name: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modify_time: Option<String>,
}

pub struct GetStyle {
    pub name: String,
}

impl Ops for GetStyle {
    type Response = BodyResponseProcessor<StyleInfo>;
    type Body = NoneBody;
    type Query = GetStyleParams;

    fn prepare(self) -> Result<Prepared<GetStyleParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetStyleParams::new(self.name)),
            ..Default::default()
        })
    }
}

pub trait GetStyleOps {
    /// Retrieve one image style by name.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getstyle>
    fn get_style(&self, name: impl Into<String>) -> impl Future<Output = Result<StyleInfo>>;
}

impl GetStyleOps for Client {
    async fn get_style(&self, name: impl Into<String>) -> Result<StyleInfo> {
        self.request(GetStyle { name: name.into() }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&GetStyleParams::new("imagestyle")).unwrap();
        assert_eq!(q, "style&styleName=imagestyle");
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Style>
 <Name>imagestyle</Name>
 <Content>image/resize,p_50</Content>
 <Category>image</Category>
 <CreateTime>Wed, 20 May 2020 12:07:15 GMT</CreateTime>
 <LastModifyTime>Wed, 21 May 2020 12:07:15 GMT</LastModifyTime>
</Style>"#;
        let parsed: StyleInfo = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.name, "imagestyle");
        assert_eq!(parsed.content, "image/resize,p_50");
        assert_eq!(parsed.category.as_deref(), Some("image"));
        assert_eq!(parsed.create_time.as_deref(), Some("Wed, 20 May 2020 12:07:15 GMT"));
    }
}

//! ListStyle.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/liststyle>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

pub use super::get_style::StyleInfo;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListStyleParams {
    style: OnlyKeyField,
}

/// Response body: `<StyleList>` wrapping zero or more `<Style>` entries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "StyleList", rename_all = "PascalCase")]
pub struct StyleList {
    #[serde(rename = "Style", default)]
    pub styles: Vec<StyleInfo>,
}

pub struct ListStyle;

impl Ops for ListStyle {
    type Response = BodyResponseProcessor<StyleList>;
    type Body = NoneBody;
    type Query = ListStyleParams;

    fn prepare(self) -> Result<Prepared<ListStyleParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(ListStyleParams::default()),
            ..Default::default()
        })
    }
}

pub trait ListStyleOps {
    /// List every image style configured for the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/liststyle>
    fn list_style(&self) -> impl Future<Output = Result<StyleList>>;
}

impl ListStyleOps for Client {
    async fn list_style(&self) -> Result<StyleList> {
        self.request(ListStyle).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(crate::ser::to_string(&ListStyleParams::default()).unwrap(), "style");
    }

    #[test]
    fn response_round_trip() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<StyleList>
 <Style>
 <Name>imagestyle</Name>
 <Content>image/resize,p_50</Content>
 <Category>image</Category>
 <CreateTime>Wed, 20 May 2020 12:07:15 GMT</CreateTime>
 <LastModifyTime>Wed, 21 May 2020 12:07:15 GMT</LastModifyTime>
 </Style>
 <Style>
 <Name>imagestyle1</Name>
 <Content>image/resize,w_200</Content>
 <Category>image</Category>
 <CreateTime>Wed, 20 May 2020 12:08:04 GMT</CreateTime>
 <LastModifyTime>Wed, 21 May 2020 12:08:04 GMT</LastModifyTime>
 </Style>
</StyleList>"#;
        let parsed: StyleList = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.styles.len(), 2);
        assert_eq!(parsed.styles[0].name, "imagestyle");
        assert_eq!(parsed.styles[1].content, "image/resize,w_200");
    }
}

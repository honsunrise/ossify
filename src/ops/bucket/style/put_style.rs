//! PutStyle.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putstyle>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct PutStyleParams {
    style: OnlyKeyField,
    #[serde(rename = "styleName")]
    pub style_name: String,
}

impl PutStyleParams {
    pub fn new(style_name: impl Into<String>) -> Self {
        Self {
            style: OnlyKeyField,
            style_name: style_name.into(),
        }
    }
}

/// Request body: `<Style><Content>...</Content></Style>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Style", rename_all = "PascalCase")]
pub struct PutStyleBody {
    pub content: String,
}

impl PutStyleBody {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

pub struct PutStyle {
    pub name: String,
    pub content: String,
}

impl Ops for PutStyle {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<PutStyleBody>;
    type Query = PutStyleParams;

    fn prepare(self) -> Result<Prepared<PutStyleParams, PutStyleBody>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutStyleParams::new(self.name)),
            body: Some(PutStyleBody::new(self.content)),
            ..Default::default()
        })
    }
}

pub trait PutStyleOps {
    /// Create an image style for this bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putstyle>
    fn put_style(
        &self,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> impl Future<Output = Result<()>>;
}

impl PutStyleOps for Client {
    async fn put_style(&self, name: impl Into<String>, content: impl Into<String>) -> Result<()> {
        self.request(PutStyle {
            name: name.into(),
            content: content.into(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&PutStyleParams::new("imagestyle")).unwrap();
        assert_eq!(q, "style&styleName=imagestyle");
    }

    #[test]
    fn body_serializes() {
        let xml = quick_xml::se::to_string(&PutStyleBody::new("image/resize,p_50")).unwrap();
        assert!(xml.contains("<Style>"));
        assert!(xml.contains("<Content>image/resize,p_50</Content>"));
    }
}

//! DeleteStyle.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletestyle>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteStyleParams {
    style: OnlyKeyField,
    #[serde(rename = "styleName")]
    pub style_name: String,
}

impl DeleteStyleParams {
    pub fn new(style_name: impl Into<String>) -> Self {
        Self {
            style: OnlyKeyField,
            style_name: style_name.into(),
        }
    }
}

pub struct DeleteStyle {
    pub name: String,
}

impl Ops for DeleteStyle {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteStyleParams;

    fn prepare(self) -> Result<Prepared<DeleteStyleParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteStyleParams::new(self.name)),
            ..Default::default()
        })
    }
}

pub trait DeleteStyleOps {
    /// Delete a specific image style from this bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletestyle>
    fn delete_style(&self, name: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteStyleOps for Client {
    async fn delete_style(&self, name: impl Into<String>) -> Result<()> {
        self.request(DeleteStyle { name: name.into() }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&DeleteStyleParams::new("imagestyle")).unwrap();
        assert_eq!(q, "style&styleName=imagestyle");
    }

    #[test]
    fn method_is_delete() {
        let p = DeleteStyle {
            name: "imagestyle".to_string(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::DELETE);
    }
}

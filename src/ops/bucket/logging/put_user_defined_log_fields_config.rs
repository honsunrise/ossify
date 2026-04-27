//! PutUserDefinedLogFieldsConfig.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putuserdefinedlogfieldsconfig>

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
pub struct PutUserDefinedLogFieldsConfigParams {
    #[serde(rename = "userDefinedLogFieldsConfig")]
    user_defined_log_fields_config: OnlyKeyField,
}

/// `<HeaderSet>` wrapper around a repeated `<header>` element.
///
/// Note: OSS uses a lowercase `<header>` tag inside `<HeaderSet>` (unlike the
/// more common PascalCase convention).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeaderSet {
    #[serde(rename = "header", default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<String>,
}

/// `<ParamSet>` wrapper around a repeated lowercase `<parameter>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParamSet {
    #[serde(rename = "parameter", default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<String>,
}

/// Root `<UserDefinedLogFieldsConfiguration>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "UserDefinedLogFieldsConfiguration", rename_all = "PascalCase")]
pub struct UserDefinedLogFieldsConfiguration {
    pub header_set: Option<HeaderSet>,
    pub param_set: Option<ParamSet>,
}

pub struct PutUserDefinedLogFieldsConfig {
    pub config: UserDefinedLogFieldsConfiguration,
}

impl Ops for PutUserDefinedLogFieldsConfig {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<UserDefinedLogFieldsConfiguration>;
    type Query = PutUserDefinedLogFieldsConfigParams;

    fn prepare(
        self,
    ) -> Result<Prepared<PutUserDefinedLogFieldsConfigParams, UserDefinedLogFieldsConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutUserDefinedLogFieldsConfigParams::default()),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait PutUserDefinedLogFieldsConfigOps {
    /// Configure custom headers / query parameters to include in the
    /// `user_defined_log_fields` real-time log field.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putuserdefinedlogfieldsconfig>
    fn put_user_defined_log_fields_config(
        &self,
        config: UserDefinedLogFieldsConfiguration,
    ) -> impl Future<Output = Result<()>>;
}

impl PutUserDefinedLogFieldsConfigOps for Client {
    async fn put_user_defined_log_fields_config(
        &self,
        config: UserDefinedLogFieldsConfiguration,
    ) -> Result<()> {
        self.request(PutUserDefinedLogFieldsConfig { config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutUserDefinedLogFieldsConfigParams::default()).unwrap(),
            "userDefinedLogFieldsConfig"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = UserDefinedLogFieldsConfiguration {
            header_set: Some(HeaderSet {
                headers: vec!["h1".to_string(), "h2".to_string()],
            }),
            param_set: Some(ParamSet {
                parameters: vec!["p1".to_string()],
            }),
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<header>h1</header>"));
        assert!(xml.contains("<header>h2</header>"));
        assert!(xml.contains("<parameter>p1</parameter>"));
        let back: UserDefinedLogFieldsConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}

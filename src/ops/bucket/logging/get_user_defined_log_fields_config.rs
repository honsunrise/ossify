//! GetUserDefinedLogFieldsConfig.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getuserdefinedlogfieldsconfig>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_user_defined_log_fields_config::UserDefinedLogFieldsConfiguration;
#[allow(unused_imports)]
pub use super::put_user_defined_log_fields_config::{HeaderSet, ParamSet};
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserDefinedLogFieldsConfigParams {
    #[serde(rename = "userDefinedLogFieldsConfig")]
    user_defined_log_fields_config: OnlyKeyField,
}

pub struct GetUserDefinedLogFieldsConfig;

impl Ops for GetUserDefinedLogFieldsConfig {
    type Response = BodyResponseProcessor<UserDefinedLogFieldsConfiguration>;
    type Body = NoneBody;
    type Query = GetUserDefinedLogFieldsConfigParams;

    fn prepare(self) -> Result<Prepared<GetUserDefinedLogFieldsConfigParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetUserDefinedLogFieldsConfigParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetUserDefinedLogFieldsConfigOps {
    /// Retrieve the custom logging-field configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getuserdefinedlogfieldsconfig>
    fn get_user_defined_log_fields_config(
        &self,
    ) -> impl Future<Output = Result<UserDefinedLogFieldsConfiguration>>;
}

impl GetUserDefinedLogFieldsConfigOps for Client {
    async fn get_user_defined_log_fields_config(&self) -> Result<UserDefinedLogFieldsConfiguration> {
        self.request(GetUserDefinedLogFieldsConfig).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetUserDefinedLogFieldsConfigParams::default()).unwrap(),
            "userDefinedLogFieldsConfig"
        );
    }

    #[test]
    fn parse_response() {
        let xml = r#"<UserDefinedLogFieldsConfiguration>
<HeaderSet>
<header>header1</header>
<header>header2</header>
</HeaderSet>
<ParamSet>
<parameter>param1</parameter>
</ParamSet>
</UserDefinedLogFieldsConfiguration>"#;
        let parsed: UserDefinedLogFieldsConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.header_set.unwrap().headers.len(), 2);
        assert_eq!(parsed.param_set.unwrap().parameters, vec!["param1".to_string()]);
    }
}

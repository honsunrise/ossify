//! DeleteUserDefinedLogFieldsConfig.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteuserdefinedlogfieldsconfig>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUserDefinedLogFieldsConfigParams {
    #[serde(rename = "userDefinedLogFieldsConfig")]
    user_defined_log_fields_config: OnlyKeyField,
}

pub struct DeleteUserDefinedLogFieldsConfig;

impl Ops for DeleteUserDefinedLogFieldsConfig {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteUserDefinedLogFieldsConfigParams;

    fn prepare(self) -> Result<Prepared<DeleteUserDefinedLogFieldsConfigParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            query: Some(DeleteUserDefinedLogFieldsConfigParams::default()),
            ..Default::default()
        })
    }
}

pub trait DeleteUserDefinedLogFieldsConfigOps {
    /// Remove the custom logging-field configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deleteuserdefinedlogfieldsconfig>
    fn delete_user_defined_log_fields_config(&self) -> impl Future<Output = Result<()>>;
}

impl DeleteUserDefinedLogFieldsConfigOps for Client {
    async fn delete_user_defined_log_fields_config(&self) -> Result<()> {
        self.request(DeleteUserDefinedLogFieldsConfig).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&DeleteUserDefinedLogFieldsConfigParams::default()).unwrap(),
            "userDefinedLogFieldsConfig"
        );
    }
}

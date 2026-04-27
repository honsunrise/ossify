//! GetBucketHttpsConfig.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbuckethttpsconfig>

use std::future::Future;

use http::Method;
use serde::Serialize;

pub use super::put_bucket_https_config::HttpsConfiguration;
#[allow(unused_imports)]
pub use super::put_bucket_https_config::{CipherSuite, Tls};
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketHttpsConfigParams {
    #[serde(rename = "httpsConfig")]
    https_config: OnlyKeyField,
}

pub struct GetBucketHttpsConfig;

impl Ops for GetBucketHttpsConfig {
    type Response = BodyResponseProcessor<HttpsConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketHttpsConfigParams;

    fn prepare(self) -> Result<Prepared<GetBucketHttpsConfigParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketHttpsConfigParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketHttpsConfigOps {
    /// Query the bucket's TLS and cipher-suite configuration.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbuckethttpsconfig>
    fn get_bucket_https_config(&self) -> impl Future<Output = Result<HttpsConfiguration>>;
}

impl GetBucketHttpsConfigOps for Client {
    async fn get_bucket_https_config(&self) -> Result<HttpsConfiguration> {
        self.request(GetBucketHttpsConfig).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketHttpsConfigParams::default()).unwrap(),
            "httpsConfig"
        );
    }
}

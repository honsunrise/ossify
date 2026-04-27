//! GetBucketRequestPayment.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketrequestpayment>

use std::future::Future;

use http::Method;
use serde::Serialize;

#[allow(unused_imports)]
pub use super::put_bucket_request_payment::Payer;
pub use super::put_bucket_request_payment::RequestPaymentConfiguration;
use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketRequestPaymentParams {
    #[serde(rename = "requestPayment")]
    request_payment: OnlyKeyField,
}

pub struct GetBucketRequestPayment;

impl Ops for GetBucketRequestPayment {
    type Response = BodyResponseProcessor<RequestPaymentConfiguration>;
    type Body = NoneBody;
    type Query = GetBucketRequestPaymentParams;

    fn prepare(self) -> Result<Prepared<GetBucketRequestPaymentParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetBucketRequestPaymentParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetBucketRequestPaymentOps {
    /// Query the pay-by-requester configuration of the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getbucketrequestpayment>
    fn get_bucket_request_payment(&self) -> impl Future<Output = Result<RequestPaymentConfiguration>>;
}

impl GetBucketRequestPaymentOps for Client {
    async fn get_bucket_request_payment(&self) -> Result<RequestPaymentConfiguration> {
        self.request(GetBucketRequestPayment).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetBucketRequestPaymentParams::default()).unwrap(),
            "requestPayment"
        );
    }

    #[test]
    fn parse_response() {
        let xml = r#"<RequestPaymentConfiguration><Payer>BucketOwner</Payer></RequestPaymentConfiguration>"#;
        let parsed: RequestPaymentConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.payer, Payer::BucketOwner);
    }
}

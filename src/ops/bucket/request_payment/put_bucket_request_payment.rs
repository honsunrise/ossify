//! PutBucketRequestPayment.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketrequestpayment>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketRequestPaymentParams {
    #[serde(rename = "requestPayment")]
    request_payment: OnlyKeyField,
}

/// Who pays for request and traffic fees.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Payer {
    #[default]
    BucketOwner,
    Requester,
}

impl Payer {
    pub fn as_str(&self) -> &'static str {
        match self {
            Payer::BucketOwner => "BucketOwner",
            Payer::Requester => "Requester",
        }
    }
}

impl AsRef<str> for Payer {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Root `<RequestPaymentConfiguration>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "RequestPaymentConfiguration", rename_all = "PascalCase")]
pub struct RequestPaymentConfiguration {
    pub payer: Payer,
}

pub struct PutBucketRequestPayment {
    pub payer: Payer,
}

impl Ops for PutBucketRequestPayment {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<RequestPaymentConfiguration>;
    type Query = PutBucketRequestPaymentParams;

    fn prepare(self) -> Result<Prepared<PutBucketRequestPaymentParams, RequestPaymentConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            query: Some(PutBucketRequestPaymentParams::default()),
            body: Some(RequestPaymentConfiguration { payer: self.payer }),
            ..Default::default()
        })
    }
}

pub trait PutBucketRequestPaymentOps {
    /// Set who pays for request and traffic fees.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketrequestpayment>
    fn put_bucket_request_payment(&self, payer: Payer) -> impl Future<Output = Result<()>>;
}

impl PutBucketRequestPaymentOps for Client {
    async fn put_bucket_request_payment(&self, payer: Payer) -> Result<()> {
        self.request(PutBucketRequestPayment { payer }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&PutBucketRequestPaymentParams::default()).unwrap(),
            "requestPayment"
        );
    }

    #[test]
    fn body_round_trip() {
        let cfg = RequestPaymentConfiguration {
            payer: Payer::Requester,
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Payer>Requester</Payer>"));
        let back: RequestPaymentConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}

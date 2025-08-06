use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use bytes::Bytes;
use http::header::CONTENT_TYPE;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::{Error, Result};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub request_id: String,
    pub host_id: String,
    #[serde(rename = "EC")]
    pub ec: String,
    pub recommend_doc: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code={}, message={}, request_id={}, host_id={}, ec={:?}, recommend_doc={}",
            self.code, self.message, self.request_id, self.host_id, self.ec, self.recommend_doc
        )
    }
}

impl std::error::Error for ErrorResponse {}

async fn process_response_error(resp: reqwest::Response) -> Result<Error> {
    let status = resp.status();
    let text = resp.text().await?;
    let error = if text.trim().is_empty() {
        None
    } else {
        Some(Box::new(quick_xml::de::from_str::<ErrorResponse>(&text)?))
    };
    Ok(Error::ApiError {
        status_code: status,
        message: error,
    })
}

pub(crate) trait ResponseProcessor {
    type Output;

    fn from_response(response: reqwest::Response) -> impl Future<Output = Result<Self::Output>>;
}

pub(crate) struct EmptyResponseProcessor;

impl ResponseProcessor for EmptyResponseProcessor {
    type Output = ();

    async fn from_response(_: reqwest::Response) -> Result<()> {
        Ok(())
    }
}

pub(crate) struct BinaryResponseProcessor;

impl ResponseProcessor for BinaryResponseProcessor {
    type Output = Bytes;

    async fn from_response(resp: reqwest::Response) -> Result<Self::Output> {
        let status = resp.status();
        if status.is_success() {
            Ok(resp.bytes().await?)
        } else {
            Err(process_response_error(resp).await?)
        }
    }
}

pub(crate) struct HeaderResponseProcessor<T>(PhantomData<T>);

impl<T> ResponseProcessor for HeaderResponseProcessor<T>
where
    T: DeserializeOwned,
{
    type Output = T;

    async fn from_response(resp: reqwest::Response) -> Result<Self::Output> {
        let status = resp.status();
        if status.is_success() {
            let headers = resp.headers();
            let mut map = HashMap::with_capacity(headers.len());
            for (key, value) in headers.iter() {
                map.insert(key.as_str().to_string(), value.to_str()?.to_string());
            }
            let value = serde_json::to_value(map)?;
            Ok(serde_json::from_value(value)?)
        } else {
            Err(process_response_error(resp).await?)
        }
    }
}

pub(crate) struct BodyResponseProcessor<T>(PhantomData<T>);

impl<T> ResponseProcessor for BodyResponseProcessor<T>
where
    T: DeserializeOwned,
{
    type Output = T;

    async fn from_response(resp: reqwest::Response) -> Result<Self::Output> {
        let status = resp.status();
        if status.is_success() {
            let headers = resp.headers();
            let content_type = headers
                .get(CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/xml");
            match content_type {
                "application/xml" => {
                    let text = resp.text().await?;
                    Ok(quick_xml::de::from_str(&text)?)
                },
                "application/json" => Ok(resp.json::<T>().await?),
                _ => Err(Error::InvalidContentType(content_type.to_string())),
            }
        } else {
            Err(process_response_error(resp).await?)
        }
    }
}

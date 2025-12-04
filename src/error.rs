use std::fmt;

use http::StatusCode;
use http::header::{InvalidHeaderName, InvalidHeaderValue};
use serde::de;
use thiserror::Error;
use url::ParseError;

use crate::{response, ser};

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] ParseError),

    #[error("Convert header value to string error: {0}")]
    ConvertHeaderValue(#[from] http::header::ToStrError),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Invalid Header Name: {0}")]
    InvalidHeaderName(#[from] InvalidHeaderName),

    #[error("Invalid Header Value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("OSS service error: code={code}, message={message}")]
    OssService { code: String, message: String },

    #[error("Invalid XML(Serialize): {0}")]
    SerializeXml(#[from] quick_xml::se::SeError),

    #[error("Invalid XML(Deserialize): {0}")]
    DeserializeXml(#[from] quick_xml::de::DeError),

    #[error("Invalid Json: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("Invalid Content Type: {0}")]
    InvalidContentType(String),

    #[error("Invalid Payload(query or body): {0}")]
    InvalidPayload(#[from] ser::Error),

    #[error("Unexpected Error: {0}")]
    Unexpected(#[from] anyhow::Error),

    #[error("Invalid API response({status_code}): {message:?}")]
    ApiError {
        status_code: StatusCode,
        message: Option<Box<response::ErrorResponse>>,
    },

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Operation not supported")]
    NotSupported,

    #[error("Missing Host in endpoint")]
    MissingHost,

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Deserialize header error: {0}")]
    DeHeaderError(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::DeHeaderError(msg.to_string())
    }
}

//! CreateSelectObjectMeta: build a split/row index for a CSV or JSON-Lines
//! object so that later `SelectObject` calls can use `line-range` or
//! `split-range` filters.
//!
//! Like `SelectObject`, the response is a binary frame stream; the summary
//! statistics (splits / rows / cols) are carried in a single
//! [`SelectFrame::MetaEndCsv`] or [`SelectFrame::MetaEndJson`] frame.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createselectobjectmeta>

use std::future::Future;

use http::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::frame::SelectFrameStream;
use super::select_object::{CsvInputSerialization, JsonInputSerialization, SelectCompressionType};
use crate::body::XMLBody;
use crate::error::Result;
use crate::response::StreamResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct CreateSelectObjectMetaParams {
    #[serde(rename = "x-oss-process")]
    x_oss_process: String,
}

impl CreateSelectObjectMetaParams {
    fn csv() -> Self {
        Self {
            x_oss_process: "csv/meta".into(),
        }
    }

    fn json() -> Self {
        Self {
            x_oss_process: "json/meta".into(),
        }
    }
}

/// Input-serialization wrapper specifically for CSV meta requests.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvMetaInputSerialization {
    pub compression_type: Option<SelectCompressionType>,
    #[serde(rename = "CSV")]
    pub csv: Option<CsvInputSerialization>,
}

/// Input-serialization wrapper for JSON-LINES meta requests. (The server
/// rejects `DOCUMENT` in this context — only `LINES` is accepted.)
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JsonMetaInputSerialization {
    pub compression_type: Option<SelectCompressionType>,
    #[serde(rename = "JSON")]
    pub json: JsonInputSerialization,
}

/// Root `<CsvMetaRequest>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "CsvMetaRequest", rename_all = "PascalCase")]
pub struct CsvMetaRequest {
    pub input_serialization: CsvMetaInputSerialization,
    /// Whether to overwrite an existing meta-index. Note: the official docs
    /// list both `OverwriteIfExists` and `OverwriteIfExisting`; the server
    /// accepts the latter (matching the Go SDK).
    pub overwrite_if_existing: Option<bool>,
}

impl CsvMetaRequest {
    pub fn new(input: CsvInputSerialization) -> Self {
        Self {
            input_serialization: CsvMetaInputSerialization {
                compression_type: None,
                csv: Some(input),
            },
            overwrite_if_existing: None,
        }
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite_if_existing = Some(overwrite);
        self
    }

    pub fn with_compression(mut self, compression: SelectCompressionType) -> Self {
        self.input_serialization.compression_type = Some(compression);
        self
    }
}

/// Root `<JsonMetaRequest>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "JsonMetaRequest", rename_all = "PascalCase")]
pub struct JsonMetaRequest {
    pub input_serialization: JsonMetaInputSerialization,
    pub overwrite_if_existing: Option<bool>,
}

impl JsonMetaRequest {
    pub fn new(input: JsonInputSerialization) -> Self {
        Self {
            input_serialization: JsonMetaInputSerialization {
                compression_type: None,
                json: input,
            },
            overwrite_if_existing: None,
        }
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite_if_existing = Some(overwrite);
        self
    }

    pub fn with_compression(mut self, compression: SelectCompressionType) -> Self {
        self.input_serialization.compression_type = Some(compression);
        self
    }
}

pub struct CreateSelectCsvObjectMeta {
    pub key: String,
    pub request: CsvMetaRequest,
}

impl Ops for CreateSelectCsvObjectMeta {
    type Response = StreamResponseProcessor;
    type Body = XMLBody<CsvMetaRequest>;
    type Query = CreateSelectObjectMetaParams;

    fn prepare(self) -> Result<Prepared<CreateSelectObjectMetaParams, CsvMetaRequest>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.key),
            query: Some(CreateSelectObjectMetaParams::csv()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub struct CreateSelectJsonObjectMeta {
    pub key: String,
    pub request: JsonMetaRequest,
}

impl Ops for CreateSelectJsonObjectMeta {
    type Response = StreamResponseProcessor;
    type Body = XMLBody<JsonMetaRequest>;
    type Query = CreateSelectObjectMetaParams;

    fn prepare(self) -> Result<Prepared<CreateSelectObjectMetaParams, JsonMetaRequest>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.key),
            query: Some(CreateSelectObjectMetaParams::json()),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait CreateSelectObjectMetaOps {
    /// Build a meta-index for a CSV object.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/createselectobjectmeta>
    fn create_select_csv_object_meta(
        &self,
        key: impl Into<String>,
        request: CsvMetaRequest,
    ) -> impl Future<Output = Result<SelectFrameStream<reqwest::Response>>>;

    /// Build a meta-index for a JSON-Lines object.
    fn create_select_json_object_meta(
        &self,
        key: impl Into<String>,
        request: JsonMetaRequest,
    ) -> impl Future<Output = Result<SelectFrameStream<reqwest::Response>>>;
}

impl CreateSelectObjectMetaOps for Client {
    async fn create_select_csv_object_meta(
        &self,
        key: impl Into<String>,
        request: CsvMetaRequest,
    ) -> Result<SelectFrameStream<reqwest::Response>> {
        let resp = self
            .request(CreateSelectCsvObjectMeta {
                key: key.into(),
                request,
            })
            .await?;
        // Meta requests don't expose EnablePayloadCrc; skip verification.
        Ok(SelectFrameStream::new(resp, false))
    }

    async fn create_select_json_object_meta(
        &self,
        key: impl Into<String>,
        request: JsonMetaRequest,
    ) -> Result<SelectFrameStream<reqwest::Response>> {
        let resp = self
            .request(CreateSelectJsonObjectMeta {
                key: key.into(),
                request,
            })
            .await?;
        Ok(SelectFrameStream::new(resp, false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::object::select::select_object::{
        CsvInputSerialization,
        FileHeaderInfo,
        JsonInputSerialization,
        JsonType,
    };

    #[test]
    fn csv_meta_params_serialize() {
        assert_eq!(
            crate::ser::to_string(&CreateSelectObjectMetaParams::csv()).unwrap(),
            "x-oss-process=csv%2Fmeta"
        );
    }

    #[test]
    fn json_meta_params_serialize() {
        assert_eq!(
            crate::ser::to_string(&CreateSelectObjectMetaParams::json()).unwrap(),
            "x-oss-process=json%2Fmeta"
        );
    }

    #[test]
    fn csv_meta_body_xml() {
        let req = CsvMetaRequest::new(CsvInputSerialization {
            file_header_info: Some(FileHeaderInfo::None),
            ..Default::default()
        })
        .overwrite(true)
        .with_compression(SelectCompressionType::None);
        let xml = quick_xml::se::to_string(&req).unwrap();
        assert!(xml.contains("<CsvMetaRequest>"));
        assert!(xml.contains("<CompressionType>None</CompressionType>"));
        assert!(xml.contains("<OverwriteIfExisting>true</OverwriteIfExisting>"));
    }

    #[test]
    fn json_meta_body_xml() {
        let req = JsonMetaRequest::new(JsonInputSerialization {
            json_type: JsonType::Lines,
            range: None,
            parse_json_number_as_string: None,
        })
        .overwrite(false);
        let xml = quick_xml::se::to_string(&req).unwrap();
        assert!(xml.contains("<JsonMetaRequest>"));
        assert!(xml.contains("<Type>LINES</Type>"));
        assert!(xml.contains("<OverwriteIfExisting>false</OverwriteIfExisting>"));
    }
}

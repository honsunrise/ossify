//! SelectObject: run a restricted SQL query over a CSV or JSON object.
//!
//! The request body is the same `<SelectRequest>` element for both formats;
//! pick the format by setting the matching input- and output-serialization
//! variants. The response is a stream of binary frames decoded by
//! [`super::frame::SelectFrame`].
//!
//! The SQL expression and the single-byte delimiters / quote characters
//! in the XML must all be base64-encoded — the public `new_csv(...)` and
//! `new_json(...)` constructors handle this for you.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/selectobject>

use std::future::Future;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use http::Method;
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::frame::SelectFrameStream;
use crate::body::XMLBody;
use crate::error::Result;
use crate::response::StreamResponseProcessor;
use crate::{Client, Ops, Prepared, Request};

/// Query parameters for [`SelectObject`].
#[derive(Debug, Clone, Serialize)]
pub struct SelectObjectParams {
    #[serde(rename = "x-oss-process")]
    x_oss_process: String,
}

impl SelectObjectParams {
    fn csv() -> Self {
        Self {
            x_oss_process: "csv/select".into(),
        }
    }

    fn json() -> Self {
        Self {
            x_oss_process: "json/select".into(),
        }
    }
}

// --------------------------------------------------------------------------
// Request XML types
// --------------------------------------------------------------------------

/// How to interpret the first line of a CSV object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum FileHeaderInfo {
    /// No header line.
    None,
    /// Treat the first line as a header but don't reference it in the query.
    Ignore,
    /// Treat the first line as a header and allow column-name references.
    Use,
}

/// JSON object layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum JsonType {
    /// Entire object is one JSON document.
    #[serde(rename = "DOCUMENT")]
    Document,
    /// One JSON object per line (newline-delimited).
    #[serde(rename = "LINES")]
    Lines,
}

/// Compression format of the source object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SelectCompressionType {
    None,
    #[serde(rename = "GZIP")]
    Gzip,
}

/// CSV-specific input serialization.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvInputSerialization {
    pub file_header_info: Option<FileHeaderInfo>,
    /// Base64-encoded record delimiter (≤ 2 bytes). Default `\n`.
    pub record_delimiter: Option<String>,
    /// Base64-encoded field delimiter (1 byte). Default `,`.
    pub field_delimiter: Option<String>,
    /// Base64-encoded quote character (1 byte). Default `"`.
    pub quote_character: Option<String>,
    /// Base64-encoded comment character (1 byte). Default empty.
    pub comment_character: Option<String>,
    /// `line-range=<start>-<end>` or `split-range=<start>-<end>`. Requires
    /// that `CreateSelectObjectMeta` has been called first.
    pub range: Option<String>,
    pub allow_quoted_record_delimiter: Option<bool>,
}

/// JSON-specific input serialization.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JsonInputSerialization {
    #[serde(rename = "Type")]
    pub json_type: JsonType,
    /// `line-range=<start>-<end>` (LINES only).
    pub range: Option<String>,
    pub parse_json_number_as_string: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SelectInputSerialization {
    pub compression_type: Option<SelectCompressionType>,
    #[serde(rename = "CSV")]
    pub csv: Option<CsvInputSerialization>,
    #[serde(rename = "JSON")]
    pub json: Option<JsonInputSerialization>,
}

/// CSV-specific output serialization.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvOutputSerialization {
    /// Base64-encoded record delimiter (≤ 2 bytes). Default `\n`.
    pub record_delimiter: Option<String>,
    /// Base64-encoded field delimiter (1 byte). Default `,`.
    pub field_delimiter: Option<String>,
}

/// JSON-specific output serialization.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct JsonOutputSerialization {
    /// Base64-encoded record delimiter (default `\n`).
    pub record_delimiter: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SelectOutputSerialization {
    #[serde(rename = "CSV")]
    pub csv: Option<CsvOutputSerialization>,
    #[serde(rename = "JSON")]
    pub json: Option<JsonOutputSerialization>,
    /// Return all columns in the CSV output even if some were not selected.
    pub keep_all_columns: Option<bool>,
    /// Return the response body as raw bytes without the frame envelope.
    /// **Incompatible with `enable_payload_crc = true`.**
    pub output_raw_data: Option<bool>,
    /// Attach a CRC-32 to each frame payload.
    pub enable_payload_crc: Option<bool>,
    /// Output the CSV header row as the first line of the response.
    pub output_header: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SelectOptions {
    pub skip_partial_data_record: Option<bool>,
    pub max_skipped_records_allowed: Option<u64>,
}

/// Root `<SelectRequest>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "SelectRequest", rename_all = "PascalCase")]
pub struct SelectRequest {
    /// Base64-encoded SQL expression.
    pub expression: String,
    pub input_serialization: SelectInputSerialization,
    pub output_serialization: SelectOutputSerialization,
    pub options: Option<SelectOptions>,
}

impl SelectRequest {
    /// Build a CSV Select request. The SQL `expression` is base64-encoded
    /// automatically.
    pub fn new_csv(
        expression: impl AsRef<str>,
        input: CsvInputSerialization,
        output: CsvOutputSerialization,
    ) -> Self {
        Self {
            expression: BASE64.encode(expression.as_ref().as_bytes()),
            input_serialization: SelectInputSerialization {
                compression_type: None,
                csv: Some(input),
                json: None,
            },
            output_serialization: SelectOutputSerialization {
                csv: Some(output),
                json: None,
                ..Default::default()
            },
            options: None,
        }
    }

    /// Build a JSON Select request. The SQL `expression` is base64-encoded
    /// automatically.
    pub fn new_json(
        expression: impl AsRef<str>,
        input: JsonInputSerialization,
        output: JsonOutputSerialization,
    ) -> Self {
        Self {
            expression: BASE64.encode(expression.as_ref().as_bytes()),
            input_serialization: SelectInputSerialization {
                compression_type: None,
                csv: None,
                json: Some(input),
            },
            output_serialization: SelectOutputSerialization {
                csv: None,
                json: Some(output),
                ..Default::default()
            },
            options: None,
        }
    }

    pub fn with_compression(mut self, compression: SelectCompressionType) -> Self {
        self.input_serialization.compression_type = Some(compression);
        self
    }

    pub fn with_options(mut self, options: SelectOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Convenience: enable frame-level payload CRC. The corresponding stream
    /// consumer should be constructed with the same flag.
    pub fn with_payload_crc(mut self, enable: bool) -> Self {
        self.output_serialization.enable_payload_crc = Some(enable);
        self
    }
}

/// Helper to base64-encode a single-byte delimiter such as `,`, `\n`, or `"`.
pub fn b64_delimiter(ch: &[u8]) -> String {
    BASE64.encode(ch)
}

// --------------------------------------------------------------------------
// Ops
// --------------------------------------------------------------------------

/// The `SelectObject` operation. `verify_payload_crc` is stored on the
/// operation struct so the returned `SelectFrameStream` can match the
/// request's `enable_payload_crc` setting without the caller repeating it.
pub struct SelectObject {
    pub key: String,
    pub request: SelectRequest,
    pub is_json: bool,
}

impl Ops for SelectObject {
    type Response = StreamResponseProcessor;
    type Body = XMLBody<SelectRequest>;
    type Query = SelectObjectParams;

    fn prepare(self) -> Result<Prepared<SelectObjectParams, SelectRequest>> {
        let query = if self.is_json {
            SelectObjectParams::json()
        } else {
            SelectObjectParams::csv()
        };
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.key),
            query: Some(query),
            body: Some(self.request),
            ..Default::default()
        })
    }
}

pub trait SelectObjectOps {
    /// Run a CSV Select query and return a [`SelectFrameStream`] over the
    /// decoded frames. The caller must inspect the final `End` /
    /// `MetaEnd*` frame and treat any non-2xx `status` as a failure
    /// (the outer HTTP response may be 206 even when the select itself
    /// ultimately errored).
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/selectobject>
    fn select_object_csv(
        &self,
        key: impl Into<String>,
        request: SelectRequest,
    ) -> impl Future<Output = Result<SelectFrameStream<reqwest::Response>>>;

    /// Run a JSON Select query; see [`select_object_csv`] for details.
    fn select_object_json(
        &self,
        key: impl Into<String>,
        request: SelectRequest,
    ) -> impl Future<Output = Result<SelectFrameStream<reqwest::Response>>>;
}

impl SelectObjectOps for Client {
    async fn select_object_csv(
        &self,
        key: impl Into<String>,
        request: SelectRequest,
    ) -> Result<SelectFrameStream<reqwest::Response>> {
        let verify_crc = request.output_serialization.enable_payload_crc.unwrap_or(false);
        let resp = self
            .request(SelectObject {
                key: key.into(),
                request,
                is_json: false,
            })
            .await?;
        Ok(SelectFrameStream::new(resp, verify_crc))
    }

    async fn select_object_json(
        &self,
        key: impl Into<String>,
        request: SelectRequest,
    ) -> Result<SelectFrameStream<reqwest::Response>> {
        let verify_crc = request.output_serialization.enable_payload_crc.unwrap_or(false);
        let resp = self
            .request(SelectObject {
                key: key.into(),
                request,
                is_json: true,
            })
            .await?;
        Ok(SelectFrameStream::new(resp, verify_crc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_csv_serialize() {
        assert_eq!(
            crate::ser::to_string(&SelectObjectParams::csv()).unwrap(),
            "x-oss-process=csv%2Fselect"
        );
    }

    #[test]
    fn params_json_serialize() {
        assert_eq!(
            crate::ser::to_string(&SelectObjectParams::json()).unwrap(),
            "x-oss-process=json%2Fselect"
        );
    }

    #[test]
    fn sql_expression_is_base64_encoded() {
        let req = SelectRequest::new_csv(
            "select * from ossobject where _1 > 10",
            CsvInputSerialization::default(),
            CsvOutputSerialization::default(),
        );
        // Verify the raw SQL is NOT in the serialized XML.
        let xml = quick_xml::se::to_string(&req).unwrap();
        assert!(!xml.contains("select *"));
        // And the base64 is present.
        let expected = BASE64.encode(b"select * from ossobject where _1 > 10");
        assert!(xml.contains(&format!("<Expression>{expected}</Expression>")));
    }

    #[test]
    fn csv_request_xml_round_trip() {
        let input = CsvInputSerialization {
            file_header_info: Some(FileHeaderInfo::Ignore),
            record_delimiter: Some(b64_delimiter(b"\n")),
            field_delimiter: Some(b64_delimiter(b",")),
            ..Default::default()
        };
        let output = CsvOutputSerialization {
            record_delimiter: Some(b64_delimiter(b"\n")),
            field_delimiter: Some(b64_delimiter(b",")),
        };
        let req = SelectRequest::new_csv("select _1 from ossobject", input, output).with_payload_crc(true);
        let xml = quick_xml::se::to_string(&req).unwrap();
        assert!(xml.contains("<FileHeaderInfo>Ignore</FileHeaderInfo>"));
        assert!(xml.contains("<EnablePayloadCrc>true</EnablePayloadCrc>"));
        // The shape must contain both <CSV> blocks (input + output).
        assert_eq!(xml.matches("<CSV>").count(), 2);
    }

    #[test]
    fn json_request_xml_uses_json_block() {
        let input = JsonInputSerialization {
            json_type: JsonType::Document,
            range: None,
            parse_json_number_as_string: None,
        };
        let output = JsonOutputSerialization {
            record_delimiter: Some(b64_delimiter(b"\n")),
        };
        let req = SelectRequest::new_json("select * from ossobject.records[*]", input, output);
        let xml = quick_xml::se::to_string(&req).unwrap();
        assert!(xml.contains("<Type>DOCUMENT</Type>"));
        assert_eq!(xml.matches("<JSON>").count(), 2);
        assert!(!xml.contains("<CSV>"));
    }

    #[test]
    fn b64_delimiter_matches_alicloud_examples() {
        // From the official documentation: '\n' -> Cg==, ',' -> LA==, '"' -> Ig==
        assert_eq!(b64_delimiter(b"\n"), "Cg==");
        assert_eq!(b64_delimiter(b","), "LA==");
        assert_eq!(b64_delimiter(b"\""), "Ig==");
    }
}

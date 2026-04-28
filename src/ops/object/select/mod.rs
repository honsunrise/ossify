//! `SelectObject` / `CreateSelectObjectMeta` operations.
//!
//! These two APIs run a small SQL dialect over CSV and JSON objects and
//! return a binary frame stream. The decoder lives in [`frame`]; the
//! request/response shapes live in [`select_object`] and
//! [`create_select_object_meta`].
//!
//! Official category:
//! <https://www.alibabacloud.com/help/en/oss/developer-reference/selectobject>

pub mod create_select_object_meta;
pub mod frame;
pub mod select_object;

pub use create_select_object_meta::{
    CreateSelectCsvObjectMeta,
    CreateSelectJsonObjectMeta,
    CreateSelectObjectMetaOps,
    CreateSelectObjectMetaParams,
    CsvMetaInputSerialization,
    CsvMetaRequest,
    JsonMetaInputSerialization,
    JsonMetaRequest,
};
pub use frame::{SelectFrame, SelectFrameStream};
pub use select_object::{
    CsvInputSerialization,
    CsvOutputSerialization,
    FileHeaderInfo,
    JsonInputSerialization,
    JsonOutputSerialization,
    JsonType,
    SelectCompressionType,
    SelectInputSerialization,
    SelectObject,
    SelectObjectOps,
    SelectObjectParams,
    SelectOptions,
    SelectOutputSerialization,
    SelectRequest,
    b64_delimiter,
};

pub trait SelectObjectOperations: SelectObjectOps + CreateSelectObjectMetaOps {}
impl<T> SelectObjectOperations for T where T: SelectObjectOps + CreateSelectObjectMetaOps {}

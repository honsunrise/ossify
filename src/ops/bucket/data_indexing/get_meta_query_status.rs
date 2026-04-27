//! GetMetaQueryStatus.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getmetaquerystatus>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Deserializer, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMetaQueryStatusParams {
    #[serde(rename = "metaQuery")]
    meta_query: OnlyKeyField,
}

/// State of the metadata-index library.
///
/// Uses a manual `Deserialize` impl because quick-xml 0.39 treats derived
/// enums specially when they appear as the text content of an element
/// (`<State>Running</State>`): it tries to use the element tag (`State`) as
/// the variant name instead of the text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MetaQueryState {
    Ready,
    Stop,
    Running,
    Retrying,
    Failed,
    Deleted,
    /// Forward-compatible placeholder.
    Other(String),
}

impl<'de> Deserialize<'de> for MetaQueryState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Ready" => Self::Ready,
            "Stop" => Self::Stop,
            "Running" => Self::Running,
            "Retrying" => Self::Retrying,
            "Failed" => Self::Failed,
            "Deleted" => Self::Deleted,
            _ => Self::Other(s),
        })
    }
}

/// Scan phase of the metadata-index library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MetaQueryPhase {
    FullScanning,
    IncrementalScanning,
    Other(String),
}

impl<'de> Deserialize<'de> for MetaQueryPhase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "FullScanning" => Self::FullScanning,
            "IncrementalScanning" => Self::IncrementalScanning,
            _ => Self::Other(s),
        })
    }
}

/// Response body (XML root `<MetaQueryStatus>`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "MetaQueryStatus", rename_all = "PascalCase")]
pub struct MetaQueryStatus {
    pub state: MetaQueryState,
    pub phase: Option<MetaQueryPhase>,
    pub create_time: String,
    pub update_time: String,
    pub meta_query_mode: Option<String>,
}

pub struct GetMetaQueryStatus;

impl Ops for GetMetaQueryStatus {
    type Response = BodyResponseProcessor<MetaQueryStatus>;
    type Body = NoneBody;
    type Query = GetMetaQueryStatusParams;

    fn prepare(self) -> Result<Prepared<GetMetaQueryStatusParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(GetMetaQueryStatusParams::default()),
            ..Default::default()
        })
    }
}

pub trait GetMetaQueryStatusOps {
    /// Query the status of the metadata-index library.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getmetaquerystatus>
    fn get_meta_query_status(&self) -> impl Future<Output = Result<MetaQueryStatus>>;
}

impl GetMetaQueryStatusOps for Client {
    async fn get_meta_query_status(&self) -> Result<MetaQueryStatus> {
        self.request(GetMetaQueryStatus).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        assert_eq!(
            crate::ser::to_string(&GetMetaQueryStatusParams::default()).unwrap(),
            "metaQuery"
        );
    }

    #[test]
    fn parse_status_response() {
        let xml = r#"<MetaQueryStatus>
  <State>Running</State>
  <Phase>FullScanning</Phase>
  <CreateTime>2024-09-11T10:49:17.289+08:00</CreateTime>
  <UpdateTime>2024-09-11T10:49:18.000+08:00</UpdateTime>
  <MetaQueryMode>basic</MetaQueryMode>
</MetaQueryStatus>"#;
        let parsed: MetaQueryStatus = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.state, MetaQueryState::Running);
        assert_eq!(parsed.phase, Some(MetaQueryPhase::FullScanning));
    }
}

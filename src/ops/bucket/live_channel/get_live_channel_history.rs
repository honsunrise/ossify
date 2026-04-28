//! GetLiveChannelHistory: query recent push history of a LiveChannel (up to 10 records).
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getlivechannelhistory>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::LiveChannelHistoryRecord;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetLiveChannelHistoryParams {
    pub(crate) live: OnlyKeyField,
    pub comp: String,
}

impl Default for GetLiveChannelHistoryParams {
    fn default() -> Self {
        Self {
            live: OnlyKeyField,
            comp: "history".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "LiveChannelHistory", rename_all = "PascalCase")]
pub struct GetLiveChannelHistoryResponse {
    #[serde(rename = "LiveRecord", default)]
    pub records: Vec<LiveChannelHistoryRecord>,
}

pub struct GetLiveChannelHistory {
    pub channel_name: String,
}

impl Ops for GetLiveChannelHistory {
    type Response = BodyResponseProcessor<GetLiveChannelHistoryResponse>;
    type Body = NoneBody;
    type Query = GetLiveChannelHistoryParams;

    fn prepare(self) -> Result<Prepared<GetLiveChannelHistoryParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.channel_name),
            query: Some(GetLiveChannelHistoryParams::default()),
            ..Default::default()
        })
    }
}

/// Trait for GetLiveChannelHistory operations.
pub trait GetLiveChannelHistoryOperations {
    fn get_live_channel_history(
        &self,
        channel_name: impl Into<String>,
    ) -> impl Future<Output = Result<GetLiveChannelHistoryResponse>>;
}

impl GetLiveChannelHistoryOperations for Client {
    async fn get_live_channel_history(
        &self,
        channel_name: impl Into<String>,
    ) -> Result<GetLiveChannelHistoryResponse> {
        let ops = GetLiveChannelHistory {
            channel_name: channel_name.into(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&GetLiveChannelHistoryParams::default()).unwrap();
        assert_eq!(q, "comp=history&live");
    }

    #[test]
    fn test_deserialize_response() {
        let xml = r#"<LiveChannelHistory>
  <LiveRecord>
    <StartTime>2024-01-01T00:00:00Z</StartTime>
    <EndTime>2024-01-01T01:00:00Z</EndTime>
    <RemoteAddr>1.2.3.4</RemoteAddr>
  </LiveRecord>
  <LiveRecord>
    <StartTime>2024-01-02T00:00:00Z</StartTime>
    <EndTime>2024-01-02T00:30:00Z</EndTime>
    <RemoteAddr>5.6.7.8</RemoteAddr>
  </LiveRecord>
</LiveChannelHistory>"#;
        let resp: GetLiveChannelHistoryResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(resp.records.len(), 2);
        assert_eq!(resp.records[1].remote_addr, "5.6.7.8");
    }
}

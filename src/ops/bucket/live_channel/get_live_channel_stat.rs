//! GetLiveChannelStat: query current streaming statistics of a LiveChannel.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getlivechannelstat>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::{LiveChannelAudioStat, LiveChannelVideoStat};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
pub struct GetLiveChannelStatParams {
    pub(crate) live: OnlyKeyField,
    pub comp: String,
}

impl Default for GetLiveChannelStatParams {
    fn default() -> Self {
        Self {
            live: OnlyKeyField,
            comp: "stat".into(),
        }
    }
}

/// Overall LiveChannel stream state.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum LiveChannelStreamStatus {
    Disabled,
    Live,
    Idle,
}

/// GetLiveChannelStat response body `<LiveChannelStat>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "LiveChannelStat", rename_all = "PascalCase")]
pub struct GetLiveChannelStatResponse {
    pub status: LiveChannelStreamStatus,
    pub connected_time: Option<String>,
    pub remote_addr: Option<String>,
    pub video: Option<LiveChannelVideoStat>,
    pub audio: Option<LiveChannelAudioStat>,
}

pub struct GetLiveChannelStat {
    pub channel_name: String,
}

impl Ops for GetLiveChannelStat {
    type Response = BodyResponseProcessor<GetLiveChannelStatResponse>;
    type Body = NoneBody;
    type Query = GetLiveChannelStatParams;

    fn prepare(self) -> Result<Prepared<GetLiveChannelStatParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.channel_name),
            query: Some(GetLiveChannelStatParams::default()),
            ..Default::default()
        })
    }
}

/// Trait for GetLiveChannelStat operations.
pub trait GetLiveChannelStatOperations {
    fn get_live_channel_stat(
        &self,
        channel_name: impl Into<String>,
    ) -> impl Future<Output = Result<GetLiveChannelStatResponse>>;
}

impl GetLiveChannelStatOperations for Client {
    async fn get_live_channel_stat(
        &self,
        channel_name: impl Into<String>,
    ) -> Result<GetLiveChannelStatResponse> {
        let ops = GetLiveChannelStat {
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
        let q = crate::ser::to_string(&GetLiveChannelStatParams::default()).unwrap();
        assert_eq!(q, "comp=stat&live");
    }

    #[test]
    fn test_deserialize_live() {
        let xml = r#"<LiveChannelStat>
  <Status>Live</Status>
  <ConnectedTime>2024-01-01T00:00:00Z</ConnectedTime>
  <RemoteAddr>1.2.3.4</RemoteAddr>
  <Video>
    <Width>1920</Width><Height>1080</Height>
    <FrameRate>30</FrameRate>
    <Bandwidth>2000000</Bandwidth>
    <Codec>H264</Codec>
  </Video>
</LiveChannelStat>"#;
        let resp: GetLiveChannelStatResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(resp.status, LiveChannelStreamStatus::Live);
        assert_eq!(resp.video.unwrap().width, Some(1920));
    }

    #[test]
    fn test_deserialize_disabled() {
        let xml = r#"<LiveChannelStat><Status>Disabled</Status></LiveChannelStat>"#;
        let resp: GetLiveChannelStatResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(resp.status, LiveChannelStreamStatus::Disabled);
    }
}

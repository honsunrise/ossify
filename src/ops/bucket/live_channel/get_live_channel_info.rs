//! GetLiveChannelInfo: query a LiveChannel's configuration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getlivechannelinfo>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::LiveChannelConfiguration;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLiveChannelInfoParams {
    pub(crate) live: OnlyKeyField,
}

pub struct GetLiveChannelInfo {
    pub channel_name: String,
}

impl Ops for GetLiveChannelInfo {
    type Response = BodyResponseProcessor<LiveChannelConfiguration>;
    type Body = NoneBody;
    type Query = GetLiveChannelInfoParams;

    fn prepare(self) -> Result<Prepared<GetLiveChannelInfoParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.channel_name),
            query: Some(GetLiveChannelInfoParams::default()),
            ..Default::default()
        })
    }
}

/// Trait for GetLiveChannelInfo operations.
pub trait GetLiveChannelInfoOperations {
    fn get_live_channel_info(
        &self,
        channel_name: impl Into<String>,
    ) -> impl Future<Output = Result<LiveChannelConfiguration>>;
}

impl GetLiveChannelInfoOperations for Client {
    async fn get_live_channel_info(
        &self,
        channel_name: impl Into<String>,
    ) -> Result<LiveChannelConfiguration> {
        let ops = GetLiveChannelInfo {
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
        assert_eq!(crate::ser::to_string(&GetLiveChannelInfoParams::default()).unwrap(), "live");
    }

    #[test]
    fn test_deserialize_response() {
        let xml = r#"<LiveChannelConfiguration>
  <Description>d</Description>
  <Status>enabled</Status>
  <Target>
    <Type>HLS</Type>
    <FragDuration>5</FragDuration>
    <FragCount>3</FragCount>
    <PlaylistName>p.m3u8</PlaylistName>
  </Target>
</LiveChannelConfiguration>"#;
        let cfg: LiveChannelConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(cfg.target.target_type, "HLS");
        assert_eq!(cfg.target.frag_duration, Some(5));
    }
}

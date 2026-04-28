//! PutLiveChannel: create a LiveChannel under a bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putlivechannel>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::ops::common::{LiveChannelConfiguration, LiveChannelUrls};
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// PutLiveChannel query parameters: `?live`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutLiveChannelParams {
    pub(crate) live: OnlyKeyField,
}

/// PutLiveChannel response body `<CreateLiveChannelResult>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "CreateLiveChannelResult", rename_all = "PascalCase")]
pub struct PutLiveChannelResponse {
    pub publish_urls: LiveChannelUrls,
    pub play_urls: LiveChannelUrls,
}

/// PutLiveChannel operation.
pub struct PutLiveChannel {
    pub channel_name: String,
    pub params: PutLiveChannelParams,
    pub body: LiveChannelConfiguration,
}

impl Ops for PutLiveChannel {
    type Response = BodyResponseProcessor<PutLiveChannelResponse>;
    type Body = XMLBody<LiveChannelConfiguration>;
    type Query = PutLiveChannelParams;

    fn prepare(self) -> Result<Prepared<PutLiveChannelParams, LiveChannelConfiguration>> {
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.channel_name),
            query: Some(self.params),
            body: Some(self.body),
            ..Default::default()
        })
    }
}

/// Trait for PutLiveChannel operations.
pub trait PutLiveChannelOperations {
    fn put_live_channel(
        &self,
        channel_name: impl Into<String>,
        configuration: LiveChannelConfiguration,
    ) -> impl Future<Output = Result<PutLiveChannelResponse>>;
}

impl PutLiveChannelOperations for Client {
    async fn put_live_channel(
        &self,
        channel_name: impl Into<String>,
        configuration: LiveChannelConfiguration,
    ) -> Result<PutLiveChannelResponse> {
        let ops = PutLiveChannel {
            channel_name: channel_name.into(),
            params: PutLiveChannelParams::default(),
            body: configuration,
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::common::LiveChannelTarget;

    #[test]
    fn test_serialize_params() {
        assert_eq!(crate::ser::to_string(&PutLiveChannelParams::default()).unwrap(), "live");
    }

    #[test]
    fn test_deserialize_response() {
        let xml = r#"<CreateLiveChannelResult>
  <PublishUrls><Url>rtmp://x</Url></PublishUrls>
  <PlayUrls><Url>http://x.m3u8</Url></PlayUrls>
</CreateLiveChannelResult>"#;
        let resp: PutLiveChannelResponse = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(resp.publish_urls.urls[0], "rtmp://x");
        assert_eq!(resp.play_urls.urls[0], "http://x.m3u8");
    }

    #[test]
    fn test_configuration_round_trip() {
        let cfg = LiveChannelConfiguration::new(LiveChannelTarget::hls());
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Type>HLS</Type>"));
    }
}

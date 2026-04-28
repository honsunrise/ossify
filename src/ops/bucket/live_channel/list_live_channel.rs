//! ListLiveChannel: list LiveChannels under a bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listlivechannel>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::LiveChannelSummary;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// ListLiveChannel query parameters.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListLiveChannelParams {
    pub(crate) live: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<String>,
}

impl ListLiveChannelParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, v: impl Into<String>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    pub fn marker(mut self, v: impl Into<String>) -> Self {
        self.marker = Some(v.into());
        self
    }

    pub fn max_keys(mut self, v: impl Into<String>) -> Self {
        self.max_keys = Some(v.into());
        self
    }
}

/// ListLiveChannel response body `<ListLiveChannelResult>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "ListLiveChannelResult", rename_all = "PascalCase")]
pub struct ListLiveChannelResponse {
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub max_keys: Option<String>,
    pub is_truncated: bool,
    pub next_marker: Option<String>,
    #[serde(rename = "LiveChannel", default)]
    pub live_channels: Vec<LiveChannelSummary>,
}

pub struct ListLiveChannel {
    pub params: ListLiveChannelParams,
}

impl Ops for ListLiveChannel {
    type Response = BodyResponseProcessor<ListLiveChannelResponse>;
    type Body = NoneBody;
    type Query = ListLiveChannelParams;

    fn prepare(self) -> Result<Prepared<ListLiveChannelParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for ListLiveChannel operations.
pub trait ListLiveChannelOperations {
    fn list_live_channel(
        &self,
        params: Option<ListLiveChannelParams>,
    ) -> impl Future<Output = Result<ListLiveChannelResponse>>;
}

impl ListLiveChannelOperations for Client {
    async fn list_live_channel(
        &self,
        params: Option<ListLiveChannelParams>,
    ) -> Result<ListLiveChannelResponse> {
        let ops = ListLiveChannel {
            params: params.unwrap_or_default(),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params_full() {
        let p = ListLiveChannelParams::new()
            .prefix("abc")
            .marker("m")
            .max_keys("50");
        let q = crate::ser::to_string(&p).unwrap();
        assert_eq!(q, "live&marker=m&max-keys=50&prefix=abc");
    }

    #[test]
    fn test_serialize_params_default() {
        let q = crate::ser::to_string(&ListLiveChannelParams::default()).unwrap();
        assert_eq!(q, "live");
    }

    #[test]
    fn test_deserialize_response() {
        let xml = r#"<ListLiveChannelResult>
  <Prefix></Prefix>
  <Marker></Marker>
  <MaxKeys>100</MaxKeys>
  <IsTruncated>false</IsTruncated>
  <LiveChannel>
    <Name>abc</Name>
    <Description>d</Description>
    <Status>enabled</Status>
    <LastModified>2024-01-01T00:00:00Z</LastModified>
    <PublishUrls><Url>rtmp://x/abc</Url></PublishUrls>
    <PlayUrls><Url>http://x/abc.m3u8</Url></PlayUrls>
  </LiveChannel>
</ListLiveChannelResult>"#;
        let resp: ListLiveChannelResponse = quick_xml::de::from_str(xml).unwrap();
        assert!(!resp.is_truncated);
        assert_eq!(resp.live_channels.len(), 1);
        assert_eq!(resp.live_channels[0].name, "abc");
    }
}

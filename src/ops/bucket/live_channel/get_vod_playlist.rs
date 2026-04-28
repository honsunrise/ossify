//! GetVodPlaylist: query a VOD playlist.
//!
//! Returns the raw m3u8 playlist text (`Content-Type: application/x-mpegURL`).
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/getvodplaylist>

use std::future::Future;

use bytes::Bytes;
use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::BinaryResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVodPlaylistParams {
    pub(crate) vod: OnlyKeyField,
    pub start_time: u64,
    pub end_time: u64,
}

impl GetVodPlaylistParams {
    pub fn new(start_time: u64, end_time: u64) -> Self {
        Self {
            vod: OnlyKeyField,
            start_time,
            end_time,
        }
    }
}

pub struct GetVodPlaylist {
    pub channel_name: String,
    pub params: GetVodPlaylistParams,
}

impl Ops for GetVodPlaylist {
    type Response = BinaryResponseProcessor;
    type Body = NoneBody;
    type Query = GetVodPlaylistParams;

    fn prepare(self) -> Result<Prepared<GetVodPlaylistParams>> {
        Ok(Prepared {
            method: Method::GET,
            key: Some(self.channel_name),
            query: Some(self.params),
            ..Default::default()
        })
    }
}

/// Trait for GetVodPlaylist operations.
pub trait GetVodPlaylistOperations {
    fn get_vod_playlist(
        &self,
        channel_name: impl Into<String>,
        start_time: u64,
        end_time: u64,
    ) -> impl Future<Output = Result<Bytes>>;
}

impl GetVodPlaylistOperations for Client {
    async fn get_vod_playlist(
        &self,
        channel_name: impl Into<String>,
        start_time: u64,
        end_time: u64,
    ) -> Result<Bytes> {
        let ops = GetVodPlaylist {
            channel_name: channel_name.into(),
            params: GetVodPlaylistParams::new(start_time, end_time),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&GetVodPlaylistParams::new(1700000000, 1700001000)).unwrap();
        assert_eq!(q, "endTime=1700001000&startTime=1700000000&vod");
    }

    #[test]
    fn test_prepare_method() {
        let p = GetVodPlaylist {
            channel_name: "chan".into(),
            params: GetVodPlaylistParams::new(1, 2),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::GET);
        assert_eq!(p.key.as_deref(), Some("chan"));
    }
}

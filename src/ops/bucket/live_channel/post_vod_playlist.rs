//! PostVodPlaylist: generate a VOD playlist from recorded fragments.
//!
//! Creates a m3u8 playlist at `ChannelName/PlaylistName` (must end with `.m3u8`)
//! covering the Unix time range `[startTime, endTime)`. The range must be
//! strictly less than 1 day.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/postvodplaylist>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostVodPlaylistParams {
    pub(crate) vod: OnlyKeyField,
    pub start_time: u64,
    pub end_time: u64,
}

impl PostVodPlaylistParams {
    pub fn new(start_time: u64, end_time: u64) -> Self {
        Self {
            vod: OnlyKeyField,
            start_time,
            end_time,
        }
    }
}

/// PostVodPlaylist operation. The URL path is `<channel>/<playlist>`; we wire
/// both segments into `object_key` joined by `/`.
pub struct PostVodPlaylist {
    pub channel_name: String,
    pub playlist_name: String,
    pub params: PostVodPlaylistParams,
}

impl Ops for PostVodPlaylist {
    type Response = EmptyResponseProcessor;
    type Body = ZeroBody;
    type Query = PostVodPlaylistParams;

    fn prepare(self) -> Result<Prepared<PostVodPlaylistParams>> {
        let key = format!("{}/{}", self.channel_name, self.playlist_name);
        Ok(Prepared {
            method: Method::POST,
            key: Some(key),
            query: Some(self.params),
            body: Some(()),
            ..Default::default()
        })
    }
}

/// Trait for PostVodPlaylist operations.
pub trait PostVodPlaylistOperations {
    fn post_vod_playlist(
        &self,
        channel_name: impl Into<String>,
        playlist_name: impl Into<String>,
        start_time: u64,
        end_time: u64,
    ) -> impl Future<Output = Result<()>>;
}

impl PostVodPlaylistOperations for Client {
    async fn post_vod_playlist(
        &self,
        channel_name: impl Into<String>,
        playlist_name: impl Into<String>,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        let ops = PostVodPlaylist {
            channel_name: channel_name.into(),
            playlist_name: playlist_name.into(),
            params: PostVodPlaylistParams::new(start_time, end_time),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params() {
        let q = crate::ser::to_string(&PostVodPlaylistParams::new(1700000000, 1700001000)).unwrap();
        assert_eq!(q, "endTime=1700001000&startTime=1700000000&vod");
    }

    #[test]
    fn test_prepare_key_joined() {
        let p = PostVodPlaylist {
            channel_name: "chan".into(),
            playlist_name: "play.m3u8".into(),
            params: PostVodPlaylistParams::new(1, 2),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::POST);
        assert_eq!(p.key.as_deref(), Some("chan/play.m3u8"));
    }
}

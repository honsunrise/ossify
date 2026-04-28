//! LiveChannel APIs (RTMP ingest + HLS playback).
//!
//! Official documentation index:
//! <https://www.alibabacloud.com/help/en/oss/developer-reference/list-of-operations-by-function>

mod delete_live_channel;
mod get_live_channel_history;
mod get_live_channel_info;
mod get_live_channel_stat;
mod get_vod_playlist;
mod list_live_channel;
mod post_vod_playlist;
mod put_live_channel;
mod put_live_channel_status;

pub use delete_live_channel::*;
pub use get_live_channel_history::*;
pub use get_live_channel_info::*;
pub use get_live_channel_stat::*;
pub use get_vod_playlist::*;
pub use list_live_channel::*;
pub use post_vod_playlist::*;
pub use put_live_channel::*;
pub use put_live_channel_status::*;

use crate::Client;

/// Aggregate supertrait that exposes every LiveChannel API on [`Client`].
pub trait BucketLiveChannelOperations:
    DeleteLiveChannelOperations
    + GetLiveChannelHistoryOperations
    + GetLiveChannelInfoOperations
    + GetLiveChannelStatOperations
    + GetVodPlaylistOperations
    + ListLiveChannelOperations
    + PostVodPlaylistOperations
    + PutLiveChannelOperations
    + PutLiveChannelStatusOperations
{
}

impl BucketLiveChannelOperations for Client {}

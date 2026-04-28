//! PutLiveChannelStatus: enable or disable a LiveChannel.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/putlivechannelstatus>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::ZeroBody;
use crate::error::Result;
use crate::ops::common::LiveChannelStatus;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// PutLiveChannelStatus query parameters: `?live&status=<enabled|disabled>`.
#[derive(Debug, Clone, Serialize)]
pub struct PutLiveChannelStatusParams {
    pub(crate) live: OnlyKeyField,
    pub status: String,
}

impl PutLiveChannelStatusParams {
    pub fn new(status: LiveChannelStatus) -> Self {
        Self {
            live: OnlyKeyField,
            status: status.as_str().to_string(),
        }
    }
}

/// PutLiveChannelStatus operation.
pub struct PutLiveChannelStatus {
    pub channel_name: String,
    pub params: PutLiveChannelStatusParams,
}

impl Ops for PutLiveChannelStatus {
    type Response = EmptyResponseProcessor;
    type Body = ZeroBody;
    type Query = PutLiveChannelStatusParams;

    fn prepare(self) -> Result<Prepared<PutLiveChannelStatusParams>> {
        Ok(Prepared {
            method: Method::PUT,
            key: Some(self.channel_name),
            query: Some(self.params),
            body: Some(()),
            ..Default::default()
        })
    }
}

/// Trait for PutLiveChannelStatus operations.
pub trait PutLiveChannelStatusOperations {
    fn put_live_channel_status(
        &self,
        channel_name: impl Into<String>,
        status: LiveChannelStatus,
    ) -> impl Future<Output = Result<()>>;
}

impl PutLiveChannelStatusOperations for Client {
    async fn put_live_channel_status(
        &self,
        channel_name: impl Into<String>,
        status: LiveChannelStatus,
    ) -> Result<()> {
        let ops = PutLiveChannelStatus {
            channel_name: channel_name.into(),
            params: PutLiveChannelStatusParams::new(status),
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params_enabled() {
        let q = crate::ser::to_string(&PutLiveChannelStatusParams::new(LiveChannelStatus::Enabled)).unwrap();
        assert_eq!(q, "live&status=enabled");
    }

    #[test]
    fn test_serialize_params_disabled() {
        let q = crate::ser::to_string(&PutLiveChannelStatusParams::new(LiveChannelStatus::Disabled)).unwrap();
        assert_eq!(q, "live&status=disabled");
    }
}

//! DeleteLiveChannel: remove a LiveChannel configuration.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/deletelivechannel>

use std::future::Future;

use http::Method;
use serde::Serialize;

use crate::body::NoneBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLiveChannelParams {
    pub(crate) live: OnlyKeyField,
}

pub struct DeleteLiveChannel {
    pub channel_name: String,
}

impl Ops for DeleteLiveChannel {
    type Response = EmptyResponseProcessor;
    type Body = NoneBody;
    type Query = DeleteLiveChannelParams;

    fn prepare(self) -> Result<Prepared<DeleteLiveChannelParams>> {
        Ok(Prepared {
            method: Method::DELETE,
            key: Some(self.channel_name),
            query: Some(DeleteLiveChannelParams::default()),
            ..Default::default()
        })
    }
}

/// Trait for DeleteLiveChannel operations.
pub trait DeleteLiveChannelOperations {
    fn delete_live_channel(&self, channel_name: impl Into<String>) -> impl Future<Output = Result<()>>;
}

impl DeleteLiveChannelOperations for Client {
    async fn delete_live_channel(&self, channel_name: impl Into<String>) -> Result<()> {
        let ops = DeleteLiveChannel {
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
        assert_eq!(crate::ser::to_string(&DeleteLiveChannelParams::default()).unwrap(), "live");
    }

    #[test]
    fn test_prepare_method() {
        let p = DeleteLiveChannel {
            channel_name: "abc".into(),
        }
        .prepare()
        .unwrap();
        assert_eq!(p.method, Method::DELETE);
        assert_eq!(p.key.as_deref(), Some("abc"));
    }
}

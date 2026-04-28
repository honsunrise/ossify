//! ListResourcePoolRequesterQoSInfos: list all per-requester QoS entries for
//! a resource pool.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolrequesterqosinfos>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::NoneBody;
use crate::error::Result;
use crate::ops::common::RequesterQoSInfo;
use crate::response::BodyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListResourcePoolRequesterQoSInfosParams {
    #[serde(rename = "requesterQosInfo")]
    pub(crate) requester_qos_info: OnlyKeyField,
    #[serde(rename = "resourcePool")]
    pub resource_pool: String,

    pub continuation_token: Option<String>,
    pub max_keys: Option<u32>,
}

impl ListResourcePoolRequesterQoSInfosParams {
    pub fn new(resource_pool: impl Into<String>) -> Self {
        Self {
            requester_qos_info: OnlyKeyField,
            resource_pool: resource_pool.into(),
            continuation_token: None,
            max_keys: None,
        }
    }

    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.continuation_token = Some(token.into());
        self
    }

    pub fn max_keys(mut self, max_keys: u32) -> Self {
        self.max_keys = Some(max_keys);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListResourcePoolRequesterQoSInfosResult {
    pub resource_pool: Option<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
    #[serde(default)]
    pub is_truncated: bool,
    #[serde(default, rename = "RequesterQoSInfo")]
    pub requester_qos_infos: Vec<RequesterQoSInfo>,
}

pub struct ListResourcePoolRequesterQoSInfos {
    pub params: ListResourcePoolRequesterQoSInfosParams,
}

impl Ops for ListResourcePoolRequesterQoSInfos {
    type Response = BodyResponseProcessor<ListResourcePoolRequesterQoSInfosResult>;
    type Body = NoneBody;
    type Query = ListResourcePoolRequesterQoSInfosParams;

    const USE_BUCKET: bool = false;

    fn prepare(self) -> Result<Prepared<ListResourcePoolRequesterQoSInfosParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListResourcePoolRequesterQoSInfosOps {
    /// List all per-requester QoS entries for the resource pool.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listresourcepoolrequesterqosinfos>
    fn list_resource_pool_requester_qos_infos(
        &self,
        params: ListResourcePoolRequesterQoSInfosParams,
    ) -> impl Future<Output = Result<ListResourcePoolRequesterQoSInfosResult>>;
}

impl ListResourcePoolRequesterQoSInfosOps for Client {
    async fn list_resource_pool_requester_qos_infos(
        &self,
        params: ListResourcePoolRequesterQoSInfosParams,
    ) -> Result<ListResourcePoolRequesterQoSInfosResult> {
        self.request(ListResourcePoolRequesterQoSInfos { params }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_defaults() {
        let q = crate::ser::to_string(&ListResourcePoolRequesterQoSInfosParams::new("rp-for-ai")).unwrap();
        assert_eq!(q, "requesterQosInfo&resourcePool=rp-for-ai");
    }

    #[test]
    fn params_serialize_with_pagination() {
        let q = crate::ser::to_string(
            &ListResourcePoolRequesterQoSInfosParams::new("rp-for-ai")
                .continuation_token("abc")
                .max_keys(10),
        )
        .unwrap();
        assert_eq!(q, "continuation-token=abc&max-keys=10&requesterQosInfo&resourcePool=rp-for-ai");
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListResourcePoolRequesterQoSInfosResult>
  <ResourcePool>rp-for-ai</ResourcePool>
  <IsTruncated>false</IsTruncated>
  <RequesterQoSInfo>
    <Requester>311xxxx</Requester>
    <QoSConfiguration>
      <TotalUploadBandwidth>10</TotalUploadBandwidth>
      <IntranetUploadBandwidth>-1</IntranetUploadBandwidth>
      <ExtranetUploadBandwidth>-1</ExtranetUploadBandwidth>
      <TotalDownloadBandwidth>10</TotalDownloadBandwidth>
      <IntranetDownloadBandwidth>-1</IntranetDownloadBandwidth>
      <ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>
    </QoSConfiguration>
  </RequesterQoSInfo>
</ListResourcePoolRequesterQoSInfosResult>"#;
        let parsed: ListResourcePoolRequesterQoSInfosResult = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.resource_pool.as_deref(), Some("rp-for-ai"));
        assert_eq!(parsed.requester_qos_infos.len(), 1);
        assert_eq!(parsed.requester_qos_infos[0].requester, "311xxxx");
    }
}

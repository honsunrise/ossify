//! ListBucketRequesterQoSInfos: list all per-requester QoS configurations
//! for the current bucket.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketrequesterqosinfos>

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

/// Query parameters for [`ListBucketRequesterQoSInfos`].
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListBucketRequesterQoSInfosParams {
    #[serde(rename = "requesterQosInfo")]
    requester_qos_info: OnlyKeyField,

    /// Pagination token returned by a previous request in
    /// `NextContinuationToken`.
    pub continuation_token: Option<String>,

    /// Maximum number of entries to return.
    pub max_keys: Option<u32>,
}

impl ListBucketRequesterQoSInfosParams {
    pub fn new() -> Self {
        Self::default()
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

/// Response body for [`ListBucketRequesterQoSInfos`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketRequesterQoSInfosResult {
    pub bucket: Option<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
    #[serde(default)]
    pub is_truncated: bool,
    #[serde(default, rename = "RequesterQoSInfo")]
    pub requester_qos_infos: Vec<RequesterQoSInfo>,
}

/// The `ListBucketRequesterQoSInfos` operation.
pub struct ListBucketRequesterQoSInfos {
    pub params: ListBucketRequesterQoSInfosParams,
}

impl Ops for ListBucketRequesterQoSInfos {
    type Response = BodyResponseProcessor<ListBucketRequesterQoSInfosResult>;
    type Body = NoneBody;
    type Query = ListBucketRequesterQoSInfosParams;

    fn prepare(self) -> Result<Prepared<ListBucketRequesterQoSInfosParams>> {
        Ok(Prepared {
            method: Method::GET,
            query: Some(self.params),
            ..Default::default()
        })
    }
}

pub trait ListBucketRequesterQoSInfosOps {
    /// List all per-requester QoS configurations for the current bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/listbucketrequesterqosinfos>
    fn list_bucket_requester_qos_infos(
        &self,
        params: Option<ListBucketRequesterQoSInfosParams>,
    ) -> impl Future<Output = Result<ListBucketRequesterQoSInfosResult>>;
}

impl ListBucketRequesterQoSInfosOps for Client {
    async fn list_bucket_requester_qos_infos(
        &self,
        params: Option<ListBucketRequesterQoSInfosParams>,
    ) -> Result<ListBucketRequesterQoSInfosResult> {
        self.request(ListBucketRequesterQoSInfos {
            params: params.unwrap_or_default(),
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize_defaults() {
        let q = crate::ser::to_string(&ListBucketRequesterQoSInfosParams::new()).unwrap();
        assert_eq!(q, "requesterQosInfo");
    }

    #[test]
    fn params_serialize_with_pagination() {
        let q = crate::ser::to_string(
            &ListBucketRequesterQoSInfosParams::new()
                .continuation_token("abc")
                .max_keys(10),
        )
        .unwrap();
        assert_eq!(q, "continuation-token=abc&max-keys=10&requesterQosInfo");
    }

    #[test]
    fn parse_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketRequesterQoSInfosResult>
  <Bucket>oss-example</Bucket>
  <ContinuationToken>abc</ContinuationToken>
  <NextContinuationToken>def</NextContinuationToken>
  <IsTruncated>true</IsTruncated>
  <RequesterQoSInfo>
    <Requester>266xxxx</Requester>
    <QoSConfiguration>
      <TotalUploadBandwidth>10</TotalUploadBandwidth>
      <IntranetUploadBandwidth>-1</IntranetUploadBandwidth>
      <ExtranetUploadBandwidth>-1</ExtranetUploadBandwidth>
      <TotalDownloadBandwidth>10</TotalDownloadBandwidth>
      <IntranetDownloadBandwidth>-1</IntranetDownloadBandwidth>
      <ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>
    </QoSConfiguration>
  </RequesterQoSInfo>
  <RequesterQoSInfo>
    <Requester>267xxxx</Requester>
    <QoSConfiguration>
      <TotalUploadBandwidth>20</TotalUploadBandwidth>
      <IntranetUploadBandwidth>-1</IntranetUploadBandwidth>
      <ExtranetUploadBandwidth>-1</ExtranetUploadBandwidth>
      <TotalDownloadBandwidth>20</TotalDownloadBandwidth>
      <IntranetDownloadBandwidth>-1</IntranetDownloadBandwidth>
      <ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>
    </QoSConfiguration>
  </RequesterQoSInfo>
</ListBucketRequesterQoSInfosResult>"#;
        let parsed: ListBucketRequesterQoSInfosResult = quick_xml::de::from_str(xml).unwrap();
        assert!(parsed.is_truncated);
        assert_eq!(parsed.bucket.as_deref(), Some("oss-example"));
        assert_eq!(parsed.next_continuation_token.as_deref(), Some("def"));
        assert_eq!(parsed.requester_qos_infos.len(), 2);
        assert_eq!(parsed.requester_qos_infos[0].requester, "266xxxx");
        assert_eq!(
            parsed.requester_qos_infos[1]
                .qos_configuration
                .total_upload_bandwidth,
            20
        );
    }
}

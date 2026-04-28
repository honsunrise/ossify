//! RestoreObject operation.
//!
//! Restores an Archive, Cold Archive, or Deep Cold Archive object so that it
//! can be read. The request body is optional: if omitted, the default
//! restored state duration is used. For Cold Archive / Deep Cold Archive a
//! `JobParameters.Tier` can be supplied.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/restoreobject>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::HeaderResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Restoration tier (Cold Archive / Deep Cold Archive only).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RestoreTier {
    /// Expedited restoration.
    Expedited,
    /// Standard restoration (default for Cold/Deep Cold Archive).
    Standard,
    /// Bulk restoration (Cold Archive only).
    Bulk,
}

/// `<JobParameters>` element of a `RestoreRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "JobParameters", rename_all = "PascalCase")]
pub struct RestoreJobParameters {
    pub tier: RestoreTier,
}

/// `<RestoreRequest>` body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "RestoreRequest", rename_all = "PascalCase")]
pub struct RestoreRequest {
    /// Number of days for the object to remain restored.
    pub days: u32,
    /// Restoration priority (Cold Archive / Deep Cold Archive only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_parameters: Option<RestoreJobParameters>,
}

impl RestoreRequest {
    pub fn new(days: u32) -> Self {
        Self {
            days,
            job_parameters: None,
        }
    }

    pub fn tier(mut self, tier: RestoreTier) -> Self {
        self.job_parameters = Some(RestoreJobParameters { tier });
        self
    }
}

/// RestoreObject query parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreObjectParams {
    pub(crate) restore: OnlyKeyField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl RestoreObjectParams {
    pub fn new() -> Self {
        Self {
            restore: OnlyKeyField,
            version_id: None,
        }
    }

    pub fn version_id(mut self, v: impl Into<String>) -> Self {
        self.version_id = Some(v.into());
        self
    }
}

impl Default for RestoreObjectParams {
    fn default() -> Self {
        Self::new()
    }
}

/// RestoreObject response headers.
#[derive(Debug, Clone, Deserialize)]
pub struct RestoreObjectResponse {
    /// Restoration priority (Cold Archive / Deep Cold Archive).
    #[serde(rename = "x-oss-object-restore-priority")]
    pub restore_priority: Option<String>,
    /// Version ID (versioned buckets).
    #[serde(rename = "x-oss-version-id")]
    pub version_id: Option<String>,
}

/// RestoreObject operation.
pub struct RestoreObject {
    pub object_key: String,
    pub params: RestoreObjectParams,
    pub body: Option<RestoreRequest>,
}

impl Ops for RestoreObject {
    type Response = HeaderResponseProcessor<RestoreObjectResponse>;
    type Body = XMLBody<RestoreRequest>;
    type Query = RestoreObjectParams;

    fn prepare(self) -> Result<Prepared<RestoreObjectParams, RestoreRequest>> {
        Ok(Prepared {
            method: Method::POST,
            key: Some(self.object_key),
            query: Some(self.params),
            body: self.body,
            ..Default::default()
        })
    }
}

/// Trait for RestoreObject operations.
pub trait RestoreObjectOperations {
    fn restore_object(
        &self,
        object_key: impl Into<String>,
        params: Option<RestoreObjectParams>,
        body: Option<RestoreRequest>,
    ) -> impl Future<Output = Result<RestoreObjectResponse>>;
}

impl RestoreObjectOperations for Client {
    async fn restore_object(
        &self,
        object_key: impl Into<String>,
        params: Option<RestoreObjectParams>,
        body: Option<RestoreRequest>,
    ) -> Result<RestoreObjectResponse> {
        let ops = RestoreObject {
            object_key: object_key.into(),
            params: params.unwrap_or_default(),
            body,
        };
        self.request(ops).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_params_with_version() {
        let q = crate::ser::to_string(&RestoreObjectParams::new().version_id("v1")).unwrap();
        assert_eq!(q, "restore&versionId=v1");
    }

    #[test]
    fn test_serialize_body_days_only() {
        let body = RestoreRequest::new(2);
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert_eq!(xml, "<RestoreRequest><Days>2</Days></RestoreRequest>");
    }

    #[test]
    fn test_serialize_body_with_tier() {
        let body = RestoreRequest::new(7).tier(RestoreTier::Bulk);
        let xml = quick_xml::se::to_string(&body).unwrap();
        assert!(xml.contains("<Days>7</Days>"));
        assert!(xml.contains("<JobParameters><Tier>Bulk</Tier></JobParameters>"));
    }

    #[test]
    fn test_deserialize_body_roundtrip() {
        let xml = r#"<RestoreRequest><Days>3</Days><JobParameters><Tier>Expedited</Tier></JobParameters></RestoreRequest>"#;
        let body: RestoreRequest = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(body.days, 3);
        assert_eq!(body.job_parameters.unwrap().tier, RestoreTier::Expedited);
    }
}

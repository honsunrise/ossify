//! OpenMetaQuery.
//!
//! Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/openmetaquery>

use std::future::Future;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::body::XMLBody;
use crate::error::Result;
use crate::response::EmptyResponseProcessor;
use crate::ser::OnlyKeyField;
use crate::{Client, Ops, Prepared, Request};

/// Retrieval mode for the metadata-index library.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetaQueryMode {
    /// Scalar search (default).
    #[default]
    Basic,
    /// Semantic (vector) search.
    Semantic,
}

impl MetaQueryMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetaQueryMode::Basic => "basic",
            MetaQueryMode::Semantic => "semantic",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenMetaQueryParams {
    pub mode: MetaQueryMode,
    pub comp: String,
    meta_query: OnlyKeyField,
}

impl Default for OpenMetaQueryParams {
    fn default() -> Self {
        Self {
            mode: MetaQueryMode::Basic,
            comp: "add".to_string(),
            meta_query: OnlyKeyField,
        }
    }
}

/// `<WorkflowParameter>` entry for AI content awareness options.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorkflowParameter {
    pub name: String,
    pub value: String,
}

/// `<WorkflowParameters>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowParameters {
    #[serde(rename = "WorkflowParameter", default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<WorkflowParameter>,
}

/// `<Filters>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaQueryFilters {
    #[serde(rename = "Filter", default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<String>,
}

/// `<Notification>` entry.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetaQueryNotification {
    #[serde(rename = "MNS")]
    pub mns: Option<String>,
}

/// `<Notifications>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaQueryNotifications {
    #[serde(rename = "Notification", default, skip_serializing_if = "Vec::is_empty")]
    pub notifications: Vec<MetaQueryNotification>,
}

/// `<WithFields>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetaQueryWithFields {
    #[serde(rename = "WithField", default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<String>,
}

/// `<NotificationAttributes>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetaQueryNotificationAttributes {
    pub notifications: Option<MetaQueryNotifications>,
    pub with_fields: Option<MetaQueryWithFields>,
}

/// Root `<MetaQuery>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MetaQuery", rename_all = "PascalCase")]
pub struct OpenMetaQueryConfig {
    pub workflow_parameters: Option<WorkflowParameters>,
    pub filters: Option<MetaQueryFilters>,
    pub notification_attributes: Option<MetaQueryNotificationAttributes>,
}

pub struct OpenMetaQuery {
    pub mode: MetaQueryMode,
    pub config: OpenMetaQueryConfig,
}

impl Ops for OpenMetaQuery {
    type Response = EmptyResponseProcessor;
    type Body = XMLBody<OpenMetaQueryConfig>;
    type Query = OpenMetaQueryParams;

    fn prepare(self) -> Result<Prepared<OpenMetaQueryParams, OpenMetaQueryConfig>> {
        Ok(Prepared {
            method: Method::POST,
            query: Some(OpenMetaQueryParams {
                mode: self.mode,
                comp: "add".to_string(),
                meta_query: OnlyKeyField,
            }),
            body: Some(self.config),
            ..Default::default()
        })
    }
}

pub trait OpenMetaQueryOps {
    /// Enable the metadata-index library on the bucket.
    ///
    /// Official document: <https://www.alibabacloud.com/help/en/oss/developer-reference/openmetaquery>
    fn open_meta_query(
        &self,
        mode: MetaQueryMode,
        config: OpenMetaQueryConfig,
    ) -> impl Future<Output = Result<()>>;
}

impl OpenMetaQueryOps for Client {
    async fn open_meta_query(&self, mode: MetaQueryMode, config: OpenMetaQueryConfig) -> Result<()> {
        self.request(OpenMetaQuery { mode, config }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_serialize() {
        let q = crate::ser::to_string(&OpenMetaQueryParams::default()).unwrap();
        assert_eq!(q, "comp=add&metaQuery&mode=basic");
    }

    #[test]
    fn body_round_trip() {
        let cfg = OpenMetaQueryConfig {
            workflow_parameters: Some(WorkflowParameters {
                parameters: vec![WorkflowParameter {
                    name: "VideoInsightEnable".to_string(),
                    value: "True".to_string(),
                }],
            }),
            filters: Some(MetaQueryFilters {
                filters: vec!["Size > 1024".to_string()],
            }),
            notification_attributes: Some(MetaQueryNotificationAttributes {
                notifications: Some(MetaQueryNotifications {
                    notifications: vec![MetaQueryNotification {
                        mns: Some("topic".to_string()),
                    }],
                }),
                with_fields: Some(MetaQueryWithFields {
                    fields: vec!["Insights".to_string()],
                }),
            }),
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<MetaQuery>"));
        assert!(xml.contains("<Filter>Size &gt; 1024</Filter>"));
        let back: OpenMetaQueryConfig = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}

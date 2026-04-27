//! Shared types for Object FC Access Point APIs.

use serde::{Deserialize, Serialize};

use super::access_point::AccessPointStatus;

/// `<Actions><Action>GetObject</Action></Actions>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Actions")]
pub struct ObjectFcActions {
    #[serde(rename = "Action", default)]
    pub actions: Vec<String>,
}

/// `<FunctionCompute>` block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "FunctionCompute", rename_all = "PascalCase")]
pub struct ObjectFcFunctionCompute {
    #[serde(rename = "FunctionAssumeRoleArn")]
    pub function_assume_role_arn: String,
    #[serde(rename = "FunctionArn")]
    pub function_arn: String,
}

/// `<ContentTransformation>` block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ContentTransformation", rename_all = "PascalCase")]
pub struct ObjectFcContentTransformation {
    pub function_compute: ObjectFcFunctionCompute,
}

/// `<TransformationConfiguration>` entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TransformationConfiguration", rename_all = "PascalCase")]
pub struct ObjectFcTransformationConfiguration {
    pub actions: ObjectFcActions,
    pub content_transformation: ObjectFcContentTransformation,
}

/// `<TransformationConfigurations>` wrapper.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "TransformationConfigurations")]
pub struct ObjectFcTransformationConfigurations {
    #[serde(rename = "TransformationConfiguration", default)]
    pub configurations: Vec<ObjectFcTransformationConfiguration>,
}

/// `<AllowedFeatures>` wrapper.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AllowedFeatures")]
pub struct ObjectFcAllowedFeatures {
    #[serde(rename = "AllowedFeature", default)]
    pub features: Vec<String>,
}

/// `<ObjectProcessConfiguration>` block.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ObjectProcessConfiguration", rename_all = "PascalCase")]
pub struct ObjectProcessConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_features: Option<ObjectFcAllowedFeatures>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transformation_configurations: Option<ObjectFcTransformationConfigurations>,
}

/// Endpoints attached to an Object FC access point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Endpoints", rename_all = "PascalCase")]
pub struct ObjectFcEndpoints {
    pub public_endpoint: String,
    pub internal_endpoint: String,
}

pub use super::access_point::AccessPointStatus as ObjectFcAccessPointStatus;

/// Summary of an Object FC access point (as returned by List*).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AccessPointForObjectProcess", rename_all = "PascalCase")]
pub struct AccessPointForObjectProcessSummary {
    pub access_point_name_for_object_process: String,
    pub access_point_for_object_process_alias: String,
    pub access_point_name: String,
    pub status: AccessPointStatus,
}

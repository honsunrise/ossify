//! Shared types for OSS Access Point APIs.

use serde::{Deserialize, Serialize};

/// Network origin of an access point.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPointNetworkOrigin {
    /// The access point is reachable through a specific VPC.
    #[serde(rename = "vpc")]
    Vpc,
    /// The access point is reachable through public/internal endpoints.
    #[serde(rename = "internet")]
    Internet,
}

impl AccessPointNetworkOrigin {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessPointNetworkOrigin::Vpc => "vpc",
            AccessPointNetworkOrigin::Internet => "internet",
        }
    }
}

impl AsRef<str> for AccessPointNetworkOrigin {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Lifecycle status of an access point.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPointStatus {
    #[serde(rename = "enable")]
    Enable,
    #[serde(rename = "disable")]
    Disable,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "deleting")]
    Deleting,
}

impl AccessPointStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessPointStatus::Enable => "enable",
            AccessPointStatus::Disable => "disable",
            AccessPointStatus::Creating => "creating",
            AccessPointStatus::Deleting => "deleting",
        }
    }
}

impl AsRef<str> for AccessPointStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// `<VpcConfiguration><VpcId>…</VpcId></VpcConfiguration>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "VpcConfiguration", rename_all = "PascalCase")]
pub struct VpcConfiguration {
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

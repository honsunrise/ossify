//! AntiDDosStatus — the status of an OSS Anti-DDoS instance.

use serde::{Deserialize, Serialize};

/// Status of an Anti-DDoS instance.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntiDdosStatus {
    /// Newly created instance; domains must be specified before defending.
    Init,
    /// The instance is actively protecting traffic.
    Defending,
    /// Protection is paused.
    HaltDefending,
}

impl AntiDdosStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AntiDdosStatus::Init => "Init",
            AntiDdosStatus::Defending => "Defending",
            AntiDdosStatus::HaltDefending => "HaltDefending",
        }
    }
}

impl AsRef<str> for AntiDdosStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Type of an Anti-DDoS instance. OSS currently only supports
/// `AntiDDosPremimum` (yes, the official spelling has a typo).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntiDdosType {
    #[serde(rename = "AntiDDosPremimum")]
    AntiDdosPremimum,
}

impl AntiDdosType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AntiDdosType::AntiDdosPremimum => "AntiDDosPremimum",
        }
    }
}

impl AsRef<str> for AntiDdosType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// List of domain names attached to an Anti-DDoS instance. XML shape:
/// `<Cnames><Domain>abc1.example.cn</Domain>...</Cnames>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Cnames")]
pub struct AntiDdosCnames {
    #[serde(rename = "Domain", default)]
    pub domains: Vec<String>,
}

/// One `<AntiDDOSConfiguration>` entry inside a list/response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AntiDDOSConfiguration", rename_all = "PascalCase")]
pub struct AntiDdosConfiguration {
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AntiDdosStatus>,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<AntiDdosType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnames: Option<AntiDdosCnames>,
}

/// `<AntiDDOSListConfiguration>` — list-style response element shared by
/// `GetUserAntiDDosInfo` and `ListBucketAntiDDosInfo`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "AntiDDOSListConfiguration", rename_all = "PascalCase")]
pub struct AntiDdosListConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "AntiDDOSConfiguration", default)]
    pub configurations: Vec<AntiDdosConfiguration>,
}

//! Shared types for LiveChannel APIs.

use serde::{Deserialize, Serialize};

/// LiveChannel status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiveChannelStatus {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}

impl LiveChannelStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LiveChannelStatus::Enabled => "enabled",
            LiveChannelStatus::Disabled => "disabled",
        }
    }
}

impl AsRef<str> for LiveChannelStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// The target segment format (`<Target>` element). Currently OSS only supports
/// `HLS`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "Target", rename_all = "PascalCase")]
pub struct LiveChannelTarget {
    /// Target format. Must be `HLS`.
    #[serde(rename = "Type")]
    pub target_type: String,
    /// Duration of each HLS fragment in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frag_duration: Option<u32>,
    /// Number of fragments in the playlist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frag_count: Option<u32>,
    /// Name of the generated playlist file (default: `playlist.m3u8`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_name: Option<String>,
}

impl LiveChannelTarget {
    pub fn hls() -> Self {
        Self {
            target_type: "HLS".into(),
            frag_duration: None,
            frag_count: None,
            playlist_name: None,
        }
    }

    pub fn frag_duration(mut self, v: u32) -> Self {
        self.frag_duration = Some(v);
        self
    }

    pub fn frag_count(mut self, v: u32) -> Self {
        self.frag_count = Some(v);
        self
    }

    pub fn playlist_name(mut self, name: impl Into<String>) -> Self {
        self.playlist_name = Some(name.into());
        self
    }
}

/// Snapshot configuration (optional part of `<LiveChannelConfiguration>`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "Snapshot", rename_all = "PascalCase")]
pub struct LiveChannelSnapshot {
    pub role_name: String,
    pub dest_bucket: String,
    pub notify_topic: String,
    pub interval: u32,
}

/// `<LiveChannelConfiguration>` root used by PUT / GET Info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "LiveChannelConfiguration", rename_all = "PascalCase")]
pub struct LiveChannelConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LiveChannelStatus>,
    pub target: LiveChannelTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<LiveChannelSnapshot>,
}

impl LiveChannelConfiguration {
    pub fn new(target: LiveChannelTarget) -> Self {
        Self {
            description: None,
            status: None,
            target,
            snapshot: None,
        }
    }

    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }

    pub fn status(mut self, v: LiveChannelStatus) -> Self {
        self.status = Some(v);
        self
    }

    pub fn snapshot(mut self, v: LiveChannelSnapshot) -> Self {
        self.snapshot = Some(v);
        self
    }
}

/// Wrapper for `<PublishUrls>` / `<PlayUrls>` elements which both contain one
/// or more `<Url>` children.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveChannelUrls {
    #[serde(rename = "Url", default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<String>,
}

/// Video metrics reported by `GetLiveChannelStat`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename = "Video", rename_all = "PascalCase")]
pub struct LiveChannelVideoStat {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<u32>,
    pub bandwidth: Option<u64>,
    pub codec: Option<String>,
}

/// Audio metrics reported by `GetLiveChannelStat`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename = "Audio", rename_all = "PascalCase")]
pub struct LiveChannelAudioStat {
    pub sample_rate: Option<u32>,
    pub bandwidth: Option<u64>,
    pub codec: Option<String>,
}

/// A single record in `<LiveChannelHistory>`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename = "LiveRecord", rename_all = "PascalCase")]
pub struct LiveChannelHistoryRecord {
    pub start_time: String,
    pub end_time: String,
    pub remote_addr: String,
}

/// Summary record returned by `ListLiveChannel`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename = "LiveChannel", rename_all = "PascalCase")]
pub struct LiveChannelSummary {
    pub name: String,
    pub description: String,
    pub status: LiveChannelStatus,
    pub last_modified: String,
    pub publish_urls: LiveChannelUrls,
    pub play_urls: LiveChannelUrls,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_roundtrip() {
        let cfg = LiveChannelConfiguration::new(LiveChannelTarget::hls().frag_duration(5).frag_count(3))
            .description("demo")
            .status(LiveChannelStatus::Enabled);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<Description>demo</Description>"));
        assert!(xml.contains("<Status>enabled</Status>"));
        assert!(xml.contains("<Type>HLS</Type>"));
        let back: LiveChannelConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn test_list_summary_deserialize() {
        let xml = r#"<LiveChannel>
  <Name>abc</Name>
  <Description>d</Description>
  <Status>enabled</Status>
  <LastModified>2024-01-01T00:00:00Z</LastModified>
  <PublishUrls><Url>rtmp://x/abc</Url></PublishUrls>
  <PlayUrls><Url>http://x/abc.m3u8</Url></PlayUrls>
</LiveChannel>"#;
        let parsed: LiveChannelSummary = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.name, "abc");
        assert_eq!(parsed.status, LiveChannelStatus::Enabled);
        assert_eq!(parsed.publish_urls.urls[0], "rtmp://x/abc");
    }
}

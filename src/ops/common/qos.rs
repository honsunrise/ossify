//! Shared `<QoSConfiguration>` XML type used by Bucket QoS, Requester QoS,
//! and Resource Pool QoS operations.
//!
//! The base configuration carries six bandwidth fields (Gbit/s, `-1` meaning
//! "no limit", `0` meaning "disabled"). Resource-pool-level requester QoS
//! additionally supports three QPS fields; those are represented as
//! `Option<i64>` so the same struct round-trips both layouts.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Root `<QoSConfiguration>` element used by every QoS API.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "QoSConfiguration", rename_all = "PascalCase")]
pub struct QoSConfiguration {
    /// Total upload bandwidth cap, Gbit/s. `-1` = no limit, `0` = disabled.
    pub total_upload_bandwidth: i64,
    /// Intranet upload bandwidth cap, Gbit/s.
    pub intranet_upload_bandwidth: i64,
    /// Extranet (public internet) upload bandwidth cap, Gbit/s.
    pub extranet_upload_bandwidth: i64,
    /// Total download bandwidth cap, Gbit/s.
    pub total_download_bandwidth: i64,
    /// Intranet download bandwidth cap, Gbit/s.
    pub intranet_download_bandwidth: i64,
    /// Extranet download bandwidth cap, Gbit/s.
    pub extranet_download_bandwidth: i64,

    /// Total QPS cap. Only used for resource-pool-level requester QoS; left
    /// unset for bucket-level operations.
    pub total_qps: Option<i64>,
    /// Intranet QPS cap. Only used for resource-pool-level requester QoS.
    pub intranet_qps: Option<i64>,
    /// Extranet QPS cap. Only used for resource-pool-level requester QoS.
    pub extranet_qps: Option<i64>,
}

impl QoSConfiguration {
    /// Build a bandwidth-only configuration (used by Bucket QoS, bucket-level
    /// Requester QoS, and Bucket-Group QoS).
    pub fn bandwidth(
        total_upload: i64,
        intranet_upload: i64,
        extranet_upload: i64,
        total_download: i64,
        intranet_download: i64,
        extranet_download: i64,
    ) -> Self {
        Self {
            total_upload_bandwidth: total_upload,
            intranet_upload_bandwidth: intranet_upload,
            extranet_upload_bandwidth: extranet_upload,
            total_download_bandwidth: total_download,
            intranet_download_bandwidth: intranet_download,
            extranet_download_bandwidth: extranet_download,
            total_qps: None,
            intranet_qps: None,
            extranet_qps: None,
        }
    }

    /// Attach QPS caps (resource-pool-level requester QoS only).
    pub fn with_qps(mut self, total: i64, intranet: i64, extranet: i64) -> Self {
        self.total_qps = Some(total);
        self.intranet_qps = Some(intranet);
        self.extranet_qps = Some(extranet);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bandwidth_only_round_trip() {
        let cfg = QoSConfiguration::bandwidth(10, -1, -1, 10, -1, -1);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<TotalUploadBandwidth>10</TotalUploadBandwidth>"));
        assert!(xml.contains("<ExtranetDownloadBandwidth>-1</ExtranetDownloadBandwidth>"));
        // QPS fields must be skipped when unset.
        assert!(!xml.contains("TotalQps"));
        let back: QoSConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn with_qps_round_trip() {
        let cfg = QoSConfiguration::bandwidth(10, -1, -1, 10, -1, -1).with_qps(-1, -1, -1);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<TotalQps>-1</TotalQps>"));
        let back: QoSConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back.total_qps, Some(-1));
        assert_eq!(back.intranet_qps, Some(-1));
        assert_eq!(back.extranet_qps, Some(-1));
    }
}

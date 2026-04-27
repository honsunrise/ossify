use serde::{Deserialize, Serialize};

/// OSS object storage class. Used by `PutBucket`, `PutObject`, `CopyObject`,
/// `InitiateMultipartUpload`, lifecycle rules, inventory, and more.
///
/// See <https://www.alibabacloud.com/help/en/oss/user-guide/overview-of-storage-classes>.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StorageClass {
    #[default]
    Standard,
    #[serde(rename = "IA")]
    InfrequentAccess,
    Archive,
    ColdArchive,
    DeepColdArchive,
}

impl StorageClass {
    /// The wire form used by OSS (what appears in XML and `x-oss-storage-class`
    /// headers).
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageClass::Standard => "Standard",
            StorageClass::InfrequentAccess => "IA",
            StorageClass::Archive => "Archive",
            StorageClass::ColdArchive => "ColdArchive",
            StorageClass::DeepColdArchive => "DeepColdArchive",
        }
    }
}

impl AsRef<str> for StorageClass {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Bucket data redundancy type. Used by `PutBucket` and the data-redundancy
/// transition APIs.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DataRedundancyType {
    /// Locally Redundant Storage.
    #[default]
    #[serde(rename = "LRS")]
    LocallyRedundantStorage,
    /// Zone Redundant Storage.
    #[serde(rename = "ZRS")]
    ZoneRedundantStorage,
}

impl DataRedundancyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataRedundancyType::LocallyRedundantStorage => "LRS",
            DataRedundancyType::ZoneRedundantStorage => "ZRS",
        }
    }
}

impl AsRef<str> for DataRedundancyType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_class_wire_names_round_trip() {
        for sc in [
            StorageClass::Standard,
            StorageClass::InfrequentAccess,
            StorageClass::Archive,
            StorageClass::ColdArchive,
            StorageClass::DeepColdArchive,
        ] {
            let json = serde_json::to_string(&sc).unwrap();
            let back: StorageClass = serde_json::from_str(&json).unwrap();
            assert_eq!(sc, back);
        }
    }

    #[test]
    fn storage_class_as_str_matches_wire() {
        assert_eq!(StorageClass::Standard.as_str(), "Standard");
        assert_eq!(StorageClass::InfrequentAccess.as_str(), "IA");
        assert_eq!(StorageClass::Archive.as_str(), "Archive");
        assert_eq!(StorageClass::ColdArchive.as_str(), "ColdArchive");
        assert_eq!(StorageClass::DeepColdArchive.as_str(), "DeepColdArchive");
    }

    #[test]
    fn data_redundancy_type_wire_names_round_trip() {
        let lrs = DataRedundancyType::LocallyRedundantStorage;
        let json = serde_json::to_string(&lrs).unwrap();
        assert_eq!(json, "\"LRS\"");
        let back: DataRedundancyType = serde_json::from_str(&json).unwrap();
        assert_eq!(lrs, back);

        let zrs = DataRedundancyType::ZoneRedundantStorage;
        let json = serde_json::to_string(&zrs).unwrap();
        assert_eq!(json, "\"ZRS\"");
        let back: DataRedundancyType = serde_json::from_str(&json).unwrap();
        assert_eq!(zrs, back);
    }
}

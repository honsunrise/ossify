//! Inventory rule types shared by `PutBucketInventory`, `GetBucketInventory`
//! and `ListBucketInventory`.
//!
//! See <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketinventory>.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Valid values for `<Format>` inside `<OSSBucketDestination>`. Only `CSV`
/// is currently accepted by OSS.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum InventoryFormat {
    #[default]
    #[serde(rename = "CSV")]
    Csv,
}

/// Valid values for `<Schedule><Frequency>`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum InventoryFrequency {
    #[default]
    Daily,
    Weekly,
}

/// Valid values for `<IncludedObjectVersions>`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum IncludedObjectVersions {
    #[default]
    All,
    Current,
}

/// Optional fields that OSS can include in the generated inventory CSV.
///
/// Uses a manual `Deserialize` impl because quick-xml 0.39 tries to interpret
/// the XML element tag (`Field`) as the variant name when the enum is the
/// text content of an element; we instead deserialize a `String` and map it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum InventoryOptionalField {
    Size,
    LastModifiedDate,
    TransitionTime,
    #[serde(rename = "ETag")]
    ETag,
    StorageClass,
    IsMultipartUploaded,
    EncryptionStatus,
    ObjectAcl,
    TaggingCount,
    ObjectType,
    Crc64,
    // Incremental-inventory-only fields:
    SequenceNumber,
    RecordType,
    RecordTimestamp,
    Requester,
    RequestId,
    SourceIp,
    Key,
    VersionId,
    IsDeleteMarker,
    /// Unknown / forward-compatible value. The `String` is the exact wire
    /// name OSS returned.
    Other(String),
}

impl<'de> Deserialize<'de> for InventoryOptionalField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Size" => Self::Size,
            "LastModifiedDate" => Self::LastModifiedDate,
            // Docs intentionally use the misspelling "TransistionTime" in
            // places; accept both.
            "TransitionTime" | "TransistionTime" => Self::TransitionTime,
            "ETag" => Self::ETag,
            "StorageClass" => Self::StorageClass,
            "IsMultipartUploaded" => Self::IsMultipartUploaded,
            "EncryptionStatus" => Self::EncryptionStatus,
            "ObjectAcl" => Self::ObjectAcl,
            "TaggingCount" => Self::TaggingCount,
            "ObjectType" => Self::ObjectType,
            "Crc64" | "CRC64" => Self::Crc64,
            "SequenceNumber" => Self::SequenceNumber,
            "RecordType" => Self::RecordType,
            "RecordTimestamp" => Self::RecordTimestamp,
            "Requester" => Self::Requester,
            "RequestId" => Self::RequestId,
            "SourceIp" => Self::SourceIp,
            "Key" => Self::Key,
            "VersionId" => Self::VersionId,
            "IsDeleteMarker" => Self::IsDeleteMarker,
            _ => Self::Other(s),
        })
    }
}

/// `<OptionalFields>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OptionalFields {
    #[serde(rename = "Field", default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<InventoryOptionalField>,
}

/// `<SSE-OSS/>` element (empty container).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SseOssEncryption {}

/// `<SSE-KMS>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SseKmsInventoryEncryption {
    pub key_id: Option<String>,
}

/// `<Encryption>` container. Exactly one of SSE-OSS or SSE-KMS is set in
/// practice; both are modelled as optional so the struct can also represent
/// GetBucketInventory responses where either may appear.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryEncryption {
    #[serde(rename = "SSE-OSS")]
    pub sse_oss: Option<SseOssEncryption>,
    #[serde(rename = "SSE-KMS")]
    pub sse_kms: Option<SseKmsInventoryEncryption>,
}

/// `<OSSBucketDestination>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OssBucketDestination {
    pub format: InventoryFormat,
    pub account_id: String,
    pub role_arn: String,
    pub bucket: String,
    pub prefix: Option<String>,
    pub encryption: Option<InventoryEncryption>,
}

/// `<Destination>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryDestination {
    #[serde(rename = "OSSBucketDestination")]
    pub oss_bucket_destination: OssBucketDestination,
}

/// `<Schedule>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventorySchedule {
    pub frequency: InventoryFrequency,
}

/// `<Filter>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryFilter {
    pub prefix: Option<String>,
    pub last_modify_begin_time_stamp: Option<String>,
    pub last_modify_end_time_stamp: Option<String>,
    pub lower_size_bound: Option<String>,
    pub upper_size_bound: Option<String>,
    /// Comma-separated list. OSS accepts "Standard,IA,Archive,ColdArchive,All".
    pub storage_class: Option<String>,
}

/// `<IncrementalInventory>` container (optional).
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncrementalInventory {
    pub is_enabled: Option<bool>,
    pub schedule: Option<IncrementalInventorySchedule>,
    pub optional_fields: Option<OptionalFields>,
}

/// Incremental-inventory schedule uses a numeric frequency in seconds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncrementalInventorySchedule {
    pub frequency: u32,
}

/// Full `<InventoryConfiguration>` body.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "InventoryConfiguration", rename_all = "PascalCase")]
pub struct InventoryConfiguration {
    #[serde(rename = "Id")]
    pub id: String,
    pub is_enabled: bool,
    pub filter: Option<InventoryFilter>,
    pub destination: InventoryDestination,
    pub schedule: InventorySchedule,
    pub included_object_versions: IncludedObjectVersions,
    pub optional_fields: Option<OptionalFields>,
    pub incremental_inventory: Option<IncrementalInventory>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_config() {
        let xml = r#"<InventoryConfiguration>
   <Id>report1</Id>
   <IsEnabled>true</IsEnabled>
   <Destination>
      <OSSBucketDestination>
         <Format>CSV</Format>
         <AccountId>1000000000000000</AccountId>
         <RoleArn>acs:ram::1000000000000000:role/AliyunOSSRole</RoleArn>
         <Bucket>acs:oss:::destination-bucket</Bucket>
         <Prefix>prefix1</Prefix>
         <Encryption>
            <SSE-OSS/>
         </Encryption>
      </OSSBucketDestination>
   </Destination>
   <Schedule>
      <Frequency>Daily</Frequency>
   </Schedule>
   <Filter>
     <Prefix>myprefix/</Prefix>
   </Filter>
   <IncludedObjectVersions>All</IncludedObjectVersions>
   <OptionalFields>
      <Field>Size</Field>
      <Field>ETag</Field>
   </OptionalFields>
</InventoryConfiguration>"#;
        let parsed: InventoryConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.id, "report1");
        assert!(parsed.is_enabled);
        assert_eq!(parsed.schedule.frequency, InventoryFrequency::Daily);
        let fields = &parsed.optional_fields.unwrap().fields;
        assert_eq!(*fields, vec![InventoryOptionalField::Size, InventoryOptionalField::ETag]);
        assert!(
            parsed
                .destination
                .oss_bucket_destination
                .encryption
                .unwrap()
                .sse_oss
                .is_some()
        );
    }

    #[test]
    fn parse_transition_time_alias() {
        let xml = r#"<OptionalFields><Field>TransistionTime</Field></OptionalFields>"#;
        let parsed: OptionalFields = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.fields, vec![InventoryOptionalField::TransitionTime]);
    }

    #[test]
    fn serialize_round_trip() {
        let cfg = InventoryConfiguration {
            id: "r".to_string(),
            is_enabled: true,
            destination: InventoryDestination {
                oss_bucket_destination: OssBucketDestination {
                    format: InventoryFormat::Csv,
                    account_id: "a".to_string(),
                    role_arn: "r".to_string(),
                    bucket: "b".to_string(),
                    ..Default::default()
                },
            },
            schedule: InventorySchedule {
                frequency: InventoryFrequency::Weekly,
            },
            included_object_versions: IncludedObjectVersions::Current,
            ..Default::default()
        };
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        let back: InventoryConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}

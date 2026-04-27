//! Replication rule types shared by `PutBucketReplication`, `GetBucketReplication`
//! and `GetBucketReplicationProgress`.
//!
//! See <https://www.alibabacloud.com/help/en/oss/developer-reference/putbucketreplication>
//! for the authoritative element reference.

use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

/// Replication action(s) propagated to the destination bucket.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ReplicationAction {
    /// PUT / DELETE / ABORT operations are all replicated.
    #[default]
    #[serde(rename = "ALL")]
    All,
    /// Only write (PUT) operations are replicated.
    #[serde(rename = "PUT")]
    Put,
}

/// Data transfer link used for cross-region replication.
///
/// This type uses a manual `Deserialize` impl instead of the `#[derive]`
/// version because quick-xml 0.39 treats derived enums specially when they
/// appear as the text content of an element (`<Type>oss_acc</Type>`): it tries
/// to interpret the element *tag* (`Type`) as the variant name rather than the
/// text. The manual impl deserializes to a `String` first and then picks the
/// variant.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize)]
pub enum TransferType {
    /// Default OSS transfer link.
    #[default]
    #[serde(rename = "internal")]
    Internal,
    /// Transfer acceleration (CRR only).
    #[serde(rename = "oss_acc")]
    OssAcc,
}

impl<'de> Deserialize<'de> for TransferType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "internal" => Ok(TransferType::Internal),
            "oss_acc" => Ok(TransferType::OssAcc),
            other => Err(serde::de::Error::custom(format!(
                "unknown TransferType `{other}`, expected `internal` or `oss_acc`"
            ))),
        }
    }
}

/// Whether a replication rule also replicates historical data.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HistoricalObjectReplication {
    #[default]
    Enabled,
    Disabled,
}

/// Status of a replication rule itself (returned by GetBucketReplication).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReplicationRuleStatus {
    /// OSS is preparing the replication tasks.
    Starting,
    /// Replication is in effect.
    Doing,
    /// The rule has been deleted and OSS is cleaning up.
    Closing,
}

/// Enable/disable flag used by RTC and SseKmsEncryptedObjects. OSS uses
/// both lowercase (`enabled` / `disabled`) on the RTC `<Status>` element
/// and Pascal-case (`Enabled` / `Disabled`) on the SSE-KMS `<Status>`
/// element. Response of GetBucketReplication may also return `enabling`
/// for RTC transitions. We model the union conservatively.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RtcStatus {
    #[serde(alias = "enabled", alias = "Enabled")]
    #[serde(rename = "enabled")]
    Enabled,
    #[default]
    #[serde(alias = "disabled", alias = "Disabled")]
    #[serde(rename = "disabled")]
    Disabled,
    /// Transitional state surfaced only by GetBucketReplication.
    #[serde(alias = "Enabling")]
    #[serde(rename = "enabling")]
    Enabling,
}

/// `<RTC>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rtc {
    pub status: RtcStatus,
}

/// `<PrefixSet>` container. OSS wraps the list in a `<Prefix>` repetition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrefixSet {
    #[serde(rename = "Prefix", default, skip_serializing_if = "Vec::is_empty")]
    pub prefixes: Vec<String>,
}

/// `<Destination>` container for a replication rule.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationDestination {
    pub bucket: String,
    pub location: String,
    pub transfer_type: Option<TransferType>,
}

/// `<SseKmsEncryptedObjects>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SseKmsEncryptedObjects {
    pub status: Option<SseKmsStatus>,
}

/// The subset of `RtcStatus` allowed on SseKmsEncryptedObjects: Pascal-case
/// `Enabled` / `Disabled`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SseKmsStatus {
    #[default]
    Enabled,
    Disabled,
}

/// `<SourceSelectionCriteria>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SourceSelectionCriteria {
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

/// `<EncryptionConfiguration>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationEncryptionConfiguration {
    #[serde(rename = "ReplicaKmsKeyID")]
    pub replica_kms_key_id: Option<String>,
}

/// Tag filtering policy (within `<UserTaggings>`).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserTaggingFilterType {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

/// Single `<UserTagging>` entry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserTagging {
    pub key: String,
    pub value: String,
}

impl UserTagging {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// `<UserTaggings>` container.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserTaggings {
    pub filter_type: Option<UserTaggingFilterType>,
    #[serde(rename = "UserTagging", default, skip_serializing_if = "Vec::is_empty")]
    pub user_taggings: Vec<UserTagging>,
}

/// Replication progress information (returned by GetBucketReplicationProgress).
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationProgressInfo {
    /// Percentage of replicated historical data as a string (e.g. "0.85").
    pub historical_object: Option<String>,
    /// GMT timestamp indicating replication cutoff for new objects.
    pub new_object: Option<String>,
}

/// A single `<Rule>` inside `<ReplicationConfiguration>`.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationRule {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "RTC")]
    pub rtc: Option<Rtc>,
    pub prefix_set: Option<PrefixSet>,
    pub action: Option<ReplicationAction>,
    pub destination: Option<ReplicationDestination>,
    pub historical_object_replication: Option<HistoricalObjectReplication>,
    pub sync_role: Option<String>,
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    pub encryption_configuration: Option<ReplicationEncryptionConfiguration>,
    pub user_taggings: Option<UserTaggings>,
    /// Populated by GetBucketReplication only.
    pub status: Option<ReplicationRuleStatus>,
    /// Populated by GetBucketReplicationProgress only.
    pub progress: Option<ReplicationProgressInfo>,
}

/// Root `<ReplicationConfiguration>` element. Supports both
/// PutBucketReplication (request body) and GetBucketReplication (response
/// body).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ReplicationConfiguration")]
pub struct ReplicationConfiguration {
    #[serde(rename = "Rule", default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ReplicationRule>,
}

impl ReplicationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rules(rules: Vec<ReplicationRule>) -> Self {
        Self { rules }
    }
}

fn unwrap_locations<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<String>::deserialize(deserializer)
}

fn unwrap_transfer_types<'de, D>(deserializer: D) -> std::result::Result<Vec<TransferType>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Inner {
        #[serde(rename = "Type", default)]
        ty: Vec<TransferType>,
    }
    Ok(Inner::deserialize(deserializer)?.ty)
}

/// `<LocationTransferType>` entry inside `<LocationTransferTypeConstraint>`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationTransferType {
    pub location: String,
    /// Inner `<TransferTypes><Type>...</Type></TransferTypes>` flattened to a
    /// `Vec<TransferType>`.
    #[serde(default, deserialize_with = "unwrap_transfer_types")]
    pub transfer_types: Vec<TransferType>,
}

/// `<LocationTransferTypeConstraint>` container.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationTransferTypeConstraint {
    #[serde(rename = "LocationTransferType", default)]
    pub location_transfer_types: Vec<LocationTransferType>,
}

/// Response body for `GetBucketReplicationLocation` (XML root
/// `<ReplicationLocation>`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicationLocation {
    #[serde(rename = "Location", default, deserialize_with = "unwrap_locations")]
    pub locations: Vec<String>,
    pub location_transfer_type_constraint: Option<LocationTransferTypeConstraint>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_rule() {
        let xml = r#"<ReplicationConfiguration>
  <Rule>
    <ID>test_replication_1</ID>
    <PrefixSet>
      <Prefix>source1</Prefix>
      <Prefix>video</Prefix>
    </PrefixSet>
    <UserTaggings>
      <FilterType>OR</FilterType>
      <UserTagging><Key>key1</Key><Value>value1</Value></UserTagging>
      <UserTagging><Key>key2</Key><Value>value2</Value></UserTagging>
    </UserTaggings>
    <Action>PUT</Action>
    <Destination>
      <Bucket>destbucket</Bucket>
      <Location>oss-cn-beijing</Location>
      <TransferType>oss_acc</TransferType>
    </Destination>
    <Status>doing</Status>
    <HistoricalObjectReplication>enabled</HistoricalObjectReplication>
    <SyncRole>aliyunramrole</SyncRole>
    <RTC>
      <Status>enabled</Status>
    </RTC>
  </Rule>
</ReplicationConfiguration>"#;
        let parsed: ReplicationConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.rules.len(), 1);
        let rule = &parsed.rules[0];
        assert_eq!(rule.id.as_deref(), Some("test_replication_1"));
        assert_eq!(rule.action, Some(ReplicationAction::Put));
        let dest = rule.destination.as_ref().unwrap();
        assert_eq!(dest.bucket, "destbucket");
        assert_eq!(dest.transfer_type, Some(TransferType::OssAcc));
        assert_eq!(rule.status, Some(ReplicationRuleStatus::Doing));
        assert_eq!(rule.rtc.as_ref().unwrap().status, RtcStatus::Enabled);
        let prefixes = &rule.prefix_set.as_ref().unwrap().prefixes;
        assert_eq!(prefixes, &vec!["source1".to_string(), "video".to_string()]);
        let tags = rule.user_taggings.as_ref().unwrap();
        assert_eq!(tags.filter_type, Some(UserTaggingFilterType::Or));
        assert_eq!(tags.user_taggings.len(), 2);
    }

    #[test]
    fn serialize_minimal_rule_round_trip() {
        let cfg = ReplicationConfiguration::with_rules(vec![ReplicationRule {
            prefix_set: Some(PrefixSet {
                prefixes: vec!["source1".to_string()],
            }),
            action: Some(ReplicationAction::Put),
            destination: Some(ReplicationDestination {
                bucket: "destbucket".to_string(),
                location: "oss-cn-beijing".to_string(),
                transfer_type: Some(TransferType::OssAcc),
            }),
            historical_object_replication: Some(HistoricalObjectReplication::Enabled),
            sync_role: Some("aliyunramrole".to_string()),
            ..Default::default()
        }]);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<ReplicationConfiguration>"));
        assert!(xml.contains("<Action>PUT</Action>"));
        assert!(xml.contains("<TransferType>oss_acc</TransferType>"));
        let back: ReplicationConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }

    #[test]
    fn parse_replication_location() {
        let xml = r#"<ReplicationLocation>
  <Location>oss-cn-beijing</Location>
  <Location>oss-cn-qingdao</Location>
  <LocationTransferTypeConstraint>
    <LocationTransferType>
      <Location>oss-cn-hongkong</Location>
      <TransferTypes><Type>oss_acc</Type></TransferTypes>
    </LocationTransferType>
  </LocationTransferTypeConstraint>
</ReplicationLocation>"#;
        let parsed: ReplicationLocation = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(
            parsed.locations,
            vec!["oss-cn-beijing".to_string(), "oss-cn-qingdao".to_string()]
        );
        let constraint = parsed.location_transfer_type_constraint.unwrap();
        assert_eq!(constraint.location_transfer_types.len(), 1);
        assert_eq!(constraint.location_transfer_types[0].location, "oss-cn-hongkong");
        assert_eq!(constraint.location_transfer_types[0].transfer_types, vec![TransferType::OssAcc]);
    }

    #[test]
    fn parse_progress() {
        let xml = r#"<ReplicationProgress>
  <Rule>
    <ID>test_replication_1</ID>
    <PrefixSet><Prefix>video</Prefix></PrefixSet>
    <Action>PUT</Action>
    <Destination>
      <Bucket>target</Bucket>
      <Location>oss-cn-beijing</Location>
      <TransferType>oss_acc</TransferType>
    </Destination>
    <Status>doing</Status>
    <HistoricalObjectReplication>enabled</HistoricalObjectReplication>
    <Progress>
      <HistoricalObject>0.85</HistoricalObject>
      <NewObject>2015-09-24T15:28:14.000Z</NewObject>
    </Progress>
  </Rule>
</ReplicationProgress>"#;
        // The wrapper type is identical to ReplicationConfiguration (just a
        // different root name). We reuse the same struct here.
        #[derive(Deserialize)]
        #[serde(rename = "ReplicationProgress", rename_all = "PascalCase")]
        struct ReplicationProgress {
            #[serde(rename = "Rule", default)]
            rules: Vec<ReplicationRule>,
        }
        let parsed: ReplicationProgress = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.rules.len(), 1);
        let progress = parsed.rules[0].progress.as_ref().unwrap();
        assert_eq!(progress.historical_object.as_deref(), Some("0.85"));
    }
}

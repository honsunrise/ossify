//! Lifecycle rule types used by `PutBucketLifecycle` / `GetBucketLifecycle`.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ops::common::{StorageClass, Tag};

/// Status of an individual lifecycle rule.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LifecycleRuleStatus {
    #[default]
    Enabled,
    Disabled,
}

/// `<Expiration>` element: delete operation for current versions of objects.
///
/// Exactly one of `days`, `created_before_date`, or `expired_object_delete_marker`
/// is typically set.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleExpiration {
    /// Days after last modification before the object expires.
    pub days: Option<u32>,
    /// Objects whose last-modified time is before this date expire.
    pub created_before_date: Option<String>,
    /// Whether to automatically remove expired delete markers.
    pub expired_object_delete_marker: Option<bool>,
}

/// `<Transition>` element: change storage class on the current version.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleTransition {
    pub days: Option<u32>,
    pub created_before_date: Option<String>,
    /// Target storage class. Valid values: IA, Archive, ColdArchive,
    /// DeepColdArchive.
    pub storage_class: Option<StorageClass>,
    /// When true, the rule applies based on the last access time rather than
    /// the last modified time.
    pub is_access_time: Option<bool>,
    /// When `is_access_time` is true, controls whether objects are changed
    /// back to Standard upon access.
    pub return_to_std_when_visit: Option<bool>,
    /// When true, objects < 64 KB are also transitioned.
    pub allow_small_file: Option<bool>,
}

/// `<AbortMultipartUpload>` element: delete stale multipart-upload parts.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbortIncompleteMultipartUpload {
    pub days: Option<u32>,
    pub created_before_date: Option<String>,
}

/// `<NoncurrentVersionExpiration>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoncurrentVersionExpiration {
    /// Days after an object became non-current before the rule fires.
    pub noncurrent_days: Option<u32>,
}

/// `<NoncurrentVersionTransition>` element.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoncurrentVersionTransition {
    pub noncurrent_days: Option<u32>,
    pub storage_class: Option<StorageClass>,
    pub is_access_time: Option<bool>,
    pub return_to_std_when_visit: Option<bool>,
    pub allow_small_file: Option<bool>,
}

/// `<Filter>/<Not>` element: negative-match filter for a lifecycle rule.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleFilterNot {
    /// Prefix of objects excluded from the rule.
    pub prefix: Option<String>,
    /// Tag objects excluded from the rule.
    pub tag: Option<Tag>,
}

/// `<Filter>` element: holds the single `<Not>` node plus object size limits.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleFilter {
    pub not: Option<LifecycleFilterNot>,
    /// Minimum object size (in bytes) the rule applies to.
    pub object_size_greater_than: Option<u64>,
    /// Maximum object size (in bytes) the rule applies to.
    pub object_size_less_than: Option<u64>,
}

/// Single `<Rule>` in a `<LifecycleConfiguration>`.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LifecycleRule {
    /// Optional rule ID (<= 255 chars). OSS auto-generates one if omitted.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Object key prefix this rule applies to.
    pub prefix: Option<String>,
    /// Enabled / Disabled.
    pub status: LifecycleRuleStatus,
    /// Expiration (current-version delete) action.
    pub expiration: Option<LifecycleExpiration>,
    /// Zero or more transition actions (current-version storage-class changes).
    /// XML repeats the element; a `Vec<>` with default is appropriate.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transition: Vec<LifecycleTransition>,
    /// Optional stale-multipart-upload abort action.
    pub abort_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    /// Tag filter(s) on the rule itself.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<Tag>,
    /// Expiration for non-current (historical) versions.
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    /// Transition for non-current versions. Repeats.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub noncurrent_version_transition: Vec<NoncurrentVersionTransition>,
    /// Advanced filter (Not / object size).
    pub filter: Option<LifecycleFilter>,
    /// Set by OSS on read: the UNIX timestamp when the bucket started tracking
    /// access time. Not sent on write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atime_base: Option<u64>,
}

/// Root `<LifecycleConfiguration>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "LifecycleConfiguration")]
pub struct LifecycleConfiguration {
    /// Lifecycle rules for the bucket (up to 1,000).
    #[serde(rename = "Rule", default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<LifecycleRule>,
}

impl LifecycleConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rules(rules: Vec<LifecycleRule>) -> Self {
        Self { rules }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_transition_rule() {
        let xml = r#"<LifecycleConfiguration>
  <Rule>
    <ID>rule</ID>
    <Prefix>log/</Prefix>
    <Status>Enabled</Status>
    <Transition>
      <Days>30</Days>
      <StorageClass>IA</StorageClass>
    </Transition>
  </Rule>
</LifecycleConfiguration>"#;
        let parsed: LifecycleConfiguration = quick_xml::de::from_str(xml).unwrap();
        assert_eq!(parsed.rules.len(), 1);
        let rule = &parsed.rules[0];
        assert_eq!(rule.id.as_deref(), Some("rule"));
        assert_eq!(rule.prefix.as_deref(), Some("log/"));
        assert_eq!(rule.status, LifecycleRuleStatus::Enabled);
        assert_eq!(rule.transition.len(), 1);
        assert_eq!(rule.transition[0].days, Some(30));
        assert_eq!(rule.transition[0].storage_class, Some(StorageClass::InfrequentAccess));
    }

    #[test]
    fn parse_multiple_transitions() {
        let xml = r#"<LifecycleConfiguration>
  <Rule>
    <ID>rule</ID>
    <Prefix>log/</Prefix>
    <Status>Enabled</Status>
    <Transition>
      <Days>30</Days>
      <StorageClass>IA</StorageClass>
    </Transition>
    <Transition>
      <Days>60</Days>
      <StorageClass>Archive</StorageClass>
    </Transition>
    <Expiration>
      <Days>3600</Days>
    </Expiration>
  </Rule>
</LifecycleConfiguration>"#;
        let parsed: LifecycleConfiguration = quick_xml::de::from_str(xml).unwrap();
        let rule = &parsed.rules[0];
        assert_eq!(rule.transition.len(), 2);
        assert_eq!(rule.transition[1].storage_class, Some(StorageClass::Archive));
        assert_eq!(rule.expiration.as_ref().unwrap().days, Some(3600));
    }

    #[test]
    fn parse_noncurrent_version_expiration_with_delete_marker() {
        let xml = r#"<LifecycleConfiguration>
  <Rule>
    <ID>rule</ID>
    <Prefix></Prefix>
    <Status>Enabled</Status>
    <Expiration>
      <ExpiredObjectDeleteMarker>true</ExpiredObjectDeleteMarker>
    </Expiration>
    <NoncurrentVersionExpiration>
      <NoncurrentDays>5</NoncurrentDays>
    </NoncurrentVersionExpiration>
  </Rule>
</LifecycleConfiguration>"#;
        let parsed: LifecycleConfiguration = quick_xml::de::from_str(xml).unwrap();
        let rule = &parsed.rules[0];
        assert_eq!(rule.expiration.as_ref().unwrap().expired_object_delete_marker, Some(true));
        assert_eq!(
            rule.noncurrent_version_expiration
                .as_ref()
                .unwrap()
                .noncurrent_days,
            Some(5)
        );
    }

    #[test]
    fn parse_filter_with_not() {
        let xml = r#"<LifecycleConfiguration>
  <Rule>
    <ID>rule</ID>
    <Prefix></Prefix>
    <Status>Enabled</Status>
    <Filter>
      <Not>
        <Prefix>log</Prefix>
        <Tag><Key>key1</Key><Value>value1</Value></Tag>
      </Not>
    </Filter>
    <Transition>
      <Days>30</Days>
      <StorageClass>Archive</StorageClass>
    </Transition>
    <Expiration>
      <Days>100</Days>
    </Expiration>
  </Rule>
</LifecycleConfiguration>"#;
        let parsed: LifecycleConfiguration = quick_xml::de::from_str(xml).unwrap();
        let rule = &parsed.rules[0];
        let not = rule.filter.as_ref().unwrap().not.as_ref().unwrap();
        assert_eq!(not.prefix.as_deref(), Some("log"));
        assert_eq!(not.tag.as_ref().unwrap().key, "key1");
    }

    #[test]
    fn parse_access_time_with_atime_base() {
        let xml = r#"<LifecycleConfiguration>
  <Rule>
    <ID>atime rule</ID>
    <Prefix>logs1/</Prefix>
    <Status>Enabled</Status>
    <Transition>
      <Days>30</Days>
      <StorageClass>IA</StorageClass>
      <IsAccessTime>true</IsAccessTime>
      <ReturnToStdWhenVisit>false</ReturnToStdWhenVisit>
    </Transition>
    <AtimeBase>1631698332</AtimeBase>
  </Rule>
</LifecycleConfiguration>"#;
        let parsed: LifecycleConfiguration = quick_xml::de::from_str(xml).unwrap();
        let rule = &parsed.rules[0];
        assert_eq!(rule.transition[0].is_access_time, Some(true));
        assert_eq!(rule.transition[0].return_to_std_when_visit, Some(false));
        assert_eq!(rule.atime_base, Some(1631698332));
    }

    #[test]
    fn parse_abort_multipart_upload() {
        let xml = r#"<LifecycleConfiguration>
  <Rule>
    <ID>rule</ID>
    <Prefix>/</Prefix>
    <Status>Enabled</Status>
    <AbortMultipartUpload>
      <Days>30</Days>
    </AbortMultipartUpload>
  </Rule>
</LifecycleConfiguration>"#;
        let parsed: LifecycleConfiguration = quick_xml::de::from_str(xml).unwrap();
        let amu = parsed.rules[0].abort_multipart_upload.as_ref().unwrap();
        assert_eq!(amu.days, Some(30));
    }

    #[test]
    fn serialize_minimal_rule() {
        let cfg = LifecycleConfiguration::with_rules(vec![LifecycleRule {
            id: Some("rule".to_string()),
            prefix: Some("log/".to_string()),
            status: LifecycleRuleStatus::Enabled,
            expiration: Some(LifecycleExpiration {
                days: Some(90),
                ..Default::default()
            }),
            ..Default::default()
        }]);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        assert!(xml.contains("<LifecycleConfiguration>"));
        assert!(xml.contains("<Rule>"));
        assert!(xml.contains("<ID>rule</ID>"));
        assert!(xml.contains("<Prefix>log/</Prefix>"));
        assert!(xml.contains("<Status>Enabled</Status>"));
        assert!(xml.contains("<Days>90</Days>"));
    }

    #[test]
    fn serialize_two_transitions_round_trip() {
        let cfg = LifecycleConfiguration::with_rules(vec![LifecycleRule {
            id: Some("r".to_string()),
            prefix: Some("l/".to_string()),
            status: LifecycleRuleStatus::Enabled,
            transition: vec![
                LifecycleTransition {
                    days: Some(30),
                    storage_class: Some(StorageClass::InfrequentAccess),
                    ..Default::default()
                },
                LifecycleTransition {
                    days: Some(60),
                    storage_class: Some(StorageClass::Archive),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }]);
        let xml = quick_xml::se::to_string(&cfg).unwrap();
        let back: LifecycleConfiguration = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, cfg);
    }
}

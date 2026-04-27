use serde::{Deserialize, Serialize};

/// Generic `<Tag>` element with `<Key>` and `<Value>` children. Shared by
/// bucket tagging, object tagging, lifecycle filters, inventory filters, etc.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// `<TagSet>` wrapper holding a repeated `<Tag>` element. Shared by
/// `PutBucketTags` / `GetBucketTags` / `PutObjectTagging` /
/// `GetObjectTagging`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagSet {
    #[serde(rename = "Tag", default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

/// Root `<Tagging>` element.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Tagging", rename_all = "PascalCase")]
pub struct Tagging {
    pub tag_set: TagSet,
}

impl Tagging {
    pub fn new(tags: Vec<Tag>) -> Self {
        Self {
            tag_set: TagSet { tags },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_xml_round_trip() {
        let tag = Tag::new("foo", "bar");
        let xml = quick_xml::se::to_string(&tag).unwrap();
        assert!(xml.contains("<Key>foo</Key>"));
        assert!(xml.contains("<Value>bar</Value>"));
        let parsed: Tag = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(parsed, tag);
    }

    #[test]
    fn tagging_round_trip() {
        let tagging = Tagging::new(vec![Tag::new("k1", "v1"), Tag::new("k2", "v2")]);
        let xml = quick_xml::se::to_string(&tagging).unwrap();
        assert!(xml.contains("<Tagging>"));
        assert!(xml.contains("<TagSet>"));
        assert!(xml.contains("<Tag>"));
        assert!(xml.contains("<Key>k1</Key>"));
        let back: Tagging = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(back, tagging);
    }
}

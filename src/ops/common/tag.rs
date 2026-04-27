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
}

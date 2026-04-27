use serde::{Deserialize, Serialize};

/// XML `<Owner>` element returned by most list-style operations.
///
/// `id` and `display_name` are optional because anonymous listings (e.g. public
/// buckets) may omit them.
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Owner {
    /// Owner UID. XML element is `<ID>`.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// Display name. Historically OSS sets this equal to `id`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub display_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_xml_round_trip() {
        let original = Owner {
            id: Some("150692521021****".to_string()),
            display_name: Some("150692521021****".to_string()),
        };
        let xml = quick_xml::se::to_string(&original).unwrap();
        assert!(xml.contains("<ID>150692521021****</ID>"));
        assert!(xml.contains("<DisplayName>150692521021****</DisplayName>"));

        let parsed: Owner = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn owner_accepts_missing_fields() {
        let parsed: Owner = quick_xml::de::from_str("<Owner></Owner>").unwrap();
        assert_eq!(parsed, Owner::default());
    }
}

use serde::{Deserialize, Serialize};

/// Encoding type for list operations. OSS currently only supports `url`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodingType {
    #[serde(rename = "url")]
    Url,
}

impl EncodingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EncodingType::Url => "url",
        }
    }
}

impl AsRef<str> for EncodingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding_type_wire_name() {
        assert_eq!(serde_json::to_string(&EncodingType::Url).unwrap(), "\"url\"");
    }

    #[test]
    fn encoding_type_round_trip() {
        let json = serde_json::to_string(&EncodingType::Url).unwrap();
        let back: EncodingType = serde_json::from_str(&json).unwrap();
        assert_eq!(back, EncodingType::Url);
    }
}

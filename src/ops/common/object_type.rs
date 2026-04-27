use serde::{Deserialize, Serialize};

/// OSS object type. Returned by `HeadObject`, `GetObjectMeta`, various list
/// operations, etc.
///
/// The wire representation matches what OSS returns in XML / the
/// `x-oss-object-type` header: `Normal`, `Multipart`, `Appendable`, `Symlink`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ObjectType {
    /// Uploaded via `PutObject`.
    #[default]
    Normal,
    /// Uploaded via multipart upload.
    Multipart,
    /// Uploaded via `AppendObject`.
    Appendable,
    /// Created via `PutSymlink`.
    Symlink,
}

impl ObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectType::Normal => "Normal",
            ObjectType::Multipart => "Multipart",
            ObjectType::Appendable => "Appendable",
            ObjectType::Symlink => "Symlink",
        }
    }
}

impl AsRef<str> for ObjectType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_type_wire_names() {
        assert_eq!(serde_json::to_string(&ObjectType::Normal).unwrap(), "\"Normal\"");
        assert_eq!(serde_json::to_string(&ObjectType::Multipart).unwrap(), "\"Multipart\"");
        assert_eq!(serde_json::to_string(&ObjectType::Appendable).unwrap(), "\"Appendable\"");
        assert_eq!(serde_json::to_string(&ObjectType::Symlink).unwrap(), "\"Symlink\"");
    }

    #[test]
    fn object_type_round_trip() {
        for ot in [
            ObjectType::Normal,
            ObjectType::Multipart,
            ObjectType::Appendable,
            ObjectType::Symlink,
        ] {
            let json = serde_json::to_string(&ot).unwrap();
            let back: ObjectType = serde_json::from_str(&json).unwrap();
            assert_eq!(ot, back);
        }
    }
}

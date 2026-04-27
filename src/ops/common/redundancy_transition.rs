use serde::{Deserialize, Serialize};

/// Status of a data-redundancy transition task. Used by
/// `ListUserDataRedundancyTransition`, `GetBucketDataRedundancyTransition`,
/// `ListBucketDataRedundancyTransition`, etc.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RedundancyTransitionStatus {
    Queueing,
    Processing,
    Finished,
}

impl RedundancyTransitionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RedundancyTransitionStatus::Queueing => "Queueing",
            RedundancyTransitionStatus::Processing => "Processing",
            RedundancyTransitionStatus::Finished => "Finished",
        }
    }
}

impl AsRef<str> for RedundancyTransitionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_names() {
        for s in [
            RedundancyTransitionStatus::Queueing,
            RedundancyTransitionStatus::Processing,
            RedundancyTransitionStatus::Finished,
        ] {
            let json = serde_json::to_string(&s).unwrap();
            let back: RedundancyTransitionStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(s, back);
        }
    }
}

//! Shared OSS types used across multiple API operations.
//!
//! As the SDK grows to cover the entire OSS API surface, many request/response
//! payloads reference the same XML element types (for example `StorageClass`,
//! `ServerSideEncryption`, or `Owner`). Keeping them in one place avoids drift
//! between duplicate definitions and makes them easy to reuse in future
//! operations.

mod acl;
mod encoding_type;
mod encryption;
mod lifecycle;
mod object_type;
mod owner;
mod redundancy_transition;
mod replication;
mod storage_class;
mod tag;

pub use acl::BucketAcl;
pub use encoding_type::EncodingType;
pub use encryption::ServerSideEncryption;
pub use lifecycle::{
    AbortIncompleteMultipartUpload, LifecycleConfiguration, LifecycleExpiration, LifecycleFilter,
    LifecycleFilterNot, LifecycleRule, LifecycleRuleStatus, LifecycleTransition,
    NoncurrentVersionExpiration, NoncurrentVersionTransition,
};
pub use object_type::ObjectType;
pub use owner::Owner;
pub use redundancy_transition::RedundancyTransitionStatus;
pub use replication::{
    HistoricalObjectReplication, LocationTransferType, LocationTransferTypeConstraint, PrefixSet,
    ReplicationAction, ReplicationConfiguration, ReplicationDestination,
    ReplicationEncryptionConfiguration, ReplicationLocation, ReplicationProgressInfo,
    ReplicationRule, ReplicationRuleStatus, Rtc, RtcStatus, SourceSelectionCriteria,
    SseKmsEncryptedObjects, SseKmsStatus, TransferType, UserTagging, UserTaggingFilterType,
    UserTaggings,
};
pub use storage_class::{DataRedundancyType, StorageClass};
pub use tag::Tag;

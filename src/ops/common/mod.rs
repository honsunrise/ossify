//! Shared OSS types used across multiple API operations.
//!
//! As the SDK grows to cover the entire OSS API surface, many request/response
//! payloads reference the same XML element types (for example `StorageClass`,
//! `ServerSideEncryption`, or `Owner`). Keeping them in one place avoids drift
//! between duplicate definitions and makes them easy to reuse in future
//! operations.

mod access_point;
mod acl;
mod anti_ddos;
mod encoding_type;
mod encryption;
mod inventory;
mod lifecycle;
mod live_channel;
mod object_fc_access_point;
mod object_type;
mod owner;
mod public_access_block;
mod qos;
mod redundancy_transition;
mod replication;
mod resource_pool;
mod storage_class;
mod tag;

pub use access_point::{AccessPointNetworkOrigin, AccessPointStatus, VpcConfiguration};
pub use acl::{BucketAcl, ObjectAcl};
pub use anti_ddos::{
    AntiDdosCnames, AntiDdosConfiguration, AntiDdosListConfiguration, AntiDdosStatus, AntiDdosType,
};
pub use encoding_type::EncodingType;
pub use encryption::ServerSideEncryption;
pub use inventory::{
    IncludedObjectVersions, IncrementalInventory, IncrementalInventorySchedule,
    InventoryConfiguration, InventoryDestination, InventoryEncryption, InventoryFilter,
    InventoryFormat, InventoryFrequency, InventoryOptionalField, InventorySchedule,
    OptionalFields, OssBucketDestination, SseKmsInventoryEncryption, SseOssEncryption,
};
pub use lifecycle::{
    AbortIncompleteMultipartUpload, LifecycleConfiguration, LifecycleExpiration, LifecycleFilter,
    LifecycleFilterNot, LifecycleRule, LifecycleRuleStatus, LifecycleTransition,
    NoncurrentVersionExpiration, NoncurrentVersionTransition,
};
pub use live_channel::{
    LiveChannelAudioStat, LiveChannelConfiguration, LiveChannelHistoryRecord,
    LiveChannelSnapshot, LiveChannelStatus, LiveChannelSummary, LiveChannelTarget, LiveChannelUrls,
    LiveChannelVideoStat,
};
pub use object_fc_access_point::{
    AccessPointForObjectProcessSummary, ObjectFcAccessPointStatus, ObjectFcActions,
    ObjectFcAllowedFeatures, ObjectFcContentTransformation, ObjectFcEndpoints,
    ObjectFcFunctionCompute, ObjectFcTransformationConfiguration,
    ObjectFcTransformationConfigurations, ObjectProcessConfiguration,
};
pub use object_type::ObjectType;
pub use owner::Owner;
pub use public_access_block::PublicAccessBlockConfiguration;
pub use qos::{QoSConfiguration, RequesterQoSInfo};
pub use redundancy_transition::RedundancyTransitionStatus;
pub use replication::{
    HistoricalObjectReplication, LocationTransferType, LocationTransferTypeConstraint, PrefixSet,
    ReplicationAction, ReplicationConfiguration, ReplicationDestination,
    ReplicationEncryptionConfiguration, ReplicationLocation, ReplicationProgressInfo,
    ReplicationRule, ReplicationRuleStatus, Rtc, RtcStatus, SourceSelectionCriteria,
    SseKmsEncryptedObjects, SseKmsStatus, TransferType, UserTagging, UserTaggingFilterType,
    UserTaggings,
};
pub use resource_pool::{
    GroupBucketInfo, ResourcePool, ResourcePoolBucket, ResourcePoolBucketGroup,
};
pub use storage_class::{DataRedundancyType, StorageClass};
pub use tag::{Tag, TagSet, Tagging};

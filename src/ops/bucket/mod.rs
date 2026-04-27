pub mod acl;
pub mod base;
pub mod data_indexing;
pub mod lifecycle;
pub mod replication;
pub mod retention;
pub mod transfer_acceleration;
pub mod versioning;

// Re-export everything so callers can `use ossify::ops::bucket::*` and get
// every bucket-level type and trait.
pub use acl::*;
pub use base::*;
pub use data_indexing::*;
pub use lifecycle::*;
pub use replication::*;
pub use retention::*;
pub use transfer_acceleration::*;
pub use versioning::*;

pub mod acl;
pub mod base;
pub mod data_indexing;
pub mod encryption;
pub mod inventory;
pub mod lifecycle;
pub mod logging;
pub mod policy;
pub mod referer;
pub mod replication;
pub mod request_payment;
pub mod retention;
pub mod tagging;
pub mod transfer_acceleration;
pub mod versioning;
pub mod website;

// Re-export everything so callers can `use ossify::ops::bucket::*` and get
// every bucket-level type and trait.
pub use acl::*;
pub use base::*;
pub use data_indexing::*;
pub use encryption::*;
pub use inventory::*;
pub use lifecycle::*;
pub use logging::*;
pub use policy::*;
pub use referer::*;
pub use replication::*;
pub use request_payment::*;
pub use retention::*;
pub use tagging::*;
pub use transfer_acceleration::*;
pub use versioning::*;
pub use website::*;

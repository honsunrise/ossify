//! Shared Block-Public-Access configuration shapes used by a family of
//! PublicAccessBlock APIs (global/user-level, bucket-level, access-point level,
//! and Object FC access-point level).

use serde::{Deserialize, Serialize};

/// `<PublicAccessBlockConfiguration><BlockPublicAccess>…</BlockPublicAccess>
/// </PublicAccessBlockConfiguration>` — the shared body for all
/// PublicAccessBlock APIs.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "PublicAccessBlockConfiguration", rename_all = "PascalCase")]
pub struct PublicAccessBlockConfiguration {
    pub block_public_access: bool,
}

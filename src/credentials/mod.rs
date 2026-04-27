//! Credentials providers for authenticating with Alibaba Cloud OSS.
//!
//! This module provides a pluggable credentials system similar to the official
//! Alibaba Cloud SDKs. The core abstraction is the [`CredentialsProvider`] trait,
//! which returns a fresh set of [`Credentials`] on demand.
//!
//! Several built-in providers are available:
//!
//! * [`StaticCredentialsProvider`] – hard-coded AK/SK (+ optional STS token).
//! * [`EnvironmentCredentialsProvider`] – reads AK/SK/STS token from environment
//!   variables (`ALIBABA_CLOUD_ACCESS_KEY_ID`, `OSS_ACCESS_KEY_ID`, …).
//! * [`RrsaCredentialsProvider`] – exchanges an OIDC token file for STS
//!   credentials by calling the STS `AssumeRoleWithOIDC` operation. This is
//!   the provider typically used with ACK RRSA (RAM Roles for Service Accounts).
//! * [`CredentialsChain`] / [`DefaultCredentialsChain`] – walks a list of
//!   providers, returning the first one that yields credentials.
//!
//! Credentials caching & expiration is handled by [`CachingCredentialsProvider`]
//! so inner providers only need to implement the slow "fetch" path.

mod cache;
mod chain;
mod env;
mod rrsa;
mod r#static;

use std::sync::Arc;

use jiff::Timestamp;

pub use self::cache::CachingCredentialsProvider;
pub use self::chain::{CredentialsChain, DefaultCredentialsChain, DefaultCredentialsChainBuilder};
pub use self::env::EnvironmentCredentialsProvider;
pub use self::rrsa::{RrsaCredentialsProvider, RrsaCredentialsProviderBuilder};
pub use self::r#static::StaticCredentialsProvider;
use crate::Result;

/// A single set of credentials used to sign OSS requests.
///
/// When `expiration` is `Some`, the credentials are considered temporary
/// (e.g. obtained from STS) and must be refreshed before that time. Long-term
/// AK/SK pairs should leave `expiration` as `None`.
#[derive(Clone, Debug)]
pub struct Credentials {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: Option<String>,
    pub expiration: Option<Timestamp>,
}

impl Credentials {
    /// Create a permanent AK/SK credential with no expiration.
    pub fn new(access_key_id: impl Into<String>, access_key_secret: impl Into<String>) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            security_token: None,
            expiration: None,
        }
    }

    /// Create a temporary STS credential.
    pub fn with_sts(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        security_token: impl Into<String>,
        expiration: Option<Timestamp>,
    ) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            security_token: Some(security_token.into()),
            expiration,
        }
    }

    /// Returns true if the credential is temporary and has either expired or
    /// will expire within `skew`.
    pub fn is_expired_within(&self, skew: std::time::Duration) -> bool {
        match self.expiration {
            None => false,
            Some(exp) => {
                let now = Timestamp::now();
                let skew = jiff::Span::try_from(skew).unwrap_or_else(|_| jiff::Span::new());
                let deadline = match now.checked_add(skew) {
                    Ok(d) => d,
                    Err(_) => return true,
                };
                deadline >= exp
            },
        }
    }
}

/// Provider that returns fresh [`Credentials`] on demand.
///
/// Implementations must be cheap to clone (usually via `Arc`) and safe to share
/// across threads. Results may be cached internally; callers should call
/// [`get_credentials`] whenever they are about to sign a request rather than
/// keeping the returned value for long.
pub trait CredentialsProvider: Send + Sync + std::fmt::Debug {
    /// Retrieve a fresh set of credentials.
    fn get_credentials(&self) -> impl Future<Output = Result<Credentials>> + Send;
}

/// Type-erased, reference-counted credentials provider used internally by
/// `Client`.
#[derive(Clone)]
pub struct DynCredentialsProvider {
    inner: Arc<dyn DynCredentialsProviderImpl>,
}

impl std::fmt::Debug for DynCredentialsProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt_debug(f)
    }
}

impl DynCredentialsProvider {
    pub fn new<P>(provider: P) -> Self
    where
        P: CredentialsProvider + 'static,
    {
        Self {
            inner: Arc::new(provider),
        }
    }

    pub(crate) async fn get_credentials(&self) -> Result<Credentials> {
        self.inner.get_credentials_dyn().await
    }
}

trait DynCredentialsProviderImpl: Send + Sync + 'static {
    fn get_credentials_dyn(&self)
    -> std::pin::Pin<Box<dyn Future<Output = Result<Credentials>> + Send + '_>>;
    fn fmt_debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<P> DynCredentialsProviderImpl for P
where
    P: CredentialsProvider + 'static,
{
    fn get_credentials_dyn(
        &self,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<Credentials>> + Send + '_>> {
        Box::pin(self.get_credentials())
    }
    fn fmt_debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

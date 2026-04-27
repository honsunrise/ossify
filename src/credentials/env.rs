use crate::credentials::{Credentials, CredentialsProvider};
use crate::{Error, Result};

/// Environment variable credential provider.
///
/// Reads credentials from the following environment variables, in order:
///
/// 1. `ALIBABA_CLOUD_ACCESS_KEY_ID` / `ALIBABA_CLOUD_ACCESS_KEY_SECRET`
///    (+ optional `ALIBABA_CLOUD_SECURITY_TOKEN`). These match the variables
///    used by the official Alibaba Cloud SDKs.
/// 2. `OSS_ACCESS_KEY_ID` / `OSS_ACCESS_KEY_SECRET` (+ optional
///    `OSS_SESSION_TOKEN`). These match the variables used by the OSS v2 SDK
///    family.
///
/// Returns `Error::InvalidCredentials` if neither pair is set.
#[derive(Clone, Debug, Default)]
pub struct EnvironmentCredentialsProvider {
    _priv: (),
}

impl EnvironmentCredentialsProvider {
    pub fn new() -> Self {
        Self { _priv: () }
    }

    fn read() -> Option<Credentials> {
        // Prefer the generic Alibaba Cloud variables.
        if let (Ok(ak), Ok(sk)) = (
            std::env::var("ALIBABA_CLOUD_ACCESS_KEY_ID"),
            std::env::var("ALIBABA_CLOUD_ACCESS_KEY_SECRET"),
        ) && !ak.is_empty()
            && !sk.is_empty()
        {
            let token = std::env::var("ALIBABA_CLOUD_SECURITY_TOKEN")
                .ok()
                .filter(|s| !s.is_empty());
            return Some(match token {
                Some(t) => Credentials::with_sts(ak, sk, t, None),
                None => Credentials::new(ak, sk),
            });
        }

        // Fall back to the OSS-specific variables.
        if let (Ok(ak), Ok(sk)) = (std::env::var("OSS_ACCESS_KEY_ID"), std::env::var("OSS_ACCESS_KEY_SECRET"))
            && !ak.is_empty()
            && !sk.is_empty()
        {
            let token = std::env::var("OSS_SESSION_TOKEN").ok().filter(|s| !s.is_empty());
            return Some(match token {
                Some(t) => Credentials::with_sts(ak, sk, t, None),
                None => Credentials::new(ak, sk),
            });
        }

        None
    }
}

impl CredentialsProvider for EnvironmentCredentialsProvider {
    async fn get_credentials(&self) -> Result<Credentials> {
        Self::read().ok_or(Error::InvalidCredentials)
    }
}

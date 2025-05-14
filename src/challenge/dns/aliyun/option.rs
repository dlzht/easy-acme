use std::{env, time::Duration};

use snafu::ResultExt;

use crate::errors::{EnvironmentVarSnafu, Result};

/// Options for create an [`crate::challenge::dns::AliyunClient`] instance
#[derive(Debug)]
pub struct AliyunClientOption {
  pub(crate) access_key: String,
  pub(crate) access_secret: String,
  pub(crate) proxy: Option<String>,
  pub(crate) timeout: Option<Duration>,
  pub(crate) seed: Option<u64>,
}

impl AliyunClientOption {
  /// Create option with aliyun api key, this key must have privilege to access __AliyunDNSFullAccess__
  pub fn new(access_key: impl Into<String>, access_secret: impl Into<String>) -> Self {
    AliyunClientOption {
      access_key: access_key.into(),
      access_secret: access_secret.into(),
      proxy: None,
      timeout: None,
      seed: None,
    }
  }

  /// Create option from environment variable, variable name for access key is __Ali_Key__ or
  /// __EASY_ACME_ALIYUN_KEY__, and for access secret is __Ali_Secret__ or __EASY_ACME_ALIYUN_SECRET__
  pub fn new_from_env() -> Result<Self> {
    Ok(Self::new(
      Self::env_access_key()?,
      Self::env_access_secret()?,
    ))
  }

  /// Set proxy, for example, `https://127.0.0.1:8080`, `socks5://127.0.0.1:9000`, default is `None`
  pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
    self.proxy = Some(proxy.into());
    self
  }

  /// Set timeout, for example, `Duration::from_secs(5)`, default is `None`
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.timeout = Some(timeout);
    self
  }

  /// Set seed for ['SmallRng'], default is 0
  pub fn seed(mut self, seed: u64) -> Self {
    self.seed = Some(seed);
    self
  }
}

impl AliyunClientOption {
  #[inline]
  pub(crate) fn env_access_key() -> Result<String> {
    let access_key = env::var("Ali_Key")
      .or_else(|_| env::var("EASY_ACME_ALIYUN_KEY"))
      .context(EnvironmentVarSnafu)?;
    Ok(access_key)
  }

  #[inline]
  pub(crate) fn env_access_secret() -> Result<String> {
    let access_secret = env::var("Ali_Secret")
      .or_else(|_| env::var("EASY_ACME_ALIYUN_SECRET"))
      .context(EnvironmentVarSnafu)?;
    Ok(access_secret)
  }
}

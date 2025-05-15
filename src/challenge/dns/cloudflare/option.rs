use std::time::Duration;

use crate::{errors::Result, util::env_single_var};

/// Options for create an [`crate::challenge::dns::cloudflare::CloudflareClient`] instance
#[derive(Debug)]
pub struct CloudflareOption {
  pub(crate) auth: CloudflareAuth,
  pub(crate) proxy: Option<String>,
  pub(crate) timeout: Option<Duration>,
}
//
impl CloudflareOption {
  /// Create option with cloudflare key and email, this key must have privilege to access __DNS Write__
  pub fn new_with_email(key: impl Into<String>, email: impl Into<String>) -> Self {
    let auth = CloudflareAuth::Email(key.into(), email.into());
    CloudflareOption {
      auth,
      proxy: None,
      timeout: None,
    }
  }

  /// Create option with cloudflare token, this token must have privilege to access __DNS Write__
  pub fn new_with_token(token: impl Into<String>) -> Self {
    let auth = CloudflareAuth::Token(token.into());
    CloudflareOption {
      auth,
      proxy: None,
      timeout: None,
    }
  }

  pub fn new_from_env() -> Result<Self> {
    if let Ok(token) = Self::env_auth_token() {
      let auth = CloudflareAuth::Token(token);
      let option = Self {
        auth,
        proxy: None,
        timeout: None,
      };
      return Ok(option);
    }
    let auth = CloudflareAuth::Email(Self::env_auth_key()?, Self::env_auth_email()?);
    Ok(Self {
      auth,
      proxy: None,
      timeout: None,
    })
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
}

impl CloudflareOption {
  #[inline]
  pub(crate) fn env_auth_key() -> Result<String> {
    env_single_var(&["CF_Key", "EASY_ACME_CLOUDFLARE_KEY"])
  }

  #[inline]
  pub(crate) fn env_auth_email() -> Result<String> {
    env_single_var(&["CF_Email", "EASY_ACME_CLOUDFLARE_EMAIL"])
  }

  #[inline]
  pub(crate) fn env_auth_token() -> Result<String> {
    env_single_var(&["CF_Token", "EASY_ACME_CLOUDFLARE_TOKEN"])
  }
}

#[derive(Debug)]
pub(crate) enum CloudflareAuth {
  Token(String),
  Email(String, String),
}

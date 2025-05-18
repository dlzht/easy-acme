use jose_jwk::Jwk;
use p256::ecdsa::signature::hazmat::PrehashSigner;
use p256::ecdsa::SigningKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use snafu::ResultExt;
use crate::util::{base64_url_str, json_serialize};
use crate::errors::{P256SignatureSnafu, Result};

#[derive(Debug, Serialize, Deserialize)]
struct JsonWebHeader {
  alg: String,
  nonce: String,
  url: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  kid: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  jwk: Option<Jwk>,
}

impl JsonWebHeader {
  pub fn new(alg: impl Into<String>, nonce: impl Into<String>, url: impl Into<String>) -> Self {
    Self {
      alg: alg.into(),
      nonce: nonce.into(),
      url: url.into(),
      kid: None,
      jwk: None,
    }
  }

  pub fn kid(mut self, id: impl Into<String>) -> Self {
    self.kid = Some(id.into());
    self
  }

  pub fn jwk(mut self, jwk: Jwk) -> Self {
    self.jwk = Some(jwk);
    self
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonWebObject {
  protected: String,
  payload: String,
  signature: String
}

impl JsonWebObject {
  pub fn new(headers: &JsonWebHeader, payload: &impl Serialize, sign_key: SigningKey) -> Result<Self> {
    let headers = base64_url_str(json_serialize(headers)?);
    let payload = base64_url_str(json_serialize(payload)?);
    let mut hasher = Sha256::default();
    hasher.update(&headers);
    hasher.update(".");
    hasher.update(&payload);
    let sha2sum = hasher.finalize();
    let (signature, _) = sign_key.sign_prehash(&sha2sum)
      .context(P256SignatureSnafu)?;
    let object = Self {
      protected: headers,
      payload,
      signature: signature.to_string(),
    };
    Ok(object)
  }
}
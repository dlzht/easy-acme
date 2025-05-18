use base64ct::Encoding;
use jose_jwk::Jwk;
use p256::ecdsa::SigningKey;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{acme::AcmeDirectory, errors::Result};

// #[derive(Debug)]
pub struct AcmeClient {
  client: Client,
  director: AcmeDirectory,
}

impl AcmeClient {
  pub fn new_with_dir(directory: AcmeDirectory) -> Self {
    Self {
      client: Client::new(),
      director: directory,
    }
  }

  pub async fn new_nonce(&self) -> Result<String> {
    let res = self
      .client
      .head(&self.director.new_nonce)
      .send()
      .await
      .unwrap();
    let nonce = res.headers().get("Replay-Nonce");
    Ok(nonce.unwrap().to_str().unwrap().to_string())
  }
}


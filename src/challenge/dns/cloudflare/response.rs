use serde::Deserialize;

use crate::errors::{PlainTextSnafu, Result};

#[derive(Debug, Deserialize)]
pub struct CloudflareRes {
  // success: bool,
  errors: Vec<FailureData>,

  result: Option<SuccessData>,
}

#[derive(Debug, Deserialize)]
struct SuccessData {
  id: String,
}

#[derive(Debug, Deserialize)]
struct FailureData {
  code: i64,
  message: String,
}

impl CloudflareRes {
  pub fn unwrap_data(self) -> Result<String> {
    if let Some(data) = self.result {
      return Ok(data.id);
    }
    let error = self
      .errors
      .first()
      .map(|e| format!("Cloudflare Error: code: {}, message: {}", e.code, e.message))
      .unwrap_or_else(|| "Cloudflare returns failure but errors is None".to_string());
    PlainTextSnafu { message: error }.fail()
  }
}

use serde::Deserialize;

use crate::errors::{PlainTextSnafu, Result};

#[derive(Debug, Deserialize)]
pub(crate) struct AliyunRes {
  #[allow(dead_code)]
  #[serde(rename = "RequestId")]
  request_id: String,

  #[serde(flatten)]
  data: AliyunData,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum AliyunData {
  Success(SuccessData),
  Failure(FailureData),
}

#[derive(Debug, Deserialize)]
struct SuccessData {
  #[serde(rename = "RecordId")]
  record_id: String,
}

#[derive(Debug, Deserialize)]
struct FailureData {
  #[serde(rename = "Code")]
  code: String,

  #[serde(rename = "Message")]
  message: String,
}

impl AliyunRes {
  pub fn unwrap_data(self) -> Result<String> {
    match self.data {
      AliyunData::Success(success) => Ok(success.record_id),
      AliyunData::Failure(failure) => PlainTextSnafu {
        message: format!(
          "Aliyun Error: code: {}, message: {}",
          failure.code, failure.message,
        ),
      }
      .fail(),
    }
  }
}

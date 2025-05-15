use hmac::{Hmac, Mac};
use http::HeaderValue;
use sha2::Sha256;
use snafu::ResultExt;

use crate::errors::{
  EnvironmentVarSnafu, InvalidHeader, InvalidHeaderSnafu, InvalidHmacKeySnafu, Result,
};

pub fn str_to_header_value(value: impl AsRef<str>) -> Result<HeaderValue> {
  let value = value.as_ref();
  let value = HeaderValue::from_str(value)
    .map_err(|_| InvalidHeader::InvalidValue)
    .with_context(|_| InvalidHeaderSnafu)?;
  Ok(value)
}

type HmacSha256 = Hmac<Sha256>;
pub fn sha2_hmac(key: impl AsRef<[u8]>, data: &[u8]) -> Result<Vec<u8>> {
  let mut hmac = HmacSha256::new_from_slice(key.as_ref()).context(InvalidHmacKeySnafu)?;
  hmac.update(data);
  Ok(hmac.finalize().into_bytes().to_vec())
}

/// __keys.len must > 0__
pub fn env_single_var<'a>(keys: impl AsRef<[&'a str]> + 'a) -> Result<String> {
  let mut keys = keys.as_ref().iter();
  let first_key = keys.next().unwrap();
  let first_err = match std::env::var(first_key) {
    Ok(res) => return Ok(res),
    Err(err) => Err(err).context(EnvironmentVarSnafu),
  };
  for key in keys {
    if let Ok(value) = std::env::var(key) {
      return Ok(value);
    }
  }
  first_err
}

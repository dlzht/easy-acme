use hmac::{Hmac, Mac};
use http::HeaderValue;
use sha2::Sha256;
use snafu::ResultExt;

use crate::errors::{InvalidHeader, InvalidHeaderSnafu, InvalidHmacKeySnafu, Result};

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

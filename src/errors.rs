use std::fmt::{Display, Formatter};
use p256::ecdsa::signature;
use snafu::{Location, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Error: {}", message))]
  PlainText { message: String },

  InvalidHeader {
    #[snafu(source)]
    source: InvalidHeader,

    #[snafu(implicit)]
    location: Location,
  },

  SerializeJson {
    #[snafu(source)]
    source: serde_json::Error,

    #[snafu(implicit)]
    location: Location,
  },

  SerializeUrl {
    #[snafu(source)]
    source: serde_urlencoded::ser::Error,

    #[snafu(implicit)]
    location: Location,
  },

  InvalidHmacKey {
    #[snafu(source)]
    source: crypto_common::InvalidLength,

    #[snafu(implicit)]
    location: Location,
  },

  ReqwestClient {
    #[snafu(source)]
    source: reqwest::Error,

    #[snafu(implicit)]
    location: Location,
  },

  EnvironmentVar {
    #[snafu(source)]
    source: std::env::VarError,

    #[snafu(implicit)]
    location: Location,
  },
  
  P256Signature {
    #[snafu(source)]
    source: signature::Error,

    #[snafu(implicit)]
    location: Location,
  }
}

#[derive(Debug)]
pub enum InvalidHeader {
  InvalidValue,
  ValueNotUTF8,
}

impl Display for InvalidHeader {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let error = match self {
      InvalidHeader::InvalidValue => "invalid header value",
      InvalidHeader::ValueNotUTF8 => "header value is not UTF-8",
    };
    write!(f, "{}", error)
  }
}

impl std::error::Error for InvalidHeader {}

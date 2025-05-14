use hmac::digest::Digest;
use http::{
  header::{AUTHORIZATION, HOST}, HeaderMap,
  HeaderValue,
};
use jiff::{tz::TimeZone, Zoned};
use rand::{prelude::SmallRng, RngCore, SeedableRng};
use serde::Serialize;
use sha2::Sha256;
use snafu::ResultExt;

use crate::{
  challenge::dns::aliyun::{
    option::AliyunClientOption,
    request::{AliyunCreateRecordReq, AliyunDeleteRecordReq},
  },
  errors::{ReqwestClientSnafu, Result, SerializeUrlSnafu},
  util::{sha2_hmac, str_to_header_value},
};
use crate::challenge::dns::aliyun::response::AliyunRes;

#[derive(Debug)]
pub struct AliyunClient {
  access_key: String,
  access_secret: String,
  http_client: reqwest::Client,
  random: SmallRng,
}

const DEFAULT_SEED: u64 = 0;
const DNS_ENDPOINT: &str = "https://alidns.aliyuncs.com/";
const EMPTY_SHA2: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
const NAME_TO_SIGN: &str =
  "host;x-acs-action;x-acs-content-sha256;x-acs-date;x-acs-signature-nonce;x-acs-version";

impl AliyunClient {
  /// Create client with aliyun api key, this key must have privilege to access __AliyunDNSFullAccess__.
  /// If you want to set proxy or timeout, please use [`Self::new_with_option`]
  pub fn new(key: impl Into<String>, secret: impl Into<String>) -> Self {
    AliyunClient {
      access_key: key.into(),
      access_secret: secret.into(),
      http_client: reqwest::Client::new(),
      random: SmallRng::seed_from_u64(DEFAULT_SEED),
    }
  }

  /// Create client from environment variable, variable name for access key is __Ali_Key__ or
  /// __EASY_ACME_ALIYUN_KEY__, and for access secret is __Ali_Secret__ or __EASY_ACME_ALIYUN_SECRET__.
  /// If you want to set proxy or timeout, please use [`Self::new_with_option`]
  pub fn new_from_env() -> Result<Self> {
    let client = AliyunClient {
      access_key: AliyunClientOption::env_access_key()?,
      access_secret: AliyunClientOption::env_access_secret()?,
      http_client: reqwest::Client::new(),
      random: SmallRng::seed_from_u64(DEFAULT_SEED),
    };
    Ok(client)
  }

  /// Create client with option, see [`AliyunClientOption`]
  pub fn new_with_option(option: AliyunClientOption) -> Result<Self> {
    let AliyunClientOption {
      access_key,
      access_secret,
      proxy,
      timeout,
      seed,
    } = option;
    let mut client = reqwest::ClientBuilder::new();
    if let Some(timeout) = timeout {
      client = client.timeout(timeout);
    }
    if let Some(proxy) = proxy {
      let proxy = reqwest::Proxy::all(proxy).context(ReqwestClientSnafu)?;
      client = client.proxy(proxy);
    }
    let client = client.build().context(ReqwestClientSnafu)?;
    let client = AliyunClient {
      access_key,
      access_secret,
      http_client: client,
      random: SmallRng::seed_from_u64(seed.unwrap_or(DEFAULT_SEED)),
    };
    Ok(client)
  }

  /// Create DNS TXT record
  ///
  /// `return`: record id or error
  pub async fn create_record(&mut self, req: AliyunCreateRecordReq<'_>) -> Result<String> {
    self.exec_request("AddDomainRecord", req).await
  }

  /// Delete DNS TXT record
  ///
  /// `return`: record id or error
  pub async fn delete_record(&mut self, req: AliyunDeleteRecordReq<'_>) -> Result<String> {
    self.exec_request("DeleteDomainRecord", req).await
  }
}
impl AliyunClient {
  async fn exec_request<T>(&mut self, action: &str, req: T) -> Result<String>
  where
    T: Serialize,
  {
    let query = serde_urlencoded::to_string(&req).context(SerializeUrlSnafu)?;
    let headers = self.create_headers(action)?;
    let str_to_sign = format!("ACS3-HMAC-SHA256\n{}", self.hash_request(&query, &headers));
    let signature = hex::encode(sha2_hmac(
      self.access_secret.as_bytes(),
      str_to_sign.as_bytes(),
    )?);
    let auth = format!(
      "ACS3-HMAC-SHA256 Credential={},SignedHeaders={},Signature={}",
      self.access_key, NAME_TO_SIGN, signature
    );
    let endpoint = format!("{}?{}", DNS_ENDPOINT, query);
    let res = self
      .http_client
      .get(endpoint)
      .headers(headers)
      .header(AUTHORIZATION, auth)
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<AliyunRes>()
      .await
      .context(ReqwestClientSnafu)?
      .unwrap_data()?;
    Ok(res)
  }

  fn create_headers(&mut self, action: &str) -> Result<HeaderMap> {
    let action_value = str_to_header_value(action)?;
    let hash_value = str_to_header_value(EMPTY_SHA2)?;
    let datetime = Zoned::now()
      .with_time_zone(TimeZone::UTC)
      .strftime("%Y-%m-%dT%H:%M:%SZ")
      .to_string();
    let date_value = str_to_header_value(&datetime)?;
    let nonce = self.random.next_u64();
    let nonce_value = str_to_header_value(&nonce.to_string())?;

    let mut headers = HeaderMap::new();
    headers.insert(HOST, HeaderValue::from_static("alidns.aliyuncs.com"));
    headers.insert("x-acs-action", action_value);
    headers.insert("x-acs-content-sha256", hash_value);
    headers.insert("x-acs-date", date_value);
    headers.insert("x-acs-signature-nonce", nonce_value);
    headers.insert("x-acs-version", HeaderValue::from_static("2015-01-09"));
    Ok(headers)
  }

  fn hash_request(&self, query: &str, headers: &HeaderMap) -> String {
    let mut hasher = Sha256::new();
    hasher.update("GET\n/\n");
    hasher.update(&query);
    hasher.update("\n");
    for (name, value) in headers.iter() {
      hasher.update(name.as_str());
      hasher.update(":");
      hasher.update(value.as_bytes());
      hasher.update("\n");
    }
    hasher.update("\n");
    hasher.update(NAME_TO_SIGN.as_bytes());
    hasher.update("\n");
    hasher.update(EMPTY_SHA2.as_bytes());
    hex::encode(hasher.finalize().as_slice())
  }
}

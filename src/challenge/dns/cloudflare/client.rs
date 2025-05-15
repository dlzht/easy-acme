use http::{HeaderMap, HeaderName, Method, header::AUTHORIZATION};
use reqwest::{Client, ClientBuilder};
use serde::Serialize;
use snafu::ResultExt;

use crate::{
  challenge::dns::cloudflare::{
    CloudflareCreateRecordReq, CloudflareDeleteRecordReq, CloudflareOption, CloudflareRes,
    option::CloudflareAuth,
  },
  errors::{ReqwestClientSnafu, Result},
  util::str_to_header_value,
};

#[derive(Debug)]
pub struct CloudflareClient {
  #[allow(dead_code)]
  auth: CloudflareAuth,
  client: Client,
}

impl CloudflareClient {
  pub fn new(option: CloudflareOption) -> Result<Self> {
    let CloudflareOption {
      auth,
      proxy,
      timeout,
    } = option;
    let mut client = ClientBuilder::new();
    if let Some(timeout) = timeout {
      client = client.timeout(timeout);
    }
    if let Some(proxy) = proxy {
      let proxy = reqwest::Proxy::all(proxy).context(ReqwestClientSnafu)?;
      client = client.proxy(proxy);
    }
    client = client.default_headers(auth_headers(&auth)?);
    let client = client.build().context(ReqwestClientSnafu)?;
    Ok(Self { auth, client })
  }

  pub async fn create_record(&self, req: CloudflareCreateRecordReq<'_>) -> Result<String> {
    let url = format!(
      "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
      req.zone_id
    );
    self.exec_request(Method::POST, &url, &req).await
  }

  pub async fn delete_record(&self, req: CloudflareDeleteRecordReq<'_>) -> Result<String> {
    let url = format!(
      "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
      req.zone_id, req.record_id
    );
    self.exec_request(Method::DELETE, &url, &req).await
  }
}

impl CloudflareClient {
  async fn exec_request(&self, method: Method, url: &str, req: &impl Serialize) -> Result<String> {
    self
      .client
      .request(method, url)
      .json(req)
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<CloudflareRes>()
      .await
      .context(ReqwestClientSnafu)?
      .unwrap_data()
  }
}

fn auth_headers(auth: &CloudflareAuth) -> Result<HeaderMap> {
  let mut headers = HeaderMap::new();
  match &auth {
    CloudflareAuth::Token(token) => {
      let token = if token.starts_with("Bearer ") {
        str_to_header_value(token)?
      } else {
        let token = format!("Bearer {}", token);
        str_to_header_value(token.as_str())?
      };
      headers.insert(AUTHORIZATION, token);
    }
    CloudflareAuth::Email(key, email) => {
      headers.insert(
        HeaderName::from_static("X-Auth-Key"),
        str_to_header_value(key)?,
      );
      headers.insert(
        HeaderName::from_static("X-Auth-Email"),
        str_to_header_value(email)?,
      );
    }
  }
  Ok(headers)
}

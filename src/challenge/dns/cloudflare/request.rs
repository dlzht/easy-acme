use serde::Serialize;

use crate::challenge::dns::RecordType;

#[derive(Debug, Serialize)]
pub struct CloudflareCreateRecordReq<'a> {
  #[serde(skip_serializing)]
  pub(crate) zone_id: &'a str,

  content: &'a str,

  name: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  proxied: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  ttl: Option<u64>,

  #[serde(rename = "type")]
  rtype: RecordType,
}

impl<'a> CloudflareCreateRecordReq<'a> {
  pub fn new(zone_id: &'a str, value: &'a str) -> Self {
    Self {
      zone_id,
      content: value,
      name: "@",
      comment: None,
      proxied: None,
      ttl: None,
      rtype: RecordType::TXT,
    }
  }

  pub fn name(mut self, name: &'a str) -> Self {
    self.name = name;
    self
  }

  pub fn comment(mut self, comment: &'a str) -> Self {
    self.comment = Some(comment);
    self
  }

  pub fn proxied(mut self, proxied: bool) -> Self {
    self.proxied = Some(proxied);
    self
  }

  pub fn ttl(mut self, ttl: u64) -> Self {
    self.ttl = Some(ttl);
    self
  }
}

#[derive(Debug, Serialize)]
pub struct CloudflareDeleteRecordReq<'a> {
  pub(crate) zone_id: &'a str,

  pub(crate) record_id: &'a str,
}

impl<'a> CloudflareDeleteRecordReq<'a> {
  pub fn new(zone_id: &'a str, record_id: &'a str) -> Self {
    Self { zone_id, record_id }
  }
}

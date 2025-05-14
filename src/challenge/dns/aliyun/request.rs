use serde::Serialize;

use crate::challenge::dns::RecordType;

/// Request of [`crate::challenge::dns::AliyunClient::create_record`]
#[derive(Debug, Serialize)]
pub struct AliyunCreateRecordReq<'a> {
  #[serde(rename = "DomainName")]
  domain: &'a str,

  #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
  lang: Option<&'a str>,

  #[serde(rename = "Line")]
  line: Option<&'a str>,

  #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
  priority: Option<i64>,

  #[serde(rename = "RR")]
  rr: &'a str,

  #[serde(rename = "TTL", skip_serializing_if = "Option::is_none")]
  ttl: Option<i64>,

  #[serde(rename = "Type")]
  rtype: RecordType,

  #[serde(rename = "Value")]
  value: &'a str,
}

impl<'a> AliyunCreateRecordReq<'a> {
  pub fn new(domain: &'a str, rr: &'a str, value: &'a str) -> Self {
    AliyunCreateRecordReq {
      domain,
      rr,
      value,
      rtype: RecordType::TXT,
      lang: None,
      ttl: None,
      priority: None,
      line: None,
    }
  }

  pub fn lang(mut self, lang: &'a str) -> Self {
    self.lang = Some(lang);
    self
  }

  pub fn ttl(mut self, ttl: i64) -> Self {
    self.ttl = Some(ttl);
    self
  }

  pub fn priority(mut self, priority: i64) -> Self {
    self.priority = Some(priority);
    self
  }

  pub fn line(mut self, line: &'a str) -> Self {
    self.line = Some(line);
    self
  }
}

/// Request of [`crate::challenge::dns::AliyunClient::delete_record`]
#[derive(Debug, Serialize)]
pub struct AliyunDeleteRecordReq<'a> {
  #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
  lang: Option<&'a str>,

  #[serde(rename = "RecordId")]
  record_id: &'a str,
}

impl<'a> AliyunDeleteRecordReq<'a> {
  pub fn new(record_id: &'a str) -> Self {
    AliyunDeleteRecordReq {
      record_id,
      lang: None,
    }
  }

  pub fn lang(mut self, lang: &'a str) -> Self {
    self.lang = Some(lang);
    self
  }
}

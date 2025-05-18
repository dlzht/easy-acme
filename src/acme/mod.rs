mod client;
mod request;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeDirectory {

  #[serde(rename = "newNonce")]
  pub new_nonce: String,

  #[serde(rename = "newAccount")]
  pub new_account: String,

  #[serde(rename = "newOrder")]
  pub new_order: String,

  #[serde(rename = "newAuthz")]
  pub new_authz: String,

  #[serde(rename = "revokeCert")]
  pub revoke_cert: String,

  #[serde(rename = "keyChange")]
  pub key_change: String,

  pub meta: Option<AcmeDirectoryMetadata>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeDirectoryMetadata {
  #[serde(rename = "termsOfService")]
  pub terms_of_service: String,

  pub website: Option<Vec<String>>,

  #[serde(rename = "caaIdentities")]
  pub caa_identities: Option<Vec<String>>,

  #[serde(rename = "externalAccountRequired")]
  pub external_account_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeAccount {
  pub status: AcmeAccountStatus,

  pub contact: Option<Vec<String>>,

  #[serde(rename = "termsOfServiceAgreed")]
  pub terms_of_service_agreed: Option<bool>,

  #[serde(rename = "externalAccountBinding")]
  pub external_account_binding: Option<Vec<u8>>,

  pub orders: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcmeAccountStatus {
  #[serde(rename = "valid")]
  Valid,

  #[serde(rename = "deactivated")]
  Deactivated,

  #[serde(rename = "revoked")]
  Revoked
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeOrderList {
  pub orders: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeOrder {
  pub status: AcmeOrderStatus,

  pub expire: Option<String>,

  pub wildcard: Option<bool>,

  pub identifiers: Vec<AcmeIdentifier>,

  #[serde(rename = "notBefore")]
  pub not_before: Option<String>,

  #[serde(rename = "notAfter")]
  pub not_after: Option<String>,

  pub error: Option<AcmeError>,

  pub authorizations: Vec<String>,

  pub finalize: String,

  pub certificate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcmeOrderStatus {
  #[serde(rename = "pending")]
  Pending,

  #[serde(rename = "ready")]
  Ready,

  #[serde(rename = "processing")]
  Processing,

  #[serde(rename = "valid")]
  Valid,

  #[serde(rename = "invalid")]
  Invalid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeIdentifier {
  #[serde(rename = "type")]
  pub itype: AcmeIdentifierType,

  pub value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcmeIdentifierType {
  #[serde(rename = "dns")]
  DNS
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeAuthorization {
  pub identifier: AcmeIdentifier,

  pub status: AcmeAuthorizationStatus,

  pub expires: Option<String>,

  pub challenges: Vec<AcmeChallenge>,

  pub wildcard: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeChallenge {
  ctype: AcmeIdentifierType,

  url: String,

  status: AcmeChallengeStatus,

  validated: Option<String>,

  error: Option<AcmeError>,

  token: String
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AcmeHttpChallenge {
//   #[serde(flatten)]
//   base: AcmeBaseChallenge,
//
//   token: String
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct AcmeDnsChallenge {
//
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum AcmeChallengeType {
  #[serde(rename = "http-01")]
  HTTP,

  #[serde(rename = "dns-01")]
  DNS
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcmeChallengeStatus {
  #[serde(rename = "pending")]
  Pending,

  #[serde(rename = "processing")]
  Processing,

  #[serde(rename = "valid")]
  Valid,

  #[serde(rename = "invalid")]
  Invalid,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AcmeAuthorizationStatus {
  #[serde(rename = "pending")]
  Pending,

  #[serde(rename = "valid")]
  Valid,

  #[serde(rename = "invalid")]
  Invalid,

  #[serde(rename = "deactivated")]
  Deactivated,

  #[serde(rename = "expired")]
  Expired,

  #[serde(rename = "revoked")]
  Revoked
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcmeError {
  #[serde(rename = "type")]
  pub etype: String,

  pub title: Option<String>,

  pub status: Option<i64>,

  pub detail: Option<String>,

  pub instance: Option<String>,
}
use serde::{Deserialize, Serialize};

pub mod aliyun;
pub mod cloudflare;

// pub trait DnsChallengeClient {
//   async fn create_record();
//
//   async fn delete_record();
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum RecordType {
  TXT,
}

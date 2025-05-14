use serde::{Deserialize, Serialize};

mod aliyun;
pub use aliyun::{
  AliyunClient, AliyunClientOption, AliyunCreateRecordReq,
  AliyunDeleteRecordReq
};

// pub trait DnsChallengeClient {
//   async fn create_record();
//
//   async fn delete_record();
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum RecordType {
  TXT,
}

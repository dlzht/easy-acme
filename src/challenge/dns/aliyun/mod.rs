//! Implement of [aliyun](https://www.aliyun.com/) DNS challenge
//!
//! Link: <https://help.aliyun.com/zh/dns/api-alidns-2015-01-09-dir-parsing-records>

mod client;
pub use client::AliyunClient;

mod option;
pub use option::AliyunClientOption;

mod request;
pub use request::{AliyunCreateRecordReq, AliyunDeleteRecordReq};

mod response;
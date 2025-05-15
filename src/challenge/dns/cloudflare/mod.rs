//! Implement of [cloudflare](https://www.cloudflare.com/) DNS challenge
//!
//! Link: <https://developers.cloudflare.com/api/resources/dns/subresources/records/>

mod client;
pub use client::CloudflareClient;

mod response;
pub use response::CloudflareRes;

mod request;
pub use request::{CloudflareCreateRecordReq, CloudflareDeleteRecordReq};

mod option;
pub use option::CloudflareOption;

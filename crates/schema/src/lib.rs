//! 配置输入 schema。

pub mod common;
pub mod dns;
pub mod inbound;
pub mod log;
pub mod outbound;
pub mod root;
pub mod route;
pub mod rule_set;

pub use root::Config;

//! 配置输入 schema。

pub mod common;
pub mod defaults;
pub mod dns;
pub mod inbound;
pub mod log;
pub mod macros;
pub mod outbound;
pub mod root;
pub mod route;
pub mod rule_set;

pub use root::ConfigRoot;

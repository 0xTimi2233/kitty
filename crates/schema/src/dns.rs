//! DNS schema。

pub mod action;
pub mod cache;
pub mod rule;
pub mod server;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::defaults;

use self::cache::DnsCache;
use self::rule::DnsRule;
use self::server::DnsServer;

/// DNS 配置。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq,PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub servers: Vec<DnsServer>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<DnsRule>,

    #[serde(default = "defaults::dns_cache")]
    pub cache: DnsCache,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            rules: Vec::new(),
            cache: defaults::dns_cache(),
        }
    }
}

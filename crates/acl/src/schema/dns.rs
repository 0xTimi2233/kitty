//! DNS schema。

pub mod action;
pub mod cache;
pub mod rule;
pub mod server;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::serde_string;
use crate::schema::common::rule::DnsStrategy;
use crate::schema::dns::cache::DnsCacheConfig;
use crate::schema::dns::rule::DnsRule;
use crate::schema::dns::server::DnsServer;

/// DNS 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DnsConfig {
    pub servers: Vec<DnsServer>,
    pub rules: Vec<DnsRule>,

    #[serde(
        rename = "final",
        default,
        deserialize_with = "serde_string::de_opt_trim_lowercase"
    )]
    pub final_server: Option<String>,

    #[serde(default)]
    pub strategy: Option<DnsStrategy>,

    pub cache: DnsCacheConfig,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            rules: Vec::new(),
            final_server: None,
            strategy: None,
            cache: DnsCacheConfig::default(),
        }
    }
}

//! DNS cache schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// DNS cache 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DnsCacheConfig {
    #[serde(default = "crate::schema::defaults::true_value")]
    pub enable: bool,

    #[serde(default = "crate::schema::defaults::dns_cache_capacity")]
    pub capacity: usize,

    pub lazy_cache: LazyCacheConfig,
    pub dump: DnsCacheDumpConfig,
}

impl Default for DnsCacheConfig {
    fn default() -> Self {
        Self {
            enable: true,
            capacity: crate::schema::defaults::dns_cache_capacity(),
            lazy_cache: LazyCacheConfig::default(),
            dump: DnsCacheDumpConfig::default(),
        }
    }
}

/// lazy cache 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct LazyCacheConfig {
    #[serde(default)]
    pub enable: bool,

    #[serde(default = "crate::schema::defaults::lazy_cache_ttl")]
    pub ttl: u64,

    #[serde(default = "crate::schema::defaults::lazy_cache_reply_ttl")]
    pub reply_ttl: u32,
}

impl Default for LazyCacheConfig {
    fn default() -> Self {
        Self {
            enable: false,
            ttl: crate::schema::defaults::lazy_cache_ttl(),
            reply_ttl: crate::schema::defaults::lazy_cache_reply_ttl(),
        }
    }
}

/// DNS cache dump 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DnsCacheDumpConfig {
    #[serde(default)]
    pub enable: bool,

    #[serde(default = "crate::schema::defaults::dns_cache_dump_path")]
    pub path: String,

    #[serde(default = "crate::schema::defaults::dns_cache_dump_interval")]
    pub interval: u64,
}

impl Default for DnsCacheDumpConfig {
    fn default() -> Self {
        Self {
            enable: false,
            path: crate::schema::defaults::dns_cache_dump_path(),
            interval: crate::schema::defaults::dns_cache_dump_interval(),
        }
    }
}

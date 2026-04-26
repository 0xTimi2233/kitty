//! DNS cache schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// DNS cache 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DnsCacheConfig {
    pub enable: bool,

    pub capacity: u32,

    pub lazy_cache: LazyCacheConfig,

    pub dump: DnsCacheDumpConfig,
}

impl Default for DnsCacheConfig {
    fn default() -> Self {
        Self {
            enable: true,
            capacity: 4096,
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
    pub enable: bool,

    pub ttl: u64,

    pub reply_ttl: u32,
}

impl Default for LazyCacheConfig {
    fn default() -> Self {
        Self {
            enable: false,
            ttl: 86_400,
            reply_ttl: 5,
        }
    }
}

/// DNS cache dump 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DnsCacheDumpConfig {
    pub enable: bool,

    pub path: String,

    pub interval: u64,
}

impl Default for DnsCacheDumpConfig {
    fn default() -> Self {
        Self {
            enable: false,
            path: ".cache/dump.db".to_owned(),
            interval: 3_600,
        }
    }
}

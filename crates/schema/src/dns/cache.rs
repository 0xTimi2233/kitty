//! DNS cache schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// DNS cache 配置。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsCache {
    #[serde(default = "default_enable")]
    pub enable: bool,

    #[serde(default = "default_capacity")]
    pub capacity: usize,

    #[serde(default)]
    pub lazy_cache: LazyCache,

    #[serde(default)]
    pub dump: CacheDump,
}

impl Default for DnsCache {
    fn default() -> Self {
        Self {
            enable: default_enable(),
            capacity: default_capacity(),
            lazy_cache: LazyCache::default(),
            dump: CacheDump::default(),
        }
    }
}

/// Lazy cache 配置。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LazyCache {
    #[serde(default)]
    pub enable: bool,

    #[serde(default = "default_lazy_ttl")]
    pub ttl: u64,

    #[serde(default = "default_lazy_reply_ttl")]
    pub reply_ttl: u64,
}

impl Default for LazyCache {
    fn default() -> Self {
        Self {
            enable: false,
            ttl: default_lazy_ttl(),
            reply_ttl: default_lazy_reply_ttl(),
        }
    }
}

/// DNS cache dump 配置。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CacheDump {
    #[serde(default)]
    pub enable: bool,

    #[serde(default = "default_dump_path")]
    pub path: String,

    #[serde(default = "default_dump_interval")]
    pub interval: u64,
}

impl Default for CacheDump {
    fn default() -> Self {
        Self {
            enable: false,
            path: default_dump_path(),
            interval: default_dump_interval(),
        }
    }
}

const fn default_enable() -> bool {
    true
}

const fn default_capacity() -> usize {
    4096
}

const fn default_lazy_ttl() -> u64 {
    86_400
}

const fn default_lazy_reply_ttl() -> u64 {
    5
}

fn default_dump_path() -> String {
    ".cache/dump.db".to_owned()
}

const fn default_dump_interval() -> u64 {
    3600
}

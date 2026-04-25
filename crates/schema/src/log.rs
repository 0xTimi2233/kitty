//! log schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::defaults;

/// 日志配置。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogConfig {
    #[serde(default = "defaults::log_level")]
    pub level: LogLevel,

    #[serde(default = "defaults::log_format")]
    pub format: LogFormat,

    #[serde(default = "defaults::log_timestamp")]
    pub timestamp: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: defaults::log_level(),
            format: defaults::log_format(),
            timestamp: defaults::log_timestamp(),
        }
    }
}

/// 日志级别。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// 日志格式。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
}

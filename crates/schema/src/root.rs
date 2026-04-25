//! root config schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::dns::DnsConfig;
use crate::inbound::InboundConfig;
use crate::log::LogConfig;
use crate::outbound::OutboundConfig;
use crate::route::RouteConfig;

/// Kitty 配置根对象。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq,PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub log: LogConfig,

    #[serde(default)]
    pub dns: DnsConfig,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inbounds: Vec<InboundConfig>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outbounds: Vec<OutboundConfig>,

    #[serde(default)]
    pub route: RouteConfig,
}

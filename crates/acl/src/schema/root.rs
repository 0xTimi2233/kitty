//! root config schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::schema::dns::DnsConfig;
use crate::schema::inbound::Inbound;
use crate::schema::log::LogConfig;
use crate::schema::outbound::Outbound;
use crate::schema::route::RouteConfig;

/// 根配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ConfigRoot {
    pub log: LogConfig,
    pub dns: DnsConfig,
    pub inbounds: Vec<Inbound>,
    pub outbounds: Vec<Outbound>,
    pub route: RouteConfig,
}

impl Default for ConfigRoot {
    fn default() -> Self {
        Self {
            log: LogConfig::default(),
            dns: DnsConfig::default(),
            inbounds: Vec::new(),
            outbounds: Vec::new(),
            route: RouteConfig::default(),
        }
    }
}

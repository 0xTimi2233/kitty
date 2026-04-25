//! inbound schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::defaults;
use crate::define_inbound_struct;

/// inbound 配置。
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum InboundConfig {
    Direct(DirectInbound),
    Socks(SocksInbound),
    Vless(VlessInbound),
    Dns(DnsInbound),
    Tc(TcInbound),
}

define_inbound_struct!(pub struct DirectInbound, listen_port_default = "defaults::socks_listen_port" {});

define_inbound_struct!(pub struct SocksInbound, listen_port_default = "defaults::socks_listen_port" {
    #[serde(default = "defaults::socks_version")]
    pub version: u8,
});

define_inbound_struct!(pub struct VlessInbound, listen_port_default = "defaults::default_remote_port" {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<VlessInboundUser>,
});

define_inbound_struct!(pub struct DnsInbound, listen_port_default = "defaults::dns_listen_port" {});

#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TcInbound {
    #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
    pub tag: String,

    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub interface: String,
}

/// VLESS inbound user。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VlessInboundUser {
    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub uuid: String,

    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim")]
    pub name: Option<String>,
}

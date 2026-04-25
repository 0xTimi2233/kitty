//! outbound schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::defaults;
use crate::define_outbound_struct;

/// outbound 配置。
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum OutboundConfig {
    Direct(DirectOutbound),
    Socks(SocksOutbound),
    Vless(VlessOutbound),
}

/// outbound network。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundNetwork {
    Tcp,
    Udp,
    TcpUdp,
}

define_outbound_struct!(pub struct DirectOutbound {
    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim")]
    pub override_address: Option<String>,

    #[serde(default)]
    pub override_port: Option<u16>,
});

define_outbound_struct!(pub struct SocksOutbound {
    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub server: String,

    #[serde(default = "defaults::socks_listen_port")]
    pub server_port: u16,

    #[serde(default = "defaults::socks_version")]
    pub version: u8,

    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim")]
    pub username: Option<String>,

    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim")]
    pub password: Option<String>,
});

define_outbound_struct!(pub struct VlessOutbound {
    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub server: String,

    #[serde(default = "defaults::default_remote_port")]
    pub server_port: u16,

    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub uuid: String,

    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
    pub flow: Option<String>,

    #[serde(default)]
    pub tls: bool,
});

//! outbound schema。

use serde::{Deserialize, Serialize};

use crate::helper::serde_string;
use crate::schema::common::rule::{Network, SocksVersion, VlessFlow, VlessPacketEncoding};
use crate::schema::common::shared::{MultiplexConfig, TlsConfig, UdpOverTcp, V2RayTransport};
use crate::schema::defaults;
use crate::schema::macros::outbound::define_outbound_struct;

/// outbound。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Outbound {
    Direct(DirectOutbound),
    Socks(SocksOutbound),
    Vless(VlessOutbound),
}

define_outbound_struct! {
    pub struct DirectOutbound {
        extra {}
    }
}

define_outbound_struct! {
    pub struct SocksOutbound {
        extra {
            #[serde(deserialize_with = "serde_string::de_trim_lowercase")]
            pub server: String,

            pub server_port: u16,

            #[serde(default)]
            pub version: SocksVersion,

            #[serde(default, deserialize_with = "serde_string::de_opt_trim")]
            pub username: Option<String>,

            #[serde(default, deserialize_with = "serde_string::de_opt_trim")]
            pub password: Option<String>,

            #[serde(default = "defaults::outbound_network", deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub network: Vec<Network>,

            #[serde(default)]
            pub udp_over_tcp: Option<UdpOverTcp>,
        }
    }
}

define_outbound_struct! {
    pub struct VlessOutbound {
        extra {
            #[serde(deserialize_with = "serde_string::de_trim_lowercase")]
            pub server: String,

            pub server_port: u16,

            #[serde(deserialize_with = "serde_string::de_trim")]
            pub uuid: String,

            #[serde(default)]
            pub flow: Option<VlessFlow>,

            #[serde(default = "defaults::outbound_network", deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub network: Vec<Network>,

            #[serde(default)]
            pub tls: Option<TlsConfig>,

            #[serde(default = "defaults::vless_packet_encoding")]
            pub packet_encoding: VlessPacketEncoding,

            #[serde(default)]
            pub multiplex: Option<MultiplexConfig>,

            #[serde(default)]
            pub transport: Option<V2RayTransport>,
        }
    }
}

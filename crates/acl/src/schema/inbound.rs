//! inbound schema。

use crate::schema::defaults;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::serde_string;
use crate::schema::common::rule::Network;
use crate::schema::macros::inbound::define_inbound_struct;

/// inbound。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Inbound {
    Direct(DirectInbound),
    Socks(SocksInbound),
    Vless(VlessInbound),
    Dns(DnsInbound),
    Tc(TcInbound),
}

define_inbound_struct! {
    pub struct DirectInbound {
        extra {
            #[serde(default = "defaults::bound_network", deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub network: Vec<Network>,
        }
    }
}

define_inbound_struct! {
    pub struct DnsInbound {
        extra {
            #[serde(default = "defaults::bound_network", skip_deserializing)]
            pub network: Vec<Network>,
        }
    }
}

define_inbound_struct! {
    pub struct TcInbound {
        extra {
            #[serde(default = "defaults::bound_network", skip_deserializing)]
            pub network: Vec<Network>,
        }
    }
}

define_inbound_struct! {
    pub struct SocksInbound {
        extra {
            #[serde(default)]
            pub users: Vec<SocksUser>,
        }
    }
}

define_inbound_struct! {
    pub struct VlessInbound {
        extra {
            pub users: Vec<VlessUser>,
        }
    }
}

/// SOCKS 用户。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SocksUser {
    #[serde(deserialize_with = "serde_string::de_trim")]
    pub username: String,

    #[serde(deserialize_with = "serde_string::de_trim")]
    pub password: String,
}

/// VLESS 用户。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VlessUser {
    #[serde(deserialize_with = "serde_string::de_trim")]
    pub uuid: String,

    #[serde(default, deserialize_with = "serde_string::de_opt_trim")]
    pub name: Option<String>,
}

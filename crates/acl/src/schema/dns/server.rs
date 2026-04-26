//! DNS server schema。

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::helper::serde_string;
use crate::schema::common::shared::TlsConfig;
use crate::schema::defaults;
use crate::schema::macros::dns_server::define_dns_server_struct;

/// DNS server。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DnsServer {
    Tcp(TcpDnsServer),
    Udp(UdpDnsServer),
    Tls(TlsDnsServer),
    Quic(QuicDnsServer),
    Https(HttpsDnsServer),
    H3(H3DnsServer),
}

define_dns_server_struct! {
    pub struct TcpDnsServer {
        default_port = "crate::schema::defaults::dns_port";
        extra {}
    }
}

define_dns_server_struct! {
    pub struct UdpDnsServer {
        default_port = "crate::schema::defaults::dns_port";
        extra {}
    }
}

define_dns_server_struct! {
    pub struct TlsDnsServer {
        default_port = "crate::schema::defaults::tls_dns_port";
        extra {
            #[serde(default)]
            pub tls: Option<TlsConfig>,
        }
    }
}

define_dns_server_struct! {
    pub struct QuicDnsServer {
        default_port = "crate::schema::defaults::tls_dns_port";
        extra {
            #[serde(default)]
            pub tls: Option<TlsConfig>,
        }
    }
}

define_dns_server_struct! {
    pub struct HttpsDnsServer {
        default_port = "crate::schema::defaults::https_dns_port";
        extra {
            #[serde(default = "defaults::doh_path", deserialize_with = "serde_string::de_trim")]
            pub path: String,

            #[serde(default)]
            pub headers: BTreeMap<String, String>,

            #[serde(default)]
            pub tls: Option<TlsConfig>,
        }
    }
}

define_dns_server_struct! {
    pub struct H3DnsServer {
        default_port = "crate::schema::defaults::https_dns_port";
        extra {
            #[serde(default = "defaults::doh_path", deserialize_with = "serde_string::de_trim")]
            pub path: String,

            #[serde(default)]
            pub headers: BTreeMap<String, String>,

            #[serde(default)]
            pub tls: Option<TlsConfig>,
        }
    }
}

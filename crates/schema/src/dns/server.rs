//! DNS server schema。

use serde::{Deserialize, Serialize};

use crate::common::defaults;
use crate::define_dns_server_struct;

/// DNS server。
#[derive(Clone, Debug, Eq,PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum DnsServer {
    Tcp(TcpDnsServer),
    Udp(UdpDnsServer),
    Tls(TlsDnsServer),
    Quic(QuicDnsServer),
    Https(HttpsDnsServer),
    H3(H3DnsServer),
}

/// DNS 查询策略。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DnsStrategy {
    PreferIpv4,
    PreferIpv6,
    Ipv4Only,
    Ipv6Only,
}

define_dns_server_struct!(pub struct TcpDnsServer, port_default = "defaults::dns_port" {});
define_dns_server_struct!(pub struct UdpDnsServer, port_default = "defaults::dns_port" {});
define_dns_server_struct!(pub struct TlsDnsServer, port_default = "defaults::dns_tls_port" {});
define_dns_server_struct!(pub struct QuicDnsServer, port_default = "defaults::dns_tls_port" {});

define_dns_server_struct!(pub struct HttpsDnsServer, port_default = "defaults::dns_https_port" {
    #[serde(default = "defaults::doh_path", deserialize_with = "crate::common::serde_helper::de_trim")]
    pub path: String,
});

define_dns_server_struct!(pub struct H3DnsServer, port_default = "defaults::dns_https_port" {
    #[serde(default = "defaults::doh_path", deserialize_with = "crate::common::serde_helper::de_trim")]
    pub path: String,
});

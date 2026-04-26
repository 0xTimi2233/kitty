//! 配置默认值。

use std::time::Duration;

use crate::schema::common::rule::{Network, SocksVersion, VlessPacketEncoding};
/// 默认 true。
pub const fn true_value() -> bool {
    true
}

/// 默认 DNS 端口。
pub const fn dns_port() -> u16 {
    53
}

/// 默认 TLS DNS 端口。
pub const fn tls_dns_port() -> u16 {
    853
}

/// 默认 HTTPS DNS 端口。
pub const fn https_dns_port() -> u16 {
    443
}

/// 默认 DoH path。
pub fn doh_path() -> String {
    "/dns-query".to_owned()
}

/// 默认 TCP keepalive。
pub const fn tcp_keep_alive() -> Duration {
    Duration::from_secs(300)
}

/// 默认 TCP keepalive interval。
pub const fn tcp_keep_alive_interval() -> Duration {
    Duration::from_secs(75)
}

/// 默认 UDP timeout。
pub const fn udp_timeout() -> Duration {
    Duration::from_secs(300)
}

/// 默认 connect timeout。
pub const fn connect_timeout() -> Duration {
    Duration::from_secs(5)
}

/// 默认 outbound network。
pub fn bound_network() -> Vec<Network> {
    vec![Network::Tcp, Network::Udp]
}

/// 默认 SOCKS 版本。
pub const fn socks_version() -> SocksVersion {
    SocksVersion::V5
}

/// 默认 VLESS packet encoding。
pub const fn vless_packet_encoding() -> VlessPacketEncoding {
    VlessPacketEncoding::Xudp
}

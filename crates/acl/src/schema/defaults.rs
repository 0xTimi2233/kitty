//! 配置默认值。

use std::time::Duration;

use crate::schema::common::rule::{Network, VlessPacketEncoding};

/// 默认 true。
pub const fn true_value() -> bool {
    true
}

/// 默认日志 timestamp。
pub const fn log_timestamp() -> bool {
    true
}

/// 默认 DNS cache 容量。
pub const fn dns_cache_capacity() -> usize {
    4096
}

/// 默认 lazy cache TTL。
pub const fn lazy_cache_ttl() -> u64 {
    86_400
}

/// 默认 lazy cache 响应 TTL。
pub const fn lazy_cache_reply_ttl() -> u32 {
    5
}

/// 默认 DNS cache dump 间隔。
pub const fn dns_cache_dump_interval() -> u64 {
    3_600
}

/// 默认 DNS cache dump 路径。
pub fn dns_cache_dump_path() -> String {
    ".cache/dump.db".to_owned()
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
pub fn outbound_network() -> Vec<Network> {
    vec![Network::Tcp, Network::Udp]
}

/// 默认 VLESS packet encoding。
pub const fn vless_packet_encoding() -> VlessPacketEncoding {
    VlessPacketEncoding::Xudp
}

/// 默认 UDP over TCP version。
pub const fn udp_over_tcp_version() -> u8 {
    2
}

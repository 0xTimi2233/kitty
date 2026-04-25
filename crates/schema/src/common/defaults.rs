//! schema 默认值。

use std::time::Duration;

use crate::common::duration::RefreshInterval;
use crate::dns::cache::DnsCache;
use crate::log::{LogFormat, LogLevel};
use crate::outbound::OutboundNetwork;
use crate::rule_set::RuleSetFormat;

/// 默认日志级别。
pub const fn log_level() -> LogLevel {
    LogLevel::Warn
}

/// 默认日志格式。
pub const fn log_format() -> LogFormat {
    LogFormat::Text
}

/// 默认输出 timestamp。
pub const fn log_timestamp() -> bool {
    true
}

/// 默认 DNS cache。
pub fn dns_cache() -> DnsCache {
    DnsCache::default()
}

/// DNS UDP/TCP 默认端口。
pub const fn dns_port() -> u16 {
    53
}

/// TLS/QUIC 默认端口。
pub const fn dns_tls_port() -> u16 {
    853
}

/// HTTPS/H3 默认端口。
pub const fn dns_https_port() -> u16 {
    443
}

/// DoH 默认路径。
pub fn doh_path() -> String {
    "/dns-query".to_owned()
}

/// inbound 默认监听地址。
pub fn listen_addr() -> String {
    "127.0.0.1".to_owned()
}

/// SOCKS inbound 默认端口。
pub const fn socks_listen_port() -> u16 {
    1080
}

/// DNS inbound 默认端口。
pub const fn dns_listen_port() -> u16 {
    53
}

/// 默认连接超时。
pub const fn connect_timeout() -> Duration {
    Duration::from_secs(5)
}

/// 默认 UDP session 超时。
pub const fn udp_timeout() -> Duration {
    Duration::from_secs(300)
}

/// 默认 outbound network。
pub const fn outbound_network() -> OutboundNetwork {
    OutboundNetwork::TcpUdp
}

/// 默认 SOCKS 版本。
pub const fn socks_version() -> u8 {
    5
}

/// 默认远程端口。
pub const fn default_remote_port() -> u16 {
    443
}

/// 默认 route final。
pub fn route_final() -> String {
    "direct".to_owned()
}

/// 默认 rule_set format。
pub const fn rule_set_format() -> RuleSetFormat {
    RuleSetFormat::Source
}

/// 默认 rule_set 自动刷新间隔。
pub const fn rule_set_update_interval() -> RefreshInterval {
    RefreshInterval::Interval(Duration::from_secs(86_400))
}

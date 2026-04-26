//! 规则公共类型。

use serde::{Deserialize, Serialize};

/// 逻辑规则类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Logical,
}

/// 逻辑规则模式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogicalMode {
    And,
    Or,
}

/// 网络类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Tcp,
    Udp,
}

/// IP 版本。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IpVersion {
    Ipv4,
    Ipv6,
}

/// DNS 查询类型。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryType {
    Number(u16),
    Name(String),
}

/// DNS strategy。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DnsStrategy {
    PreferIpv4,
    PreferIpv6,
    Ipv4Only,
    Ipv6Only,
}

/// SOCKS version。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocksVersion {
    #[serde(rename = "4")]
    V4,
    #[serde(rename = "4a")]
    V4A,
    #[serde(rename = "5")]
    V5,
}

impl Default for SocksVersion {
    fn default() -> Self {
        Self::V5
    }
}

/// VLESS flow。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VlessFlow {
    #[serde(rename = "xtls-rprx-vision")]
    XtlsRprxVision,
}

/// VLESS packet encoding。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VlessPacketEncoding {
    #[serde(rename = "")]
    None,
    Packetaddr,
    Xudp,
}

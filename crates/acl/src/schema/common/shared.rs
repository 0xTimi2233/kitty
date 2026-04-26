//! 共享 schema 类型。

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::serde_string;
use crate::schema::common::rule::DnsStrategy;

/// 域名解析器引用或对象配置。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DomainResolver {
    Tag(#[serde(deserialize_with = "serde_string::de_trim_lowercase")] String),
    Options(DomainResolverOptions),
}

/// 域名解析器对象配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainResolverOptions {
    #[serde(deserialize_with = "serde_string::de_trim_lowercase")]
    pub server: String,

    #[serde(default)]
    pub strategy: Option<DnsStrategy>,

    #[serde(default)]
    pub rewrite_ttl: Option<u32>,
}

/// TLS 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct TlsConfig {
    #[serde(default = "crate::schema::defaults::true_value")]
    pub enabled: bool,

    #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
    pub server_name: Option<String>,

    #[serde(default)]
    pub insecure: bool,

    #[serde(default, deserialize_with = "serde_string::de_one_or_many_trim")]
    pub alpn: Vec<String>,

    #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
    pub min_version: Option<String>,

    #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
    pub max_version: Option<String>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            server_name: None,
            insecure: false,
            alpn: Vec::new(),
            min_version: None,
            max_version: None,
        }
    }
}

/// Multiplex 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MultiplexConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
    pub protocol: Option<String>,

    #[serde(default)]
    pub max_connections: u32,

    #[serde(default)]
    pub min_streams: u32,

    #[serde(default)]
    pub max_streams: u32,

    #[serde(default)]
    pub padding: bool,
}

impl Default for MultiplexConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            protocol: None,
            max_connections: 0,
            min_streams: 0,
            max_streams: 0,
            padding: false,
        }
    }
}

/// V2Ray transport。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum V2RayTransport {
    Ws(WebSocketTransport),
    Http(HttpTransport),
    Grpc(GrpcTransport),
}

/// WebSocket transport。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WebSocketTransport {
    #[serde(default, deserialize_with = "serde_string::de_trim")]
    pub path: String,

    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

impl Default for WebSocketTransport {
    fn default() -> Self {
        Self {
            path: String::new(),
            headers: BTreeMap::new(),
        }
    }
}

/// HTTP transport。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HttpTransport {
    #[serde(
        default,
        deserialize_with = "serde_string::de_one_or_many_trim_lowercase"
    )]
    pub host: Vec<String>,

    #[serde(default, deserialize_with = "serde_string::de_trim")]
    pub path: String,

    #[serde(default, deserialize_with = "serde_string::de_trim")]
    pub method: String,

    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

impl Default for HttpTransport {
    fn default() -> Self {
        Self {
            host: Vec::new(),
            path: String::new(),
            method: String::new(),
            headers: BTreeMap::new(),
        }
    }
}

/// gRPC transport。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GrpcTransport {
    #[serde(default, deserialize_with = "serde_string::de_trim")]
    pub service_name: String,

    #[serde(default)]
    pub idle_timeout: u64,

    #[serde(default)]
    pub ping_timeout: u64,

    #[serde(default)]
    pub permit_without_stream: bool,
}

impl Default for GrpcTransport {
    fn default() -> Self {
        Self {
            service_name: String::new(),
            idle_timeout: 0,
            ping_timeout: 0,
            permit_without_stream: false,
        }
    }
}

/// UDP over TCP。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UdpOverTcp {
    Bool(bool),
    Options(UdpOverTcpOptions),
}

/// UDP over TCP options。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct UdpOverTcpOptions {
    #[serde(default = "crate::schema::defaults::true_value")]
    pub enabled: bool,

    #[serde(default = "crate::schema::defaults::udp_over_tcp_version")]
    pub version: u8,
}

impl Default for UdpOverTcpOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            version: crate::schema::defaults::udp_over_tcp_version(),
        }
    }
}

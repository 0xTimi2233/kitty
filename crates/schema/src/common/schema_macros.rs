//! schema struct 生成宏。

/// 定义 DNS server struct。
#[macro_export]
macro_rules! define_dns_server_struct {
    ($vis:vis struct $name:ident, port_default = $port_default:literal { $($extra:tt)* }) => {
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        $vis struct $name {
            #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
            pub tag: String,

            #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
            pub server: String,

            #[serde(default = $port_default)]
            pub server_port: u16,

            #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
            pub detour: Option<String>,

            #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
            pub domain_resolver: Option<String>,

            #[serde(default)]
            pub strategy: Option<$crate::dns::server::DnsStrategy>,

            $($extra)*
        }
    };
}

/// 定义 inbound struct。
#[macro_export]
macro_rules! define_inbound_struct {
    ($vis:vis struct $name:ident, listen_port_default = $port_default:literal { $($extra:tt)* }) => {
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        $vis struct $name {
            #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
            pub tag: String,

            #[serde(default = "crate::common::defaults::listen_addr", deserialize_with = "crate::common::serde_helper::de_trim")]
            pub listen: String,

            #[serde(default = $port_default)]
            pub listen_port: u16,

            $($extra)*
        }
    };
}

/// 定义 outbound struct。
#[macro_export]
macro_rules! define_outbound_struct {
    ($vis:vis struct $name:ident { $($extra:tt)* }) => {
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        $vis struct $name {
            #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
            pub tag: String,

            #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
            pub detour: Option<String>,

            #[serde(default = "crate::common::defaults::connect_timeout", with = "humantime_serde")]
            pub connect_timeout: std::time::Duration,

            #[serde(default)]
            pub tcp_fast_open: bool,

            #[serde(default)]
            pub udp_fragment: bool,

            #[serde(default = "crate::common::defaults::outbound_network")]
            pub network: $crate::outbound::OutboundNetwork,

            $($extra)*
        }
    };
}

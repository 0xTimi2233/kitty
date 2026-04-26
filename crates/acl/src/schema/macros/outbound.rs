//! outbound schema 宏。

macro_rules! define_outbound_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            extra { $($extra_field:tt)* }
        }
    ) => {
        $(#[$meta])*
        #[serde_with::skip_serializing_none]
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        $vis struct $name {
            #[serde(deserialize_with = "crate::helper::serde_string::de_trim_lowercase")]
            pub tag: String,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_opt_trim_lowercase")]
            pub detour: Option<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_opt_trim")]
            pub bind_interface: Option<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_opt_trim_lowercase")]
            pub inet4_bind_address: Option<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_opt_trim_lowercase")]
            pub inet6_bind_address: Option<String>,

            #[serde(default)]
            pub bind_address_no_port: bool,

            #[serde(default)]
            pub routing_mark: u32,

            #[serde(default)]
            pub reuse_addr: bool,

            #[serde(default = "crate::schema::defaults::connect_timeout", with = "humantime_serde")]
            pub connect_timeout: std::time::Duration,

            #[serde(default)]
            pub tcp_fast_open: bool,

            #[serde(default)]
            pub tcp_multi_path: bool,

            #[serde(default)]
            pub disable_tcp_keep_alive: bool,

            #[serde(default = "crate::schema::defaults::tcp_keep_alive", with = "humantime_serde")]
            pub tcp_keep_alive: std::time::Duration,

            #[serde(default = "crate::schema::defaults::tcp_keep_alive_interval", with = "humantime_serde")]
            pub tcp_keep_alive_interval: std::time::Duration,

            #[serde(default)]
            pub udp_fragment: bool,

            #[serde(default)]
            pub domain_resolver: Option<$crate::schema::common::shared::DomainResolver>,

            $($extra_field)*
        }
    };
}

pub(crate) use define_outbound_struct;

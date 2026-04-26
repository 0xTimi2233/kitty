//! inbound schema 宏。

macro_rules! define_inbound_struct {
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

            #[serde(deserialize_with = "crate::helper::serde_string::de_trim")]
            pub listen: String,

            #[serde(default)]
            pub listen_port: u16,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_opt_trim")]
            pub bind_interface: Option<String>,

            #[serde(default)]
            pub routing_mark: u32,

            #[serde(default)]
            pub reuse_addr: bool,

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

            #[serde(default = "crate::schema::defaults::udp_timeout", with = "humantime_serde")]
            pub udp_timeout: std::time::Duration,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_opt_trim_lowercase")]
            pub detour: Option<String>,

            $($extra_field)*
        }
    };
}

pub(crate) use define_inbound_struct;

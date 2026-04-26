//! DNS server schema 宏。

macro_rules! define_dns_server_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            default_port = $default_port:literal;
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

            #[serde(deserialize_with = "crate::helper::serde_string::de_trim_lowercase")]
            pub server: String,

            #[serde(default = $default_port)]
            pub server_port: u16,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_opt_trim_lowercase")]
            pub detour: Option<String>,

            #[serde(default)]
            pub domain_resolver: Option<$crate::schema::common::shared::DomainResolver>,

            #[serde(default)]
            pub strategy: Option<$crate::schema::common::rule::DnsStrategy>,

            $($extra_field)*
        }
    };
}

pub(crate) use define_dns_server_struct;

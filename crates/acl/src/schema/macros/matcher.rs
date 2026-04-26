//! matcher schema 宏。

macro_rules! define_matcher_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            extra { $($extra_field:tt)* }
            tail { $($tail_field:tt)* }
        }
    ) => {
        $(#[$meta])*
        #[serde_with::skip_serializing_none]
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        $vis struct $name {
            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim_lowercase")]
            pub inbound: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub ip_version: Vec<$crate::schema::common::rule::IpVersion>,

            #[serde(default, deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub network: Vec<$crate::schema::common::rule::Network>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim_lowercase")]
            pub domain: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_domain_suffix")]
            pub domain_suffix: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim_lowercase")]
            pub domain_keyword: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub domain_regex: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim_lowercase")]
            pub ip_cidr: Vec<String>,

            #[serde(default)]
            pub ip_is_private: bool,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim_lowercase")]
            pub source_ip_cidr: Vec<String>,

            #[serde(default)]
            pub source_ip_is_private: bool,

            #[serde(default, deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub port: Vec<u16>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub port_range: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub source_port: Vec<u16>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub source_port_range: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub source_mac_address: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub process_name: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub process_path: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub process_path_regex: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim")]
            pub user: Vec<String>,

            #[serde(default, deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub user_id: Vec<u32>,

            #[serde(default, deserialize_with = "crate::helper::serde_string::de_one_or_many_trim_lowercase")]
            pub rule_set: Vec<String>,

            #[serde(default)]
            pub invert: bool,

            $($extra_field)*
            $($tail_field)*
        }
    };
}

pub(crate) use define_matcher_struct;

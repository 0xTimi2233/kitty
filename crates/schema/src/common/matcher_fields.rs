//! matcher 字段宏。

/// 定义带公共 matcher 字段的 rule struct。
#[macro_export]
macro_rules! define_matcher_struct {
    (
        $vis:vis struct $name:ident {
            extra { $($extra:tt)* }
            tail { $($tail:tt)* }
        }
    ) => {
        #[serde_with::skip_serializing_none]
        #[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        $vis struct $name {
            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_trim_lowercase",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub domain: Vec<String>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_domain_suffix",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub domain_suffix: Vec<String>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_trim_lowercase",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub domain_keyword: Vec<String>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_trim",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub domain_regex: Vec<String>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub ip_cidr: Vec<$crate::common::types::IpCidr>,

            #[serde(default)]
            pub ip_is_private: bool,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub source_ip_cidr: Vec<$crate::common::types::IpCidr>,

            #[serde(default)]
            pub source_ip_is_private: bool,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub port: Vec<u16>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub port_range: Vec<$crate::common::types::PortRange>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub source_port: Vec<u16>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub source_port_range: Vec<$crate::common::types::PortRange>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_trim_lowercase",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub process_name: Vec<String>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_trim",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub process_path: Vec<String>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_trim_lowercase",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub user: Vec<String>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub user_id: Vec<u32>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub source_mac: Vec<macaddr::MacAddr6>,

            #[serde(
                default,
                deserialize_with = "crate::common::serde_helper::de_one_or_many_trim_lowercase",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub rule_set: Vec<String>,

            #[serde(default)]
            pub invert: bool,

            $($extra)*
            $($tail)*
        }
    };
}

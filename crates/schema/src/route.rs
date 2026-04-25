//! route schema。

pub mod action;
pub mod rule;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::defaults;
use crate::rule_set::RuleSetConfig;

use self::rule::RouteRule;

/// route 配置。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RouteRule>,

    #[serde(default = "defaults::route_final", rename = "final", deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
    pub final_outbound: String,

    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
    pub default_domain_resolver: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_set: Vec<RuleSetConfig>,
}

impl Default for RouteConfig {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            final_outbound: defaults::route_final(),
            default_domain_resolver: None,
            rule_set: Vec::new(),
        }
    }
}

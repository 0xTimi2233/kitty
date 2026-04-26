//! route schema。

pub mod action;
pub mod rule;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::serde_string;
use crate::schema::common::shared::DomainResolver;
use crate::schema::route::rule::RouteRule;
use crate::schema::rule_set::RuleSet;

/// route 配置。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteConfig {
    #[serde(default)]
    pub rules: Vec<RouteRule>,

    #[serde(default)]
    pub rule_set: Vec<RuleSet>,

    #[serde(
        rename = "final",
        default,
        deserialize_with = "serde_string::de_opt_trim_lowercase"
    )]
    pub final_outbound: Option<String>,

    #[serde(default)]
    pub default_domain_resolver: Option<DomainResolver>,
}

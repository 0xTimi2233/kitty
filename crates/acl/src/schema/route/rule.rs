//! route rule schema。

use serde::{Deserialize, Serialize};

use crate::helper::serde_string;
use crate::schema::common::rule::{LogicalMode, RuleType};
use crate::schema::macros::matcher::define_matcher_struct;
use crate::schema::route::action::RouteRuleAction;

/// route rule。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteRule {
    Logical(Box<RouteLogicalRule>),
    Leaf(Box<RouteLeafRule>),
}

/// route logical rule。
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteLogicalRule {
    #[serde(rename = "type")]
    pub rule_type: RuleType,

    pub mode: LogicalMode,

    #[serde(default)]
    pub rules: Vec<RouteSubRule>,

    #[serde(default)]
    pub action: Option<RouteRuleAction>,

    #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
    pub outbound: Option<String>,
}

define_matcher_struct! {
    pub struct RouteLeafRule {
        extra {}
        tail {
            #[serde(default)]
            pub action: Option<RouteRuleAction>,

            #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
            pub outbound: Option<String>,
        }
    }
}

define_matcher_struct! {
    pub struct RouteSubRule {
        extra {}
        tail {}
    }
}

//! DNS rule schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::define_matcher_struct;
use crate::dns::action::DnsRuleAction;

/// DNS rule。
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsRule {
    Logical(DnsLogicalRule),
    Default(DnsDefaultRule),
}

define_matcher_struct! {
    pub struct DnsDefaultRule {
        extra {}
        tail {
            #[serde(default)]
            pub action: Option<DnsRuleAction>,

            #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
            pub server: Option<String>,
        }
    }
}

/// DNS logical rule。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsLogicalRule {
    #[serde(rename = "type")]
    pub rule_type: LogicalRuleType,

    pub mode: LogicalMode,

    #[serde(default)]
    pub invert: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<DnsRule>,

    #[serde(default)]
    pub action: Option<DnsRuleAction>,

    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
    pub server: Option<String>,
}

/// Logical rule type。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogicalRuleType {
    Logical,
}

/// Logical mode。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogicalMode {
    And,
    Or,
}

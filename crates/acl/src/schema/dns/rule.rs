//! DNS rule schema。

use serde::{Deserialize, Serialize};

use crate::helper::serde_string;
use crate::schema::common::rule::{LogicalMode, QueryType, RuleType};
use crate::schema::dns::action::DnsRuleAction;
use crate::schema::macros::matcher::define_matcher_struct;

/// DNS rule。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DnsRule {
    Logical(Box<DnsLogicalRule>),
    Leaf(Box<DnsLeafRule>),
}

/// DNS logical rule。
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsLogicalRule {
    #[serde(rename = "type")]
    pub rule_type: RuleType,

    pub mode: LogicalMode,

    #[serde(default)]
    pub rules: Vec<DnsSubRule>,

    #[serde(default)]
    pub action: Option<DnsRuleAction>,

    #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
    pub server: Option<String>,
}

define_matcher_struct! {
    pub struct DnsLeafRule {
        extra {
            #[serde(default, deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub query_type: Vec<QueryType>,
        }
        tail {
            #[serde(default)]
            pub action: Option<DnsRuleAction>,

            #[serde(default, deserialize_with = "serde_string::de_opt_trim_lowercase")]
            pub server: Option<String>,
        }
    }
}

define_matcher_struct! {
    pub struct DnsSubRule {
        extra {
            #[serde(default, deserialize_with = "crate::helper::one_or_many::de_one_or_many")]
            pub query_type: Vec<QueryType>,
        }
        tail {}
    }
}

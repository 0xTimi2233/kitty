//! DNS rule action schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::serde_string;
use crate::schema::common::action::RejectAction;
use crate::schema::common::rule::DnsStrategy;

/// DNS rule action。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum DnsRuleAction {
    Route(DnsRouteAction),
    Reject(RejectAction),
    Predefined(DnsPredefinedAction),
}

/// DNS route action。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsRouteAction {
    #[serde(deserialize_with = "serde_string::de_trim_lowercase")]
    pub server: String,

    #[serde(default)]
    pub strategy: Option<DnsStrategy>,

    #[serde(default)]
    pub rewrite_ttl: Option<u32>,
}

/// DNS predefined action。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsPredefinedAction {
    #[serde(default, deserialize_with = "serde_string::de_opt_trim")]
    pub rcode: Option<String>,

    #[serde(default, deserialize_with = "serde_string::de_one_or_many_trim")]
    pub answer: Vec<String>,

    #[serde(default, deserialize_with = "serde_string::de_one_or_many_trim")]
    pub ns: Vec<String>,

    #[serde(default, deserialize_with = "serde_string::de_one_or_many_trim")]
    pub extra: Vec<String>,
}

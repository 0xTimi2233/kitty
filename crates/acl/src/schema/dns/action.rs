//! DNS rule action schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::serde_string;
use crate::schema::common::rule::DnsStrategy;

/// DNS rule action。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum DnsRuleAction {
    Route(DnsRouteAction),
    Reject(DnsRejectAction),
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

/// DNS reject action。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DnsRejectAction {
    pub method: RejectMethod,
    pub no_drop: bool,
}

impl Default for DnsRejectAction {
    fn default() -> Self {
        Self {
            method: RejectMethod::Default,
            no_drop: false,
        }
    }
}

/// reject method。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RejectMethod {
    Default,
    Drop,
}

impl Default for RejectMethod {
    fn default() -> Self {
        Self::Default
    }
}

/// DNS predefined action。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
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

impl Default for DnsPredefinedAction {
    fn default() -> Self {
        Self {
            rcode: None,
            answer: Vec::new(),
            ns: Vec::new(),
            extra: Vec::new(),
        }
    }
}

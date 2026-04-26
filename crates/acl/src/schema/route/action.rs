//! route rule action schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::serde_string;
use crate::schema::common::action::RejectAction;

/// route rule action。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum RouteRuleAction {
    Route(RouteAction),
    Reject(RejectAction),
}

/// route action。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteAction {
    #[serde(deserialize_with = "serde_string::de_trim_lowercase")]
    pub outbound: String,
}

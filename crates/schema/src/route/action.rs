//! route rule action schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// route action。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum RouteRuleAction {
    Route {
        #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
        outbound: String,
    },
    Reject {
        #[serde(default)]
        method: RouteRejectMethod,
    },
}

/// route reject 方法。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteRejectMethod {
    Default,
    Drop,
}

impl Default for RouteRejectMethod {
    fn default() -> Self {
        Self::Default
    }
}

//! DNS rule action schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::dns::server::DnsStrategy;

/// DNS rule action。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum DnsRuleAction {
    Route {
        #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
        server: String,

        #[serde(default)]
        strategy: Option<DnsStrategy>,
    },
    Reject {
        #[serde(default)]
        method: DnsRejectMethod,
    },
    Predefined {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        answers: Vec<PredefinedAnswer>,
    },
}

/// DNS reject 方法。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DnsRejectMethod {
    Refused,
    Nxdomain,
}

impl Default for DnsRejectMethod {
    fn default() -> Self {
        Self::Refused
    }
}

/// 预定义 DNS answer。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PredefinedAnswer {
    #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
    pub name: String,

    #[serde(rename = "type", deserialize_with = "crate::common::serde_helper::de_trim_uppercase")]
    pub record_type: String,

    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub value: String,

    #[serde(default = "default_ttl")]
    pub ttl: u32,
}

const fn default_ttl() -> u32 {
    60
}

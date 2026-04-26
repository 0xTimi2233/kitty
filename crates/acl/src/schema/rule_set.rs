//! rule set schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helper::duration::RefreshInterval;
use crate::helper::serde_string;

/// rule set。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RuleSet {
    Local(LocalRuleSet),
    Remote(RemoteRuleSet),
}

/// rule set format。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleSetFormat {
    Source,
    Binary,
}

/// local rule set。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRuleSet {
    #[serde(deserialize_with = "serde_string::de_trim_lowercase")]
    pub tag: String,

    pub format: RuleSetFormat,

    #[serde(deserialize_with = "serde_string::de_trim")]
    pub path: String,
}

/// remote rule set。
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteRuleSet {
    #[serde(deserialize_with = "serde_string::de_trim_lowercase")]
    pub tag: String,

    pub format: RuleSetFormat,

    #[serde(deserialize_with = "serde_string::de_trim")]
    pub url: String,

    #[serde(default)]
    pub update_interval: RefreshInterval,
}

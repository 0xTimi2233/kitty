//! rule_set schema。

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::defaults;
use crate::common::duration::RefreshInterval;

/// rule_set 配置。
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RuleSetConfig {
    Local(LocalRuleSet),
    Remote(RemoteRuleSet),
}

/// rule_set format。
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleSetFormat {
    Source,
    Binary,
}

/// local rule_set。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRuleSet {
    #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
    pub tag: String,

    #[serde(default = "defaults::rule_set_format")]
    pub format: RuleSetFormat,

    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub path: String,
}

/// remote rule_set。
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteRuleSet {
    #[serde(deserialize_with = "crate::common::serde_helper::de_trim_lowercase")]
    pub tag: String,

    #[serde(default = "defaults::rule_set_format")]
    pub format: RuleSetFormat,

    #[serde(deserialize_with = "crate::common::serde_helper::de_trim")]
    pub url: String,

    #[serde(default, deserialize_with = "crate::common::serde_helper::de_opt_trim_lowercase")]
    pub download_detour: Option<String>,

    #[serde(default = "defaults::rule_set_update_interval", deserialize_with = "crate::common::duration::de_refresh_interval")]
    pub update_interval: RefreshInterval,
}

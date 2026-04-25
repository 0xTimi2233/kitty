//! duration 字段 helper。

use std::fmt;
use std::time::Duration;

use serde::de::{self, Deserializer, Visitor};
use serde::{Serialize, Serializer};

/// rule_set 自动刷新间隔。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshInterval {
    Disabled,
    Interval(Duration),
}

impl Serialize for RefreshInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_u64(0),
            Self::Interval(duration) => humantime::format_duration(*duration)
                .to_string()
                .serialize(serializer),
        }
    }
}

/// 反序列化 rule_set 自动刷新间隔。
pub fn de_refresh_interval<'de, D>(deserializer: D) -> Result<RefreshInterval, D::Error>
where
    D: Deserializer<'de>,
{
    struct RefreshIntervalVisitor;

    impl<'de> Visitor<'de> for RefreshIntervalVisitor {
        type Value = RefreshInterval;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("duration string or 0")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == 0 {
                Ok(RefreshInterval::Disabled)
            } else {
                Ok(RefreshInterval::Interval(Duration::from_secs(value)))
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value < 0 {
                return Err(E::custom("duration must not be negative"));
            }
            self.visit_u64(value as u64)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let normalized = value.trim();
            if normalized == "0" || normalized == "0s" {
                return Ok(RefreshInterval::Disabled);
            }
            humantime::parse_duration(normalized)
                .map(RefreshInterval::Interval)
                .map_err(E::custom)
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }
    }

    deserializer.deserialize_any(RefreshIntervalVisitor)
}

/// humantime duration option helper。
pub mod option_humantime {
    pub use humantime_serde::option::{deserialize, serialize};
}

//! duration helper。

use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// rule_set 自动刷新间隔。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshInterval {
    Disabled,
    Interval(Duration),
}

impl Default for RefreshInterval {
    fn default() -> Self {
        Self::Interval(Duration::from_secs(86_400))
    }
}

impl Serialize for RefreshInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_str("0s"),
            Self::Interval(value) => {
                serializer.serialize_str(&humantime::format_duration(*value).to_string())
            }
        }
    }
}

impl<'de> Deserialize<'de> for RefreshInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RefreshInterval;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("0, \"0s\", or a duration string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value == 0 {
                    Ok(RefreshInterval::Disabled)
                } else {
                    Ok(RefreshInterval::Interval(Duration::from_secs(value)))
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value < 0 {
                    return Err(E::custom("duration must be non-negative"));
                }
                self.visit_u64(value as u64)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let duration = humantime::parse_duration(value.trim()).map_err(E::custom)?;
                if duration.is_zero() {
                    Ok(RefreshInterval::Disabled)
                } else {
                    Ok(RefreshInterval::Interval(duration))
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

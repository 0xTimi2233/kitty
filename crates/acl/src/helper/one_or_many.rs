//! one-or-many serde helper。

use serde::{Deserialize, Deserializer};

/// 单值或数组。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

/// 将单值或数组反序列化为 Vec。
pub fn de_one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let raw = Option::<OneOrMany<T>>::deserialize(deserializer)?;
    Ok(match raw {
        Some(OneOrMany::One(value)) => vec![value],
        Some(OneOrMany::Many(values)) => values,
        None => Vec::new(),
    })
}

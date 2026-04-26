//! serde 字符串 helper。

use serde::{Deserialize, Deserializer};

use crate::helper::one_or_many::OneOrMany;
use crate::helper::string;

/// trim 字符串。
pub fn de_trim<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut value = String::deserialize(deserializer)?;
    string::normalize_trim(&mut value);
    Ok(value)
}

/// trim Option 字符串，空字符串映射为 None。
pub fn de_opt_trim<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.and_then(|mut item| {
        string::normalize_trim(&mut item);
        if item.is_empty() { None } else { Some(item) }
    }))
}

/// trim + lowercase 字符串。
pub fn de_trim_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut value = String::deserialize(deserializer)?;
    string::normalize_trim_lowercase(&mut value);
    Ok(value)
}

/// trim + lowercase Option 字符串，空字符串映射为 None。
pub fn de_opt_trim_lowercase<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.and_then(|mut item| {
        string::normalize_trim_lowercase(&mut item);
        if item.is_empty() { None } else { Some(item) }
    }))
}

/// one-or-many 字符串，逐项 trim。
pub fn de_one_or_many_trim<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<OneOrMany<String>>::deserialize(deserializer)?;
    let mut values = match raw {
        Some(OneOrMany::One(value)) => vec![value],
        Some(OneOrMany::Many(values)) => values,
        None => Vec::new(),
    };
    values.retain_mut(string::normalize_trim_is_not_empty);
    Ok(values)
}

/// one-or-many 字符串，逐项 trim + lowercase。
pub fn de_one_or_many_trim_lowercase<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<OneOrMany<String>>::deserialize(deserializer)?;
    let mut values = match raw {
        Some(OneOrMany::One(value)) => vec![value],
        Some(OneOrMany::Many(values)) => values,
        None => Vec::new(),
    };
    values.retain_mut(string::normalize_trim_lowercase_is_not_empty);
    Ok(values)
}

/// one-or-many domain_suffix，逐项 trim + lowercase + 去掉前导点。
pub fn de_one_or_many_domain_suffix<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<OneOrMany<String>>::deserialize(deserializer)?;
    let mut values = match raw {
        Some(OneOrMany::One(value)) => vec![value],
        Some(OneOrMany::Many(values)) => values,
        None => Vec::new(),
    };
    values.retain_mut(string::normalize_domain_suffix);
    Ok(values)
}

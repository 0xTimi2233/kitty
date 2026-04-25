//! serde 反序列化 helper。

use serde::{Deserialize, Deserializer};

use crate::common::string_utils;

/// 去空格。
pub fn de_trim<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut value = String::deserialize(deserializer)?;
    string_utils::normalize_trim(&mut value);
    Ok(value)
}

/// Option 去空格。
pub fn de_opt_trim<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.and_then(|mut value| {
        string_utils::normalize_trim(&mut value);
        if value.is_empty() { None } else { Some(value) }
    }))
}

/// 去空格 + 小写。
pub fn de_trim_lowercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut value = String::deserialize(deserializer)?;
    string_utils::normalize_trim_lowercase(&mut value);
    Ok(value)
}

/// 去空格 + 大写。
pub fn de_trim_uppercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let mut value = String::deserialize(deserializer)?;
    string_utils::normalize_trim(&mut value);
    if value.is_ascii() {
        value.make_ascii_uppercase();
    } else if value.chars().any(char::is_lowercase) {
        value = value.to_uppercase();
    }
    Ok(value)
}

/// Option 去空格 + 小写。
pub fn de_opt_trim_lowercase<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.and_then(|mut value| {
        string_utils::normalize_trim_lowercase(&mut value);
        if value.is_empty() { None } else { Some(value) }
    }))
}

/// Vec 中每个元素去空格，过滤空字符串。
pub fn de_vec_trim<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut values = Option::<Vec<String>>::deserialize(deserializer)?.unwrap_or_default();
    values.retain_mut(string_utils::normalize_trim_is_not_empty);
    Ok(values)
}

/// Vec 中每个元素去空格 + 小写，过滤空字符串。
pub fn de_vec_trim_lowercase<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut values = Option::<Vec<String>>::deserialize(deserializer)?.unwrap_or_default();
    values.retain_mut(string_utils::normalize_trim_lowercase_is_not_empty);
    Ok(values)
}

/// Vec 中每个 domain suffix 去空格、小写、去前导点，过滤空字符串。
pub fn de_vec_domain_suffix<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut values = Option::<Vec<String>>::deserialize(deserializer)?.unwrap_or_default();
    values.retain_mut(string_utils::normalize_domain_suffix_is_not_empty);
    Ok(values)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawOneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

/// 单个字符串或字符串数组统一转 Vec，并对元素 trim。
pub fn de_one_or_many_trim<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<RawOneOrMany<String>>::deserialize(deserializer)?;
    let mut values = match raw {
        Some(RawOneOrMany::One(value)) => vec![value],
        Some(RawOneOrMany::Many(values)) => values,
        None => Vec::new(),
    };
    values.retain_mut(string_utils::normalize_trim_is_not_empty);
    Ok(values)
}

/// 单个字符串或字符串数组统一转 Vec，并对元素 trim + lowercase。
pub fn de_one_or_many_trim_lowercase<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<RawOneOrMany<String>>::deserialize(deserializer)?;
    let mut values = match raw {
        Some(RawOneOrMany::One(value)) => vec![value],
        Some(RawOneOrMany::Many(values)) => values,
        None => Vec::new(),
    };
    values.retain_mut(string_utils::normalize_trim_lowercase_is_not_empty);
    Ok(values)
}

/// 单个 domain suffix 或数组统一转 Vec，并对元素 trim + lowercase + 去前导点。
pub fn de_one_or_many_domain_suffix<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = Option::<RawOneOrMany<String>>::deserialize(deserializer)?;
    let mut values = match raw {
        Some(RawOneOrMany::One(value)) => vec![value],
        Some(RawOneOrMany::Many(values)) => values,
        None => Vec::new(),
    };
    values.retain_mut(string_utils::normalize_domain_suffix_is_not_empty);
    Ok(values)
}

/// 单个值或数组统一转 Vec。
pub fn de_one_or_many<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let raw = Option::<RawOneOrMany<T>>::deserialize(deserializer)?;
    Ok(match raw {
        Some(RawOneOrMany::One(value)) => vec![value],
        Some(RawOneOrMany::Many(values)) => values,
        None => Vec::new(),
    })
}

//! 字符串局部 normalize 工具。

/// 原地 trim。
pub fn normalize_trim(value: &mut String) {
    let trimmed = value.trim();
    let len = trimmed.len();
    if len == 0 {
        value.clear();
        return;
    }
    let start = trimmed.as_ptr() as usize - value.as_ptr() as usize;
    value.truncate(start + len);
    if start > 0 {
        value.drain(..start);
    }
}

/// 原地 trim，并返回是否非空。
pub fn normalize_trim_is_not_empty(value: &mut String) -> bool {
    normalize_trim(value);
    !value.is_empty()
}

/// 原地 lowercase。
pub fn normalize_lowercase(value: &mut String) {
    if value.is_ascii() {
        value.make_ascii_lowercase();
    } else if value.chars().any(char::is_uppercase) {
        *value = value.to_lowercase();
    }
}

/// 原地 trim + lowercase。
pub fn normalize_trim_lowercase(value: &mut String) {
    let (start, trimmed_len, needs_lowercase) = {
        let trimmed = value.trim();
        let len = trimmed.len();
        if len == 0 {
            value.clear();
            return;
        }
        let offset = trimmed.as_ptr() as usize - value.as_ptr() as usize;
        let needs_lower = trimmed.chars().any(char::is_uppercase);
        (offset, len, needs_lower)
    };

    if value.len() != trimmed_len + start {
        value.truncate(start + trimmed_len);
    }
    if start > 0 {
        value.drain(..start);
    }
    if needs_lowercase {
        normalize_lowercase(value);
    }
}

/// 原地 trim + lowercase，并返回是否非空。
pub fn normalize_trim_lowercase_is_not_empty(value: &mut String) -> bool {
    normalize_trim_lowercase(value);
    !value.is_empty()
}

/// 归一化 domain_suffix。
pub fn normalize_domain_suffix(value: &mut String) -> bool {
    normalize_trim_lowercase(value);
    while value.starts_with('.') {
        value.remove(0);
    }
    !value.is_empty()
}

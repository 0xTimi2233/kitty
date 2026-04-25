//! 字符串 normalize 工具。

/// 去掉首尾空白。
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

/// 去掉首尾空白，并返回是否非空。
pub fn normalize_trim_is_not_empty(value: &mut String) -> bool {
    normalize_trim(value);
    !value.is_empty()
}

/// 小写化字符串。
pub fn normalize_lowercase(value: &mut String) {
    if value.is_ascii() {
        value.make_ascii_lowercase();
    } else if value.chars().any(char::is_uppercase) {
        *value = value.to_lowercase();
    }
}

/// 去掉首尾空白并小写化。
pub fn normalize_trim_lowercase(value: &mut String) {
    let (start, len, needs_lowercase) = {
        let trimmed = value.trim();
        let len = trimmed.len();
        if len == 0 {
            value.clear();
            return;
        }
        let start = trimmed.as_ptr() as usize - value.as_ptr() as usize;
        let needs_lowercase = trimmed.chars().any(char::is_uppercase);
        (start, len, needs_lowercase)
    };

    if value.len() != start + len {
        value.truncate(start + len);
    }
    if start > 0 {
        value.drain(..start);
    }
    if needs_lowercase {
        normalize_lowercase(value);
    }
}

/// 去掉首尾空白、小写化，并返回是否非空。
pub fn normalize_trim_lowercase_is_not_empty(value: &mut String) -> bool {
    normalize_trim_lowercase(value);
    !value.is_empty()
}

/// 去掉 domain suffix 前导点。
pub fn normalize_domain_suffix(value: &mut String) {
    normalize_trim_lowercase(value);
    while value.starts_with('.') {
        value.drain(..1);
    }
}

/// 去掉 domain suffix 前导点，并返回是否非空。
pub fn normalize_domain_suffix_is_not_empty(value: &mut String) -> bool {
    normalize_domain_suffix(value);
    !value.is_empty()
}

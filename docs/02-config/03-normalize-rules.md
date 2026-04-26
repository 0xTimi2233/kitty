# Normalize 规则

Decode-time local normalize：

- 普通 tag 字符串：trim + lowercase。
- 域名类字段：trim + lowercase。
- domain_suffix：trim + lowercase，并移除前导点。
- 正则字段：trim，不 lowercase。
- path 类字段：trim，不 lowercase。
- one-or-many 字段统一反序列化为 Vec。
- 空字符串在 Option 字段中归一化为 None。

Structural normalize 在 basic validate 之后执行，负责统一 rule/action 写法和构建 Rule IR。

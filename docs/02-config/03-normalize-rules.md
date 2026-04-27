# Normalize 规则

## Decode-time local normalize

ACL schema 在反序列化阶段只做局部、无跨字段语义的 normalize：

- 普通 tag 字符串：trim + lowercase。
- 域名类字段：trim + lowercase。
- `domain_suffix`：trim + lowercase，并移除前导点。
- 正则字段：trim，不 lowercase。
- path 类字段：trim，不 lowercase。
- one-or-many 字段统一反序列化为 Vec。
- 空字符串在 `Option<T>` 字段中归一化为 None。

Decode-time local normalize 不做引用解析、不检查 tag 重复、不检查 action target 是否存在，也不构建 Rule IR。

## Structural normalize

Structural normalize 在 basic validate 之后执行，负责统一 rule/action 写法和构建 Rule IR。

DNS rule 允许同时出现对象 `action` 和快捷 `server` 字段。二者同时出现时：

- `action` 优先。
- `server` 作为兼容输入被忽略。
- normalize 结果必须等价于对象 `action`。

Route rule 允许同时出现对象 `action` 和快捷 `outbound` 字段。二者同时出现时：

- `action` 优先。
- `outbound` 作为兼容输入被忽略。
- normalize 结果必须等价于对象 `action`。

快捷字段单独出现时，由 structural normalize 转成对象 action：

- DNS `server` 转成 DNS route action。
- route `outbound` 转成 route action。

Structural normalize 不解析 action target 是否存在。target、detour、final 的存在性和类型由 semantic validate 处理。

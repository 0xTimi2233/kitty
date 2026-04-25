# Schema 代码指南

`schema` crate 只负责输入契约。

允许：

- serde schema。
- 默认值填充。
- one-or-many 输入兼容。
- 字符串 trim / lowercase。
- 重复字段通过 macro 生成。

不允许：

- 引用校验。
- 语义校验。
- rule_set 下载。
- rule_set 展开。
- runtime 构建。

字段规则：

- 必填字段使用 `T`。
- 有默认值字段使用 `#[serde(default = "...")]`。
- 默认 false 的 bool 使用 `#[serde(default)]`。
- 集合字段默认空集合。
- 只有缺省本身具备语义时才使用 `Option<T>`。

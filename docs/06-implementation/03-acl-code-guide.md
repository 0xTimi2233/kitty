# ACL 代码指南

ACL crate 内部结构：

```text
helper/             # 工具函数：string、serde string、one-or-many、duration
schema/             # 配置输入结构
schema/common/      # schema 共享结构与共享字段类型
schema/macros/      # schema 声明式宏
schema/defaults.rs  # 配置默认值
```

规则：

- helper 不表达配置结构。
- common 表达 schema 的共享组成部分。
- defaults 是配置契约，不属于 helper。
- macros 只用于生成重复 schema 字段。
- required 字段不使用 default。
- 有明确默认值的字段使用 schema default。
- 缺省本身有语义时才使用 Option。

# ACL Schema

ACL schema 的职责：

- 定义 JSON 输入结构。
- serde 注解。
- 默认值填充。
- one-or-many 输入归一化。
- trim / lowercase 等局部 normalize。
- schema macro 消除重复字段。

ACL schema 不做：

- 基础校验。
- 语义校验。
- tag 重复校验。
- 引用解析。
- rule_set 下载。
- runtime 构建。

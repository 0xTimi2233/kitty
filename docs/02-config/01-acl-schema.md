# ACL Schema

ACL schema 是配置输入防腐层，只描述 JSON 输入结构和 decode-time 行为，不表达 control-plane 语义。

ACL schema 的职责：

- 定义 JSON 输入结构。
- serde 注解。
- 默认值填充。
- one-or-many 输入归一化。
- trim / lowercase 等局部 normalize。
- schema macro 消除重复字段。
- 将缺省但有明确语义的字段解码为 `Option<T>`。

ACL schema 不做：

- 基础校验。
- 语义校验。
- tag 重复校验。
- 引用解析。
- rule_set 下载。
- runtime 构建。
- 危险默认拦截。
- duration 下限、端口范围、URL scheme、CIDR、正则可编译性等合法性校验。

危险默认和下限规则归属 basic validate。例如 `update_interval` 非 0 时不得小于 15 分钟，`listen_port` 必须显式配置且端口合法。

基础数据类型除了 `bool`，其他字段必须符合以下三类之一：

- required。
- `Option<T>`。
- 有明确产品语义的具体默认值。

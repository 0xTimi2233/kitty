# 术语

- `schema`：用户 JSON 配置对应的输入契约，只做反序列化、默认值填充和输入 normalize。
- `domain model`：compile-pipeline 生成的领域模型，不包含 serde 注解。
- `runtime model`：发布给数据面的只读驻留模型。
- `compile-pipeline`：从 schema 到 runtime model 的控制面流水线。
- `match-pipeline`：数据面流量匹配与分流流水线。
- `rule_set`：可被规则引用的规则集合，编译时展开合并。
- `KSR`：Kitty 自定义二进制规则集缓存格式。

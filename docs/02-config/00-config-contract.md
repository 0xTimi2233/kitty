# 配置契约

根配置字段：

```text
log
dns
inbounds
outbounds
route
```

ACL schema 负责反序列化、默认值填充和局部 normalize。基础校验、结构 normalize、语义收集、语义校验和 runtime 构建由 control-plane 完成。

必填字段不在 schema 中提供默认值。带明确默认值的字段在 schema 层填充。可选字段只有在缺省本身有语义时使用 `Option<T>`。

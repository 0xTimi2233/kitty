# 配置契约

根配置字段：

```text
log
dns
inbounds
outbounds
route
```

`log` 可缺省并由 ACL schema 填充默认值。`dns`、`inbounds`、`outbounds` 和 `route` 是必填根字段。

## 最小有效配置

用于 start/reload 的运行时配置必须满足：

- `dns.servers` 至少一个。
- `inbounds` 至少一个。
- `outbounds` 至少一个。
- 运行时需要的 `dns.final` 必须显式配置。
- 运行时需要的 `route.final` 必须显式配置。
- route 无匹配 rule 时走显式 `route.final`。
- DNS 无匹配 rule 时走显式 `dns.final`。

空数组、缺失运行时 final、空字符串、非法端口、非法 duration 下限等由 basic validate 或 semantic validate 拒绝，不由 ACL schema 静默修正。

## Schema 字段规则

ACL schema 负责反序列化、默认值填充和局部 normalize。基础校验、结构 normalize、语义收集、语义校验和 runtime 构建由 control-plane 完成。

字段规则：

- 必填字段不在 schema 中提供默认值。
- 带明确默认值的字段在 schema 层填充。
- 可选字段只有在缺省本身有语义时使用 `Option<T>`。
- 基础数据类型除了 `bool`，其他字段要么必填，要么是 `Option<T>`，要么指定具体默认值。
- `listen_port` 不应有隐式默认值；监听端口必须由用户显式配置，并由 validate 检查合法范围。

# Route 配置

Route 配置包含：

- `rules`。
- `rule_set`。
- `final`。
- `default_domain_resolver`。

## Final 行为

运行时配置必须显式配置 `route.final`。route rule 无匹配结果时走显式 `route.final`。

`route.final`、`detour` 和 action target 在 semantic validate 中解析并在 runtime model 中固化。缺失、引用不存在或引用类型错误都必须失败。

## Route rule action

Route rule action 支持：

- route。
- reject。

Route rule 支持快捷 `outbound` 写法。快捷 `outbound` 单独出现时由 structural normalize 转成 route action。

当对象 `action` 与快捷 `outbound` 同时出现时：

- `action` 优先。
- `outbound` 作为兼容输入被忽略。
- normalize 结果必须等价于对象 `action`。

## Domain resolver

`default_domain_resolver` 提供 route 层默认 domain resolver。outbound 或 DNS server 层的 resolver 覆盖关系由 structural normalize 固化，引用存在性由 semantic validate 处理。

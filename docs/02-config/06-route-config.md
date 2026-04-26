# Route 配置

Route 配置包含：

- rules。
- rule_set。
- final。
- default_domain_resolver。

Route rule action 支持：

- route。
- reject。

快捷 `outbound` 写法由 structural normalize 转成对象 action。

`final`、`detour` 和 action target 在 semantic validate 中解析并在 runtime model 中固化。

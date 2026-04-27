# DNS 配置

DNS 配置包含：

- `servers`：tcp、udp、tls、quic、https、h3。
- `rules`：leaf rule 或 logical rule。
- `final`：默认 DNS server 引用。
- `strategy`：DNS 查询策略。
- `cache`：DNS cache、lazy cache、dump。

## Final 行为

运行时配置必须显式配置 `dns.final`。DNS rule 无匹配结果时走显式 `dns.final`。

`dns.final` 引用目标在 semantic validate 中解析并在 runtime model 中固化。缺失、引用不存在或引用类型错误都必须失败。

## DNS rule action

DNS rule action 支持：

- route。
- reject。
- predefined。

DNS rule 支持快捷 `server` 写法。快捷 `server` 单独出现时由 structural normalize 转成 route action。

当对象 `action` 与快捷 `server` 同时出现时：

- `action` 优先。
- `server` 作为兼容输入被忽略。
- normalize 结果必须等价于对象 `action`。

## DNS cache 默认行为

- DNS cache 默认启用。
- lazy cache 默认禁用。
- dump 默认禁用。
- cache capacity、lazy ttl、lazy reply_ttl、dump path 和 dump interval 的默认值见 `docs/02-config/02-default-values.md`。

DNS cache、lazy cache 和 dump 的运行期行为由数据面和 operations 文档继续约束；ACL schema 只负责 decode/default/local normalize。

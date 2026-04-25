# 配置契约

Root 字段：

- `log`
- `dns`
- `inbounds`
- `outbounds`
- `route`

## log

字段：

- `level`: `trace | debug | info | warn | error`，默认 `warn`。
- `format`: `text | json`，默认 `text`。
- `timestamp`: 默认 `true`。

## dns

字段：

- `servers`
- `rules`
- `cache`

DNS server 类型：

- `udp`
- `tcp`
- `tls`
- `quic`
- `https`
- `h3`

DNS action：

- `route`
- `reject`
- `predefined`

## inbound

类型：

- `direct`
- `socks`
- `vless`
- `dns`
- `tc`

## outbound

类型：

- `direct`
- `socks`
- `vless`

## route

字段：

- `rules`
- `final`
- `default_domain_resolver`
- `rule_set`

route action：

- `route`
- `reject`

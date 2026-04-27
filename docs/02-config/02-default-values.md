# 默认值

默认值在 ACL schema 中填充。未列入本文件的非 `bool` 基础数据类型字段，必须是 required、`Option<T>` 或在后续文档中补充明确默认值。

## Log

- `log.level`：warn。
- `log.format`：text。
- `log.timestamp`：true。

## DNS cache

- `dns.cache.enable`：true。
- `dns.cache.capacity`：4096。
- `dns.cache.lazy_cache.enable`：false。
- `dns.cache.lazy_cache.ttl`：86400 秒。
- `dns.cache.lazy_cache.reply_ttl`：5 秒。
- `dns.cache.dump.enable`：false。
- `dns.cache.dump.path`：`.cache/dump.db`。
- `dns.cache.dump.interval`：3600 秒。

## DNS server

- DNS TCP/UDP port：53。
- DNS TLS/QUIC port：853。
- DNS HTTPS/H3 port：443。
- DoH path：`/dns-query`。

## Inbound / outbound shared defaults

- TCP keepalive：300 秒。
- TCP keepalive interval：75 秒。
- UDP timeout：300 秒。
- outbound connect timeout：5 秒。
- outbound network：tcp + udp。
- `bind_interface`：缺省为 None。
- `detour`：缺省为 None。
- `routing_mark`：0。
- `reuse_addr`：false。
- `tcp_fast_open`：false。
- `tcp_multi_path`：false。
- `disable_tcp_keep_alive`：false。
- `udp_fragment`：false。

`listen_port` 不应有默认值。监听端口必须由用户显式配置，并由 basic validate 检查合法范围。

## Protocol defaults

- SOCKS version：5。
- VLESS packet encoding：xudp。
- TLS `enabled`：true。
- TLS `insecure`：false。
- TLS `server_name`：缺省为 None。
- TLS `alpn`：空列表。
- TLS `min_version`：缺省为 None。
- TLS `max_version`：缺省为 None。
- multiplex 对象缺省为 None；对象存在时 `enabled` 默认为 false。
- UDP over TCP 对象缺省为 None；对象存在时 `enabled` 默认为 true，version 默认为 2。

## Action defaults

- reject action `method`：default。
- reject action `no_drop`：false。

## Rule set

- `rule_set.update_interval`：缺失时 1d。
- `rule_set.update_interval` 为 0 或 "0s" 表示禁用自动刷新。
- 非 0 `rule_set.update_interval` 小于 15 分钟时，由 basic validate 拒绝。

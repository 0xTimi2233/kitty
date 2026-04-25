# 默认值

默认值在 schema 层填充。

关键默认值：

| 字段                             |              默认值 |
|--------------------------------|-----------------:|
| log.level                      |           `warn` |
| log.format                     |           `text` |
| log.timestamp                  |             true |
| dns.cache.enable               |             true |
| dns.cache.capacity             |             4096 |
| dns.cache.lazy_cache.enable    |            false |
| dns.cache.lazy_cache.ttl       |            86400 |
| dns.cache.lazy_cache.reply_ttl |                5 |
| dns.cache.dump.enable          |            false |
| dns.cache.dump.path            | `.cache/dump.db` |
| dns.cache.dump.interval        |             3600 |
| udp/tcp DNS server port        |               53 |
| tls/quic DNS server port       |              853 |
| https/h3 DNS server port       |              443 |
| DoH path                       |     `/dns-query` |
| outbound.connect_timeout       |             `5s` |
| outbound.network               |     `[tcp, udp]` |
| rule_set.format                |         `source` |
| rule_set.update_interval       |             `1d` |
| listen tcp_keep_alive          |             `5m` |
| listen tcp_keep_alive_interval |            `75s` |
| listen udp_timeout             |             `5m` |
| dial connect_timeout           |             `5s` |
| dial tcp_keep_alive            |             `5m` |
| dial tcp_keep_alive_interval   |            `75s` |
| outbound network               |     `[tcp, udp]` |
| socks.version                  |                5 |
| vless.packet_encoding          |           `xudp` |

`rule_set.update_interval` 显式配置为 `0` 或 `"0s"` 表示关闭自动刷新。

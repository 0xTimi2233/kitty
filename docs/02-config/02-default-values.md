# 默认值

默认值在 ACL schema 中填充。

关键默认值：

- log.level：warn。
- log.format：text。
- log.timestamp：true。
- DNS cache enable：true。
- DNS cache capacity：4096。
- lazy cache ttl：86400 秒。
- lazy cache reply_ttl：5 秒。
- dump path：`.cache/dump.db`。
- dump interval：3600 秒。
- DNS TCP/UDP port：53。
- DNS TLS/QUIC port：853。
- DNS HTTPS/H3 port：443。
- DoH path：`/dns-query`。
- TCP keepalive：300 秒。
- TCP keepalive interval：75 秒。
- UDP timeout：300 秒。
- outbound connect timeout：5 秒。
- outbound network：tcp + udp。
- rule_set.update_interval：缺失时 1d；0 或 "0s" 表示禁用自动刷新。

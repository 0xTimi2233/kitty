# 09 eBPF Map Sync

## 职责

eBPF Map Sync 在用户态维护 kernel-side map 的写入、更新、过期和清理。

## 输入

- DNS query result。
- Route match result。
- Runtime eBPF sync spec。

## 输出

- eBPF map update。
- eBPF map cleanup。
- sync error event。

## map 类型

第一版维护四类核心 map：

- domain to IPv4 answer map。
- domain to IPv6 answer map。
- IPv4 route mark map。
- IPv6 route mark map。

## DNS cache key

```text
normalized_domain_hash_128 + qtype + qclass
```

约束：

- 只处理 `qclass = IN`。
- 只写入 A / AAAA。
- 其他 record type 进入用户态。
- 用户态保留完整 domain，用于 debug 和碰撞检测。
- 发现 hash 碰撞时，该 domain 不写入 eBPF map。

## DNS action value

DNS map value 需要表达：

- valid answer。
- no domain。
- reject。
- ttl / expire_at。

## route mark

route mark value 需要表达：

- `DIRECT`。
- `REJECT`。
- `outbound_id`。

kernel-side 看到非 `DIRECT` / `REJECT` 时，将流量导向用户态，并把 outbound 标记传递给用户态。

## 写入时机

DNS 处理完成后，用户态使用 domain + answer IP 再执行一次 route match，得到 route mark，再写入 eBPF map。

## 生命周期

- map 创建、打开、pin、写入、清理都由用户态执行。
- kernel-side program 不负责 cache cleanup。
- Runtime 切换后，旧 generation 的 map entry 需要按计划清理。

## 测试要点

- A / AAAA 结果能写入对应 map。
- 非 A / AAAA 不写入 DNS map。
- route mark 能区分 Direct、Reject、Outbound。
- TTL 过期 entry 能被用户态清理。

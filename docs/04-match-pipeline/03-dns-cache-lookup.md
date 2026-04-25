# 03 DNS Cache Lookup

## 职责

DNS Cache Lookup 在用户态 DNS pipeline 中处理 DNS cache、lazy cache 和 dump 恢复数据。

## 输入

- `NormalizedMatchContext`
- DNS cache runtime。
- DNS query。

## 输出

- cache hit response。
- cache miss marker。
- cache statistics event。

## cache 类型

### normal cache

按 qname、qtype、qclass 查询有效记录。

### lazy cache

记录过期后，在允许的 lazy window 内可先返回短 TTL 响应，并异步触发刷新。

### dump cache

启动时可从 dump 文件恢复 DNS cache。dump 文件由 cache runtime 管理，不参与规则编译。

## eBPF cache 关系

用户态 DNS cache 是权威缓存层。eBPF DNS cache 只保存 A / AAAA 快速响应所需数据，由用户态负责写入、更新和清理。

## 约束

- DNS cache 不作为通用规则决策缓存。
- cache hit 仍需要记录统计事件。
- cache miss 才进入 DNS rule match。

## 测试要点

- normal cache hit 返回原始 TTL 语义下的结果。
- lazy cache hit 返回 configured reply TTL。
- cache miss 进入后续 DNS rule match。
- dump 恢复数据不破坏 TTL 判断。

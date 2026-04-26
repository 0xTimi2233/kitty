# Match Pipeline 编排

Match Pipeline 是数据面热路径。它只依赖已发布 RuntimeModel，不读取 ACL schema，不访问原始配置，不做 rule_set 展开。

顺序：

```text
01 ingress-classify
02 context-normalize
03 dns-cache-lookup
04 context-enrich
05 index-probe
06 bitmap-short-circuit
07 first-match-evaluator
08 action-dispatch
09 ebpf-map-sync
```

规则决策不使用通用 decision cache。DNS cache 和 eBPF DNS cache 是独立缓存层。

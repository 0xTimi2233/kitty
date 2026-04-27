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

## 当前上下文范围

当前匹配需求只承诺 schema 已有字段。

低成本上下文包括 domain、IP、port、network、rule_set 等字段。

高成本上下文当前只包括 process/user 类字段：

- process/user 字段是可选上下文。
- 采集需要对应平台能力和权限。
- 采集失败时，依赖该字段的条件不匹配。
- 采集失败可记录按需 debug 事件。

SNI 与 HTTP host 是 future scope，不属于当前 match pipeline 必须支持的上下文字段。

## 不变量

- 不在数据面热路径做配置解析。
- 不访问原始配置。
- 不做 rule_set 展开。
- 保持 first-match 语义。

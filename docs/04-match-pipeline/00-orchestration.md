# Match Pipeline 编排

Match Pipeline 的目标是在已发布 Runtime Model 上完成 DNS 和代理流量的高速分流处理。

## 阶段顺序

1. `01-ingress-classify.md`：识别入口类型和可用 fast path。
2. `02-context-normalize.md`：对请求上下文做运行期 normalize。
3. `03-dns-cache-lookup.md`：处理 DNS cache 和 lazy cache。
4. `04-context-enrich.md`：按需补充 process、user、sniff 等上下文。
5. `05-index-probe.md`：查询倒排索引得到候选规则。
6. `06-bitmap-short-circuit.md`：使用 bitmap 合并条件组并短路候选。
7. `07-first-match-evaluator.md`：按规则顺序确认 first-match。
8. `08-action-dispatch.md`：执行 route、reject、predefined 等 action。
9. `09-ebpf-map-sync.md`：同步 DNS 与 route 相关 eBPF map。

## 热路径原则

- 只读取已发布 Runtime Model。
- 不做通用规则决策缓存。
- 不在热路径展开 rule_set。
- 不在热路径解析字符串 target。
- 只有配置使用相关 matcher 时才补充昂贵上下文。
- bitmap 只用于缩小候选，最终命中仍由 evaluator 确认。

## Runtime 输入

- `RuntimeModel`
- `MatchRequest`
- `IngressContext`

## 输出

- DNS response。
- Proxy route decision。
- Reject decision。
- eBPF map update task。
- 统计事件。

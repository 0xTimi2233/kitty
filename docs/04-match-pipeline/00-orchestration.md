# Match Pipeline 编排

推荐顺序：

1. 构建最小上下文。
2. 对输入域名、tag、地址做 normalize。
3. 查询 DNS cache 或 eBPF fast path 结果。
4. 按需补充上下文。
5. 使用倒排索引缩小候选规则。
6. 使用 bitmap 做短路。
7. first-match evaluator 逐条确认。
8. dispatch action。
9. 同步必要 eBPF map。

规则决策不做通用 decision cache。

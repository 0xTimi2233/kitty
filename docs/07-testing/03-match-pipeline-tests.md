# Match Pipeline Tests

覆盖：

- context enrich 按需触发。
- process/user 采集成功时参与匹配。
- process/user 采集失败时，依赖该字段的条件不匹配。
- 采集失败可产生按需 debug 事件。
- bitmap short-circuit。
- first-match evaluator。
- DNS cache。
- action dispatch。

当前测试不要求 SNI 或 HTTP host 匹配能力。SNI 与 HTTP host 是 future scope。

# Batch 002 User Response

Recorded at: 2026-04-27T15:38:22Z

## Q1

选择：1

API 保留为 future scope。当前 docs 明确管理面当前只要求 CLI 和 signal；API 只作为未来能力，不进入当前需求细节。

## Q2

选择：2

eBPF 是必需能力；权限或 attach 失败时 start/reload 失败。privileged port bind 失败同样失败。

## Q3

用户决策：

开发代码时约定不输出这些敏感日志。

主线程解释：

- 本项不是原选项的直接编号选择，应作为自定义决策处理。
- 文档更新时应明确实现约束：代码不得输出敏感日志。
- 需要由 PM 判断是否还要补充“敏感字段列表、脱敏策略、debug dump 边界、错误摘要字段”等文档细节。

## Q4

选择：2

启动和 external reload 遇到过期缓存且刷新失败就失败；只有 internal refresh 允许保留旧 runtime。cache hash 使用 URL/path。

## Q5

选择：1

当前只把 schema 已有字段作为当前匹配需求：process、user 类字段保留为需要权限/平台能力的可选上下文；SNI/HTTP host 标记为 future。采集失败时该字段条件不匹配，并记录按需 debug 事件。

# Docs Update Batch C Report

Status: pass

## Scope

本轮只修改 dispatch 允许的 8 个 match/testing/logging/implementation guide 文档：

- `docs/04-match-pipeline/00-orchestration.md`
- `docs/04-match-pipeline/04-context-enrich.md`
- `docs/07-testing/03-match-pipeline-tests.md`
- `docs/07-testing/05-ebpf-tests.md`
- `docs/07-testing/06-performance-testing.md`
- `docs/06-implementation/03-acl-code-guide.md`
- `docs/05-operations/05-logging-and-error.md`
- `docs/02-config/04-logging-config.md`

未修改源码、`codexspec/vision.md`、`codexspec/roadmap.md`、Batch A/B 文件或其他 `docs/` 文件。

## Decisions Applied

- Match pipeline 当前只承诺 schema 已有字段；process/user 为当前可选高成本上下文，SNI/HTTP host 为 future scope。
- process/user 采集失败时，依赖该字段的条件不匹配，并可记录按需 debug 事件。
- Match pipeline tests 不要求 SNI/HTTP host 当前测试。
- eBPF tests 对齐 eBPF 必需能力和失败行为测试边界，不测试 optional fallback。
- Performance testing 早期只定义 benchmark harness 和数据规模占位，硬阈值后续再定。
- ACL code guide 对齐 required/default/Option 规则，`listen_port` 不允许隐式 default。
- Logging/error docs 只写原则级敏感日志要求，不新增敏感字段清单、错误摘要字段清单或 debug dump 边界。

## Notes

本轮更新仍属于 preflight docs update，不是 formal planning，也不表示正式需求冻结。后续正式立项时仍应从更新后的 `docs/` 重新读取需求。

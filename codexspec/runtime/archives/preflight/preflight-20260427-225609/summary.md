# Preflight Closure Summary

## 状态

Status: ready-for-plan-later / ready-for-archive

本次 preflight 审计、用户决策澄清和三批 `docs/` 更新轮已经完成。当前没有剩余用户决策，也没有新的 PM blocker。该状态表示本次 preflight 可以归档，未来可以用于正式 `$plan commit` 的参考；不表示正式需求冻结。

## Source Coverage

本次 preflight 的原始审计覆盖由 `sources.md` 记录：协议文件、`codexspec/vision.md`、`codexspec/roadmap.md`、`docs/` 正文和 `crates/acl/` schema。审计结论认为 `docs/` 已覆盖愿景、分层、配置、pipeline、operations、实现规范和测试主题，但很多内容在本轮澄清前仍缺少默认行为、冲突处理和可验收口径；`crates/acl/` schema 已覆盖大部分配置形状与默认值，但曾与文档在默认启用、危险默认和 validation 边界上不完全一致。

本 closure 轮没有重新读取 `docs/` 或 `crates/` 正文，只读取已有 preflight runtime 产物和三批 docs update report。

## 关键决策

| ID | 结论 |
|---|---|
| D-001 | 本次不是需求立项，不划分 milestone，不进入 commit planning；只做需求澄清和 `docs/` 更新准备。 |
| D-002 | 最小有效配置采用严格显式配置；`dns.servers`、`inbounds`、`outbounds` 至少各一个；`dns.final` 和 `route.final` 必须显式配置。 |
| D-003 | 现有 schema 形状作为当前事实来源；默认值写回 docs；危险默认进入 basic validate；`listen_port` 不应有隐式默认值。 |
| D-004 | rule `action` object 与快捷字段可同时出现，由 normalize 阶段处理，且 `action` 优先。 |
| D-005 | 早期性能要求只定义 benchmark harness 和数据规模占位，硬阈值后续再定。 |
| D-006 | API 是 future scope；当前管理面只要求 CLI 和 signal。 |
| D-007 | eBPF 是必需能力；权限、load、attach 或 privileged port bind 失败导致 start/reload 失败。 |
| D-008 | logging/error docs 只写原则级敏感日志要求：代码不得输出敏感日志；不新增敏感字段清单、错误摘要字段清单或 debug dump 边界。 |
| D-009 | remote rule_set 过期缓存刷新失败时，start/external reload 失败；internal refresh 可保留旧 runtime；cache hash 使用 URL/path。 |
| D-010 | 当前 match context 只承诺 schema 已有字段；process/user 为当前可选上下文；SNI/HTTP host 为 future；采集失败时相关条件不匹配并可记录按需 debug 事件。 |

## Docs 更新批次

| Batch | 状态 | 范围 |
|---|---|---|
| Batch A | pass | scope/config 文档更新完成。 |
| Batch B | pass | compile/architecture/operations 文档更新完成。 |
| Batch C | pass | match/testing/logging/implementation guide 文档更新完成。 |

## 已修改 Docs 文件清单

- `docs/00-overview/02-scope.md`
- `docs/02-config/00-config-contract.md`
- `docs/02-config/01-acl-schema.md`
- `docs/02-config/02-default-values.md`
- `docs/02-config/03-normalize-rules.md`
- `docs/02-config/05-dns-config.md`
- `docs/02-config/06-route-config.md`
- `docs/02-config/07-rule-set-config.md`
- `docs/03-compile-pipeline/02-basic-validate.md`
- `docs/03-compile-pipeline/03-structural-normalize.md`
- `docs/03-compile-pipeline/05-semantic-validate.md`
- `docs/01-architecture/03-management-control-data-plane.md`
- `docs/05-operations/00-process-bootstrap.md`
- `docs/01-architecture/06-ebpf-architecture.md`
- `docs/04-match-pipeline/09-ebpf-map-sync.md`
- `docs/05-operations/03-internal-rule-set-refresh.md`
- `docs/05-operations/04-cache-cleanup.md`
- `docs/04-match-pipeline/00-orchestration.md`
- `docs/04-match-pipeline/04-context-enrich.md`
- `docs/07-testing/03-match-pipeline-tests.md`
- `docs/07-testing/05-ebpf-tests.md`
- `docs/07-testing/06-performance-testing.md`
- `docs/06-implementation/03-acl-code-guide.md`
- `docs/05-operations/05-logging-and-error.md`
- `docs/02-config/04-logging-config.md`

## 未修改内容

- 未修改源码或 `crates/`。
- 未修改 `codexspec/vision.md` 或 `codexspec/roadmap.md`。
- 未创建 run，未创建正式 planning package。
- 未把本次 preflight 结论写成正式需求冻结。
- 本 closure 轮未继续修改任何 `docs/` 正文。

## 残余风险

- 正式 `$plan commit` 仍必须重新读取更新后的 `docs/`，并可能发现新的需求缺口。
- compile/match pipeline 全节点输入/输出、错误矩阵和配置路径定位规则仍可在后续专门文档轮或正式 planning 中细化。
- 敏感日志当前只保留原则级要求；未来实现阶段如需要更严格验收，可能需要补充敏感字段、摘要策略或 debug dump 边界。
- 性能 hard thresholds 已明确延后，后续数据面或 matcher 立项前仍需补充可量化基线。
- 本次 preflight 参考了 `crates/acl/` schema 作为事实来源，但没有修改源码；后续实现仍需把 docs 与代码差异作为正式任务处理。

## 未来 `$plan commit` 使用方式

后续正式 `$plan commit` 应以更新后的 `docs/` 为权威需求输入，重新选择 scope、拆分 milestone、定义验收标准和退出条件。本次 preflight 的 `summary.md`、`docs-update-plan.md`、`blocker-ledger.md`、`assumptions.md`、`decisions/queue.md` 和三批 docs update report 可作为背景材料，帮助解释为什么文档采用当前口径，但不能替代正式 planning 的需求读取。

## Closure 判定

- ready-for-plan-later: yes
- ready-for-archive: yes
- remaining user decisions: none
- formal requirements frozen: no

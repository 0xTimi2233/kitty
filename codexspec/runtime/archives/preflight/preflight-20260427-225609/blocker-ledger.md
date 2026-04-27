# Blocker Ledger

## 状态说明

- `closed`：用户已决策，后续只需按 `docs-update-plan.md` 修改文档。
- `deferred`：不阻塞当前 `docs/` 更新，但未来正式 planning 可能需要更细化规格。

| ID | Status | 类型 | 阻塞点 | 当前处理结论 | 后续动作 |
|---|---|---|---|---|---|
| B-001 | closed | workflow 边界 | 原先把 Q1 误判为第一轮正式 milestone scope。 | 用户已纠正：本次不是需求立项，不划分 milestone，只做全量需求澄清和 `docs/` 更新准备。 | 保持 preflight/docs 更新轨道。 |
| B-002 | closed | 默认行为不清 | 最小有效配置未定义。 | Batch 001 Q2 已决定严格显式配置。 | 更新配置契约、DNS/route final、basic/semantic validate docs。 |
| B-003 | closed | schema 与需求不一致 | 默认值文档未完整反映 schema，且 `listen_port` 默认 0 未说明。 | Batch 001 Q3 已决定以现有 schema 形状为当前事实来源，同时要求删除 `listen_port` 默认值和约束基础数据类型默认值。 | 更新 default/schema/code guide docs。 |
| B-004 | closed | schema 与 validate 边界不清 | `update_interval` 15 分钟下限不在 schema 层。 | Batch 001 Q3 已确认危险默认和下限进入 basic validate。 | 更新 basic validate、rule_set、internal refresh docs。 |
| B-005 | closed | 冲突处理不清 | DNS/route rule 同时出现 `action` object 与快捷 `server`/`outbound` 的处理策略未定义。 | Batch 001 Q4 已决定允许同时出现，normalize 阶段处理，`action` 优先。 | 更新 normalize、DNS/route config、structural normalize docs。 |
| B-006 | closed | 不可验收表述 | 性能目标未量化。 | Batch 001 Q5 已决定早期只定义 benchmark harness 和数据规模占位。 | 更新 performance testing docs。 |
| B-007 | deferred | 跨模块边界不清 | compile/match pipeline 节点缺少全量最小输入/输出、错误集合和配置路径定位规则。 | 当前 `docs` 更新先覆盖已决策的产品行为；节点级完整 I/O 和错误矩阵可在后续专门文档轮补充。 | 不阻塞当前 docs 修改准备。 |
| B-008 | closed | 安全/权限缺失 | eBPF、privileged ports、敏感配置字段缺少权限、失败和脱敏策略。 | Batch 002 Q2 已关闭 eBPF/privileged port 行为；Batch 002 Q3 与 Batch 003 Q1 已确定 docs 只写“代码不得输出敏感日志”原则，不列字段清单、错误摘要字段清单或 debug dump 边界。 | 更新 eBPF、operations、logging/error docs。 |
| B-009 | closed | rule_set 生命周期缺口 | remote cache 过期刷新失败、stale cache、hash 输入未定义。 | Batch 002 Q4 已决定：启动和 external reload 遇到过期缓存且刷新失败就失败；只有 internal refresh 允许保留旧 runtime；cache hash 使用 URL/path。 | 更新 rule_set、internal refresh、cache cleanup docs。 |
| B-010 | closed | 可观测性目录不足 | stable event、flat fields、config path、错误码和 secret redaction 规则不足。 | Batch 003 Q1 已决定 docs 只写原则：代码不得输出敏感日志；不新增敏感字段清单、错误摘要字段清单或 debug dump 边界。 | 更新 logging/error docs，保留原则级要求。 |
| B-011 | closed | 产品范围表述冲突 | API 在目标中写作“未来 API”，但 architecture/operations 已描述 API 可触发 reload 和配置修改。 | Batch 002 Q1 已决定 API 保留为 future scope；当前 docs 只要求 CLI 和 signal。 | 更新 overview、management/control/data plane、process bootstrap docs。 |
| B-012 | closed | 运行时上下文边界不清 | process、user、SNI、HTTP host 高成本上下文范围和失败行为未定义。 | Batch 002 Q5 已决定当前只把 schema 已有字段作为当前匹配需求；process/user 可选；SNI/HTTP host future；采集失败时该字段条件不匹配并记录按需 debug 事件。 | 更新 scope、match orchestration、context enrich、match tests docs。 |

## 当前阻塞结论

当前 preflight blockers 已关闭。可以开始 `docs` 修改轮。该结论不代表正式需求冻结；后续正式 planning 仍以更新后的 `docs` 为准，并可能继续发现需补充的需求。

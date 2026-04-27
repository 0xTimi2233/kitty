# Assumptions

## 已确认决策

| ID | 来源 | 决策 | 影响 |
|---|---|---|---|
| DC-001 | Batch 001 Q1 | 本次 preflight 不是需求立项，不划分 milestone，不进入 commit planning；本次只澄清需求并准备修改 `docs`。 | 当前所有产物不再使用“首个 milestone”作为推进目标；未来正式立项重新读取更新后的 `docs`。 |
| DC-002 | Batch 001 Q2 | 最小有效配置采用严格显式配置：`dns.servers`、`inbounds`、`outbounds` 至少各一个；运行时需要的 `dns.final` 和 `route.final` 必须显式配置；无匹配 rule 时走显式 final。 | 配置 docs、basic validate 和 semantic validate 需要写入该行为。 |
| DC-003 | Batch 001 Q3 | 保持现有 schema 形状作为当前事实来源，把所有默认值写回 docs，把危险默认放到 basic validate 中拦截。 | `docs/02-config/02-default-values.md` 需要补齐 schema 默认值；validate 文档需要列出危险默认和下限。 |
| DC-004 | Batch 001 Q3 附加约束 | 删除 `listen_port` 默认值；基础数据类型除了 `bool`，其他字段要么 required、要么 `Option`、要么有具体默认值。 | `docs/02-config/00-config-contract.md`、`docs/06-implementation/03-acl-code-guide.md` 和后续 schema 修改需求需要对齐。 |
| DC-005 | Batch 001 Q4 | rule 中 `action` object 与快捷字段可以同时出现；normalize 阶段处理；`action` 优先。 | DNS/route config 和 structural normalize 文档需要改写，不能再写成互斥或 strict error。 |
| DC-006 | Batch 001 Q5 | 早期只定义 benchmark harness 和数据规模占位，等数据面/匹配器需求进一步明确后再给硬阈值。 | 性能测试文档需要把硬阈值延后，并保留规模、baseline、采样方式占位。 |
| DC-007 | Batch 002 Q1 | API 保留为 future scope；当前 docs 明确管理面当前只要求 CLI 和 signal；API 只作为未来能力，不进入当前需求细节。 | 需要改写 scope、architecture 和 operations 中已经像当前能力一样描述 API 的内容。 |
| DC-008 | Batch 002 Q2 | eBPF 是必需能力；权限或 attach 失败时 start/reload 失败；privileged port bind 失败同样失败。 | eBPF/operations docs 不能写 optional fallback。 |
| DC-009 | Batch 002 Q3 custom | 开发代码时约定不输出敏感日志。 | 该原则需要写入 logging/error docs。 |
| DC-010 | Batch 002 Q4 | 启动和 external reload 遇到过期缓存且刷新失败就失败；只有 internal refresh 允许保留旧 runtime；cache hash 使用 URL/path。 | 需要更新 rule_set、internal refresh 和 cache cleanup docs。 |
| DC-011 | Batch 002 Q5 | 当前只把 schema 已有字段作为当前匹配需求；process/user 类字段保留为需要权限/平台能力的可选上下文；SNI/HTTP host 标记为 future；采集失败时该字段条件不匹配，并记录按需 debug 事件。 | 需要更新 scope、match/context enrich 和测试文档。 |
| DC-012 | Batch 003 Q1 | 敏感日志 docs 只写原则，不列字段清单；docs 只写“代码不得输出敏感日志”，具体敏感字段和摘要策略由实现时判断。 | logging/error docs 不新增敏感字段清单、错误摘要字段清单或 debug dump 边界。 |

## 仍可暂时采用的假设

| ID | 假设 | 风险 | 适用范围 |
|---|---|---|---|
| A-001 | 项目只面向 Linux 6.1+，不需要多平台适配。 | 低；文档已明确多平台不在范围。 | 本次 `docs` 更新。 |
| A-002 | ACL schema 只做 decode/default/local normalize，不做基础校验、语义校验、引用解析、下载和 runtime 构建。 | 低；文档和代码方向一致，且 Batch 001 继续确认了 validate 边界。 | 配置、compile pipeline 相关 docs。 |
| A-003 | `docs` 中 pipeline 文档是需求草案，需要补全产品行为和验收口径，但本轮不做架构设计。 | 中；如果写入实现结构会越过 PM 边界。 | `docs-update-plan.md` 和后续 docs 修改准备。 |
| A-004 | 日志和错误消息面向机器/运维使用英文，工作流产物和注释使用中文。 | 低；文档已明确。 | 所有后续 docs 修改。 |

## 非阻塞注意事项

| ID | 注意事项 | 影响 |
|---|---|---|
| C-001 | 当前 preflight blockers 关闭不等于正式需求冻结。 | 后续正式 planning 仍需从更新后的 `docs` 读取需求，并可能继续发现缺口。 |
| C-002 | Batch 003 选择了原则级敏感日志约束，不列字段清单。 | 这降低当前 docs 更新阻力，但未来实现阶段如果需要更严格验收，可能要重新补充正式需求。 |
| C-003 | compile/match pipeline 全节点输入/输出和错误矩阵仍是后续可细化内容。 | 不阻塞当前 docs 修改轮。 |

# Decision Queue

## 已处理决策

| ID | Status | 来源 | 决策结果 | 后续落点 |
|---|---|---|---|---|
| D-001 | resolved | Batch 001 Q1 | 本次不是需求立项，不划分 milestone，只做全量需求澄清和 `docs` 更新准备。 | `brief.md`、`docs-update-plan.md` |
| D-002 | resolved | Batch 001 Q2 | 严格显式配置；`dns.servers`、`inbounds`、`outbounds` 至少各一个；运行时需要的 `dns.final`、`route.final` 必须显式配置；无匹配 rule 走显式 final。 | config、DNS/route、basic/semantic validate docs |
| D-003 | resolved | Batch 001 Q3 | 现有 schema 形状作为当前事实来源；默认值写回 docs；危险默认进入 basic validate；删除 `listen_port` 默认值；基础数据类型除 `bool` 外必须 required、`Option` 或明确 default。 | ACL schema、default values、ACL code guide docs |
| D-004 | resolved | Batch 001 Q4 | 允许 rule `action` object 与快捷字段同时出现；normalize 阶段处理；`action` 优先。 | normalize、DNS/route、structural normalize docs |
| D-005 | resolved | Batch 001 Q5 | 早期只定义 benchmark harness 和数据规模占位，硬阈值等数据面/匹配器需求进一步明确后再定。 | performance testing docs |
| D-006 | resolved | Batch 002 Q1 | API 保留为 future scope；当前 docs 只要求 CLI 和 signal。 | overview、architecture、process bootstrap docs |
| D-007 | resolved | Batch 002 Q2 | eBPF 是必需能力；权限或 attach 失败时 start/reload 失败；privileged port bind 失败同样失败。 | eBPF architecture、eBPF map sync、eBPF tests docs |
| D-008 | resolved | Batch 002 Q3 custom + Batch 003 Q1 | 用户自定义决策：开发代码时约定不输出敏感日志；Batch 003 决定 docs 只写原则，不列敏感字段清单、错误摘要字段清单或 debug dump 边界。 | logging/error docs |
| D-009 | resolved | Batch 002 Q4 | 启动和 external reload 遇到过期缓存且刷新失败就失败；只有 internal refresh 允许保留旧 runtime；cache hash 使用 URL/path。 | rule_set、internal refresh、cache cleanup docs |
| D-010 | resolved | Batch 002 Q5 | 当前只把 schema 已有字段作为当前匹配需求；process/user 可选；SNI/HTTP host future；采集失败时该字段条件不匹配并记录按需 debug 事件。 | scope、match/context、match tests docs |

## 待用户确认决策

暂无下一批用户决策。

## 当前推进口径

当前 preflight blockers 已关闭，可以开始 `docs` 修改轮。该结论只适用于本次已审计 scope 与已发现问题，不代表正式需求冻结。

# Docs Update Plan

## 目标

本计划只准备后续 `docs` 修改，不直接修改 `docs`。后续正式立项将重新从更新后的 `docs` 读取需求，并参考本次 preflight 记录。

当前 preflight blockers 已关闭，可以开始 `docs` 修改轮。该结论不代表正式需求冻结。

## 推荐起始 batch

建议从配置契约和 scope 文档开始，因为它们会影响后续所有文档的措辞：

1. `docs/00-overview/02-scope.md`
2. `docs/02-config/00-config-contract.md`
3. `docs/02-config/01-acl-schema.md`
4. `docs/02-config/02-default-values.md`
5. `docs/02-config/03-normalize-rules.md`
6. `docs/02-config/05-dns-config.md`
7. `docs/02-config/06-route-config.md`
8. `docs/02-config/07-rule-set-config.md`

## 完整修改顺序

| Order | File | 变更摘要 | 对应决策或 blocker |
|---|---|---|---|
| 1 | `docs/00-overview/02-scope.md` | 拆清 current/future/non-goal：API 是 future；eBPF 是当前必需能力；SNI/HTTP host 是 future；process/user 为当前可选上下文。 | DC-007, DC-008, DC-011 |
| 2 | `docs/02-config/00-config-contract.md` | 写入严格显式配置、required/default/Option 规则和基础数据类型约束。 | DC-002, DC-004 |
| 3 | `docs/02-config/01-acl-schema.md` | 补充 ACL schema 只负责 decode/default/local normalize；危险默认和下限归 basic validate。 | DC-003, DC-004 |
| 4 | `docs/02-config/02-default-values.md` | 补齐所有 schema 默认值；标注 `listen_port` 不应有默认值。 | DC-003, DC-004 |
| 5 | `docs/02-config/03-normalize-rules.md` | 写明 action object 与快捷字段同时出现时由 structural normalize 处理，且 `action` 优先。 | DC-005 |
| 6 | `docs/02-config/05-dns-config.md` | 写入 DNS final 必须显式配置、DNS rule action precedence、DNS cache/lazy/dump 默认启用状态。 | DC-002, DC-003, DC-005 |
| 7 | `docs/02-config/06-route-config.md` | 写入 route final 必须显式配置、无匹配 rule 走 final、route rule action precedence。 | DC-002, DC-005 |
| 8 | `docs/02-config/07-rule-set-config.md` | 写入 `update_interval < 15m` 属于 basic validate；启动/external reload 过期缓存刷新失败即失败；internal refresh 可保留旧 runtime；cache hash 使用 URL/path。 | DC-003, DC-010 |
| 9 | `docs/03-compile-pipeline/02-basic-validate.md` | 列入空数组、空字符串、`listen_port` 必填/合法范围、duration 下限、`update_interval` 下限等局部 validate 项。 | DC-002, DC-003, DC-004 |
| 10 | `docs/03-compile-pipeline/03-structural-normalize.md` | 写入 action precedence normalize、shortcut action 展开和 domain_resolver 覆盖关系。 | DC-005 |
| 11 | `docs/03-compile-pipeline/05-semantic-validate.md` | 写入 `dns.final`、`route.final`、action target、detour、引用类型、重复 tag 的语义校验。 | DC-002 |
| 12 | `docs/01-architecture/03-management-control-data-plane.md` | 改写管理面：当前 CLI/signal；API future，不写当前 API reload/config 修改能力。 | DC-007 |
| 13 | `docs/05-operations/00-process-bootstrap.md` | 移除或改写当前 API 运行期管理能力，保留 API future scope。 | DC-007 |
| 14 | `docs/01-architecture/06-ebpf-architecture.md` | 写入 eBPF 是必需能力，权限或 attach 失败导致 start/reload 失败。 | DC-008 |
| 15 | `docs/04-match-pipeline/09-ebpf-map-sync.md` | 写入 eBPF map sync/attach/权限失败的产品行为。 | DC-008 |
| 16 | `docs/05-operations/03-internal-rule-set-refresh.md` | 对齐 `update_interval` 默认、0/0s 禁用、15 分钟下限和 internal refresh 失败保留旧 runtime。 | DC-003, DC-010 |
| 17 | `docs/05-operations/04-cache-cleanup.md` | 补 rule_set cache hash、CacheKeepSet 和过期缓存失败策略，不写 stale-cache fallback。 | DC-010 |
| 18 | `docs/04-match-pipeline/00-orchestration.md` | 写入当前高成本上下文范围：process/user 当前可选，SNI/HTTP host future。 | DC-011 |
| 19 | `docs/04-match-pipeline/04-context-enrich.md` | 写入 process/user 采集需要权限/平台能力，采集失败则相关字段条件不匹配并记录按需 debug 事件。 | DC-011 |
| 20 | `docs/07-testing/03-match-pipeline-tests.md` | 对齐 context enrich 当前范围，避免要求 SNI/HTTP host 当前测试。 | DC-011 |
| 21 | `docs/07-testing/05-ebpf-tests.md` | 对齐 eBPF 必需能力和失败行为测试边界。 | DC-008 |
| 22 | `docs/07-testing/06-performance-testing.md` | 写入早期只定义 benchmark harness 和数据规模占位，硬阈值后续再定。 | DC-006 |
| 23 | `docs/06-implementation/03-acl-code-guide.md` | 对齐 schema 字段规则：基础数据类型除 `bool` 外必须 required、`Option` 或显式 default；`listen_port` 不允许隐式 default。 | DC-004 |
| 24 | `docs/05-operations/05-logging-and-error.md` | 写入原则级敏感日志要求：开发代码时约定不输出敏感日志；不新增敏感字段清单、错误摘要字段清单或 debug dump 边界。 | DC-009, DC-012 |
| 25 | `docs/02-config/04-logging-config.md` | 对齐原则级敏感日志要求和 config path/error msg 方向；不新增敏感字段清单。 | DC-009, DC-012 |

## 后续可延后细化

| File/Area | 延后原因 |
|---|---|
| `docs/03-compile-pipeline/*` 全节点输入/输出和错误矩阵 | 当前 preflight 已解决产品默认行为；完整节点级 contract 可作为后续专门 docs 轮。 |
| `docs/04-match-pipeline/*` 全节点输入/输出和错误矩阵 | 当前只需先对齐 context/current scope；完整 matcher contract 后续细化。 |
| `docs/06-implementation/04-ksr-binary-format.md` | Batch 002 已决定 cache 失败行为和 hash 输入；KSR version compatibility 可在 KSR 具体需求轮细化。 |

## 修改策略

- 按上表顺序修改，先修 scope/config，再修 pipeline/operations/testing。
- 所有修改只写产品需求、默认行为和验收口径，不写架构实现方案或测试计划细节。
- 不把当前 preflight 结论写成正式需求冻结；只写为“当前 docs 更新所需决策已收口”。

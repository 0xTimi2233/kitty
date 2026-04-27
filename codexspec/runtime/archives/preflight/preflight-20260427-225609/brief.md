# Preflight Brief

## 当前结论

本次 preflight 的审计、需求澄清和三批 `docs/` 更新轮已经完成。当前没有剩余用户决策，也没有新的 PM blocker 需要继续提出。

需要准确区分两件事：

- 当前 preflight blockers 已关闭：是。意思是本轮审计已经把已发现的配置默认值、最小有效配置、API scope、eBPF 失败策略、rule_set 失败策略、context enrich、敏感日志原则等阻塞 `docs/` 更新的问题都收口，并已通过三批文档更新写回相应 `docs/` 文件。
- 正式需求冻结：否。未来正式立项仍会重新从更新后的 `docs/` 读取需求，并参考本次 preflight 记录；正式 planning/design/execute 过程中仍可能暴露新的需求缺口或需要补充决策。

对用户追问“此外就没了？澄清全部需求了吗？”的准确回答是：

> 就当前 preflight 审计范围和当前已发现的 blockers 而言，阻塞 `docs/` 更新的需求澄清已经完成，三批 `docs/` 更新也已经完成，暂无下一批用户决策；但这不等于项目需求永久完整冻结。后续正式 `$plan commit` 应以更新后的 `docs/` 作为正式需求来源，并参考本次 preflight 记录。

## 已关闭的关键决策

- 本次不是需求立项，不划分 milestone，不进入 commit planning，只做需求澄清和 `docs/` 更新准备。
- 最小有效配置采用严格显式配置。
- 现有 schema 形状作为当前事实来源；默认值写回 docs；危险默认进入 basic validate；删除 `listen_port` 默认值。
- rule `action` object 与快捷字段可同时出现，normalize 阶段处理，`action` 优先。
- 性能测试早期只定义 benchmark harness 和数据规模占位。
- API 是 future scope，当前管理面只要求 CLI 和 signal。
- eBPF 是必需能力；权限或 attach 失败导致 start/reload 失败；privileged port bind 失败也失败。
- remote rule_set 过期缓存刷新失败时，start/external reload 失败；internal refresh 可保留旧 runtime；cache hash 使用 URL/path。
- process/user 当前可选；SNI/HTTP host future；采集失败时相关字段条件不匹配，并记录按需 debug 事件。
- 敏感日志只写原则：开发代码时约定不输出敏感日志；docs 不新增敏感字段清单、错误摘要字段清单或 debug dump 边界。

## Docs 更新结果

- Batch A 已完成 scope/config 文档更新。
- Batch B 已完成 compile/architecture/operations 文档更新。
- Batch C 已完成 match/testing/logging/implementation guide 文档更新。
- 本次 closure 不再修改 `docs/` 正文。

## Preflight 状态

Status: ready-for-plan-later / ready-for-archive

当前没有必须继续由用户确认的 preflight 决策。`decisions/queue.md` 已记录“暂无下一批用户决策”。

## 后续使用方式

未来正式 `$plan commit` 应重新读取更新后的 `docs/`，把其中仍被选中的范围转化为正式 planning package。`summary.md`、`docs-update-plan.md`、`blocker-ledger.md`、`assumptions.md` 和 `decisions/queue.md` 可作为参考材料，但不能替代正式需求读取，也不能被解释为正式需求冻结。

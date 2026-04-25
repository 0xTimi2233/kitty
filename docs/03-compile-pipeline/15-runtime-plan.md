# 15 Runtime Plan

## 职责

Runtime Plan 将已编译配置、索引、规则集、刷新任务和监听变更整理成待发布计划。

## 输入

- `CompiledMatchIndex`
- `InternedConfig`
- `RuleSetMetaSet`
- 当前 Runtime snapshot。

## 输出

- `RuntimeModel`
- `RuntimeApplyPlan`
- `RuleSetRefreshSchedule`

## Runtime Model 内容

- Runtime generation。
- DNS runtime spec。
- Route runtime spec。
- inbound runtime spec。
- outbound runtime spec。
- compiled match index。
- rule order table。
- intern table。
- rule metadata。
- cache metadata。
- eBPF map sync spec。

## RuntimeApplyPlan 内容

- listener add / remove / update。
- log reload plan。
- eBPF map seed plan。
- old Runtime cleanup plan。

## RuleSetRefreshSchedule

- 只包含被引用且启用自动刷新的 remote rule_set。
- 多个短时间内到期的任务可合并触发一次 internal reload。
- 禁用自动刷新的 rule_set 不注册定时任务。

## 约束

- 该阶段只生成 plan，不绑定端口，不发布指针。
- Runtime Model 必须只读。
- Runtime Model 不持有编译阶段临时对象引用。

## 测试要点

- 禁用自动刷新的 rule_set 不进入 schedule。
- 未引用 rule_set 不进入 Runtime Model。
- RuntimeApplyPlan 能表达 listener diff。
- Runtime generation 单调递增。

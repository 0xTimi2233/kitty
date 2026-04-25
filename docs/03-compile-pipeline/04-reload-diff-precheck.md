# 04 Reload Diff Precheck

## 职责

Reload Diff Precheck 在全量编译前判断 reload 类型，避免不必要的重编译。

该阶段只用于 reload。`start` 必须继续进入完整编译流程。

## 输入

- 旧的 `ConfigFingerprint`。
- 新的 `NormalizedConfig`。
- local rule_set 文件元信息。
- 当前 Runtime generation。

## 输出

- `ReloadDecision::Noop`
- `ReloadDecision::LogOnly`
- `ReloadDecision::FullCompile`

## 判断规则

### Noop

满足以下条件时不触发发布：

- 配置语义指纹未变化。
- 被引用 local rule_set 文件未变化。
- 触发来源不是 internal rule_set refresh。

### LogOnly

只有日志配置变化，且被引用 local rule_set 文件未变化时，只替换日志配置。

### FullCompile

以下任一条件成立时进入全量编译：

- DNS、inbound、outbound、route、rule_set 配置变化。
- 被引用 local rule_set 文件变化。
- 被引用 remote rule_set refresh 成功并产生新内容。
- listener apply plan 发生变化。

## 指纹规则

- `ConfigFingerprint` 不存储 rule_set 内容 hash。
- rule_set 内容 hash 存储在各自 cache meta 中。
- remote rule_set 自动刷新任务由发布后的 Runtime Plan 维护。

## 自动刷新合并

同一短时间窗口内的多个 rule_set refresh 应合并成一次 internal reload，避免频繁发布 Runtime Model。

推荐合并窗口：`1m`。

## 测试要点

- no-op reload 不执行 rule_set 下载。
- log-only reload 不编译 match index。
- local rule_set 文件变化会触发 FullCompile。
- 多个 internal refresh 能合并成一次编译。

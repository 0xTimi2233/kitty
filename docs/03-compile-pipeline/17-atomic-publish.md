# 17 Atomic Publish

## 职责

Atomic Publish 将新的 Runtime Model 和相关 apply 结果原子切换为当前运行态。

## 输入

- `RuntimeModel`
- `RuntimeApplyPlan`
- `ListenerApplyPlan`
- 当前 runtime handle。

## 输出

- 新的 current runtime handle。
- publish event。

## 发布流程

1. 执行 listener apply plan。
2. 初始化需要的 Runtime side resource。
3. 写入必要的 eBPF map seed。
4. 原子替换 current runtime handle。
5. 旧 Runtime Model 延迟释放。
6. 执行 cache cleanup。
7. 注册或替换 rule_set refresh schedule。

## start 行为

start 阶段 publish 失败直接启动失败。

## external reload 行为

external reload 编译或 apply 失败时，reload 失败。旧 Runtime Model 保持运行。

## internal rule_set refresh 行为

internal refresh 编译或 apply 失败时，输出 error 日志。旧 Runtime Model 保持运行。

## 约束

- Match Pipeline 只能读取已发布的 Runtime Model。
- 发布过程不得让 Match Pipeline 看到半初始化状态。
- 旧 Runtime Model 的资源清理不得阻塞热路径。

## 测试要点

- publish 后新请求看到新 generation。
- publish 失败时旧 generation 继续可用。
- 旧 Runtime Model 能安全延迟释放。
- internal refresh 失败不影响旧运行态。

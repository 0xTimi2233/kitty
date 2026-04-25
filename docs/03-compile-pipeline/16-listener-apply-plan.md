# 16 Listener Apply Plan

## 职责

Listener Apply Plan 计算 inbound 监听器需要新增、删除或保持的操作，使 Runtime 发布可以安全切换监听状态。

## 输入

- 新 `RuntimeModel`。
- 当前 listener registry。
- 当前 Runtime snapshot。

## 输出

- `ListenerApplyPlan`

## 操作类型

- `Keep`：监听配置未变化。
- `Add`：新增监听器。
- `Remove`：删除旧监听器。
- `Replace`：监听地址、协议或关键参数变化。

## 设计原则

- 编译阶段只生成 plan。
- 绑定端口、挂载 eBPF、创建 socket 等实际动作在 apply 阶段执行。
- apply 失败时必须保留旧 Runtime Model 和旧 listener registry。
- 对外 reload 不应在旧监听释放后才尝试新监听，避免服务空窗。

## listener 类型

- DNS inbound。
- direct inbound。
- socks inbound。
- vless inbound。
- tc inbound。

## 测试要点

- 未变化监听器生成 Keep。
- 地址变化生成 Replace。
- 新增 inbound 生成 Add。
- apply 失败不会破坏旧 listener registry。

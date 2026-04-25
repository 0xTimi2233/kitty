# 08 Action Dispatch

## 职责

Action Dispatch 根据匹配结果执行 DNS 或 Route action。

## 输入

- `MatchedRule` 或 fallback action。
- `RuntimeModel`。
- `EnrichedMatchContext`。

## 输出

- DNS response。
- Proxy route decision。
- Reject decision。
- eBPF sync task。

## DNS action

- `route`：选择 DNS server，并按 server runtime spec 执行查询。
- `reject`：返回拒绝结果。
- `predefined`：返回预定义响应。

## Route action

- `route`：选择 Direct 或 outbound。
- `reject`：拒绝连接。

## 约束

- action target 已在编译期解析为 ID 或内部 action。
- dispatch 阶段不得查找用户配置字符串。
- dispatch 阶段不得修改 Runtime Model。

## 错误处理

- DNS upstream 失败按 DNS runtime 策略处理。
- outbound 建连失败按代理运行策略处理。
- reject action 必须产生明确的 close reason。

## 测试要点

- DNS route 能选择正确 server。
- DNS predefined 能直接返回响应。
- Route reject 能关闭连接。
- Direct action 不依赖用户 outbound tag。

# Listener Apply Plan

## 职责

生成监听变更计划。bind/listen 的真实应用在 publish 阶段执行，失败时不破坏旧 runtime。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。

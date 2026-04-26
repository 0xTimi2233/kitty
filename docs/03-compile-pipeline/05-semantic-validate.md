# Semantic Validate

## 职责

在收集完成后统一校验 tag 重复、引用缺失、引用类型错误、action target 解析、final/detour 固化。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。

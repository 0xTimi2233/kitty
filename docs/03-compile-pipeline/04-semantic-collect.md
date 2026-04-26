# Semantic Collect

## 职责

收集 DNS server tag、inbound tag、outbound tag、rule_set tag、action target、detour、final、resolver 引用，构建 symbol table 与 reference table。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。

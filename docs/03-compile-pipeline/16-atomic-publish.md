# Atomic Publish

## 职责

原子切换 runtime generation。外部 reload 失败返回失败；内部 rule_set refresh 失败只记录错误并保留旧 runtime。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。

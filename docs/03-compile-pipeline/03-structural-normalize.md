# Structural Normalize

## 职责

统一快捷 action 写法、logical rule 与 leaf rule 表达、domain_resolver 覆盖关系和 Rule IR 雏形。此阶段不解析引用是否存在。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。

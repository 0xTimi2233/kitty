# Structural Normalize

## 职责

Structural Normalize 在 basic validate 之后执行，负责统一配置表达，不解析引用是否存在。

本阶段负责：

- 将快捷 action 写法展开为对象 action。
- 处理对象 action 与快捷字段同时出现的 precedence。
- 统一 logical rule 与 leaf rule 表达。
- 固化 domain_resolver 覆盖关系。
- 构建 Rule IR 雏形。

## Action precedence

DNS rule 允许同时出现对象 `action` 和快捷 `server` 字段。二者同时出现时：

- `action` 优先。
- `server` 作为兼容输入被忽略。
- normalize 结果必须等价于对象 `action`。

Route rule 允许同时出现对象 `action` 和快捷 `outbound` 字段。二者同时出现时：

- `action` 优先。
- `outbound` 作为兼容输入被忽略。
- normalize 结果必须等价于对象 `action`。

快捷字段单独出现时：

- DNS `server` 转成 DNS route action。
- route `outbound` 转成 route action。

## Domain resolver

domain_resolver 覆盖关系在本阶段固化为后续 semantic validate 可检查的结构。引用是否存在、引用类型是否正确不在本阶段处理。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。
- 不解析引用是否存在。

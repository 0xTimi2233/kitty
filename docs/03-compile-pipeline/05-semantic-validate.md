# Semantic Validate

## 职责

Semantic Validate 在 semantic collect 完成后执行，负责所有跨对象语义校验。

本阶段必须校验：

- DNS server tag、inbound tag、outbound tag、rule_set tag 不重复。
- `dns.final` 已显式配置，引用存在且引用类型是 DNS server。
- `route.final` 已显式配置，引用存在且引用类型是 outbound。
- DNS rule action target 存在且类型正确。
- route rule action target 存在且类型正确。
- inbound/outbound `detour` 引用存在且类型正确。
- domain resolver 引用存在且类型正确。
- rule_set 引用存在且类型正确。
- final、detour 和 action target 可在 runtime model 中固化。

缺失 final、引用不存在、引用类型错误或重复 tag 都必须失败。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

失败原因必须可定位到配置路径。错误消息使用英文。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。

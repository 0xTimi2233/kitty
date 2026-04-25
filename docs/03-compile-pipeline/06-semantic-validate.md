# 06 Semantic Validate

## 职责

Semantic Validate 解析跨对象引用，并把字符串 target 固化为稳定 ID。

## 输入

- `BasicValidConfig`

## 输出

- `SemanticValidConfig`
- `SymbolTable`
- `ResolvedReferenceSet`

## 引用解析

需要解析的引用包括：

- DNS rule action 指向的 DNS server。
- DNS server detour 指向的 outbound。
- route final 指向的 action target。
- Route rule action 指向的 outbound。
- route default domain resolver 指向的 DNS server。
- rule_set 引用指向的 rule_set tag。
- inbound / outbound 内部引用。

## Direct 解析规则

默认 final 和默认 detour 在编译期固化为内部 Direct action。

用户显式写入 `direct` 时：

1. 如果存在同名 outbound tag，则解析为用户 outbound。
2. 如果不存在同名 outbound tag，则解析为内部 Direct action。

运行态不再根据字符串临时判断 `direct`。

## 输出约束

- Runtime 相关结构不得保留未解析的 action target 字符串。
- 所有 target 必须转换为稳定 ID 或内部 action 枚举。
- 语义错误必须包含可定位的配置路径。

## 非职责

- 不下载 rule_set。
- 不展开 rule_set。
- 不检测回环。
- 不构建 match index。

## 测试要点

- 不存在的 DNS server 引用会失败。
- 不存在的 outbound 引用会失败。
- 用户 outbound tag 为 `direct` 时能正确优先解析。
- 默认 final 不会污染用户 tag 命名空间。

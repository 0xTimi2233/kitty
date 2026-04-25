# 14 Match Index Compile

## 职责

Match Index Compile 将规则条件编译成 Match Pipeline 使用的倒排索引和 bitmap，加速候选规则筛选，同时保持 first-match 语义。

## 输入

- `InternedConfig`
- `ExpandedRuleList`

## 输出

- `CompiledMatchIndex`
- `RuleOrderTable`

## 索引类型

### Domain exact

使用高速 hash set / hash map 保存 exact domain 到候选 rule bitmap 的映射。

### Domain suffix

编译为 suffix 匹配结构，输出候选 rule bitmap。

### Domain keyword

编译为多模式匹配结构，输出候选 rule bitmap。

### Domain regex

编译为延迟 DFA 或等价高性能 regex 自动机，输出候选 rule bitmap。

### CIDR

IPv4 和 IPv6 分开编译，输出候选 rule bitmap。

### Port

port 和 port_range 编译为可快速查询的区间结构或 bitmap。

### Process / User

仅在配置使用相关字段时启用索引，并向 Match Pipeline 暴露上下文补充需求。

## bitmap 语义

- 同一 group 内多个条件是 OR。
- 不同 group 之间是 AND。
- 空 group 表示 ANY。
- bitmap 只用于筛候选，不决定最终命中。

## first-match 保证

索引编译必须保留 `RuleOrderTable`。Match Pipeline 在 bitmap 得到候选后，仍按规则原始顺序执行 evaluator。

## 测试要点

- bitmap 候选结果不得漏掉 slow matcher 会命中的规则。
- first-match evaluator 与 slow oracle 输出一致。
- 空 group 能正确表示 ANY。
- 使用 process/user 字段时才触发上下文补充需求。

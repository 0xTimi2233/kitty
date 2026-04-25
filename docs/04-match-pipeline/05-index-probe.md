# 05 Index Probe

## 职责

Index Probe 使用编译期生成的倒排索引，根据请求上下文快速找到候选规则 bitmap。

## 输入

- `EnrichedMatchContext`
- `CompiledMatchIndex`

## 输出

- `ConditionBitmapSet`

## 查询内容

- exact domain。
- domain suffix。
- domain keyword。
- domain regex。
- destination IP CIDR。
- source IP CIDR。
- destination port。
- source port。
- process matcher。
- user matcher。

## 约束

- Index Probe 不决定最终规则命中。
- Index Probe 不改变规则顺序。
- rule_set 已在编译期展开，运行期不查询 rule_set。

## 空条件

对于规则中未配置的 group，Index Probe 需要把该 group 视为 ANY，而不是空 bitmap。

## 测试要点

- exact domain 查询能返回候选 bitmap。
- CIDR 查询能返回 IPv4 / IPv6 对应候选。
- 未配置 group 不会误过滤规则。
- Index Probe 候选结果不得漏掉 slow matcher 命中的规则。

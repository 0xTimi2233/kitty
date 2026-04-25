# 12 Priority Flattening

## 职责

Priority Flattening 将配置中的默认值、覆盖关系、fallback action 和引用目标全部固化到编译后结构中，避免运行期临时判断。

## 输入

- `ExpandedRuleList`
- `SemanticValidConfig`
- `SymbolTable`

## 输出

- `PriorityFlattenedRules`
- `ResolvedActionSet`

## 需要固化的内容

- route final。
- DNS default server。
- DNS server resolver 覆盖。
- DNS server detour。
- Route rule action target。
- DNS rule action target。
- inbound 默认 action。
- outbound 默认 dial option。

## action 固化

运行态 action 必须是已解析结构，例如：

```text
Direct
Reject
Outbound(outbound_id)
DnsServer(server_id)
Predefined(response_id)
```

不得在运行态保留需要再次查表的 action 字符串。

## Direct 默认值

默认 Direct 是编译期内部 action，不生成虚拟 outbound，也不占用用户 tag。

## first-match

Priority Flattening 不改变规则顺序。它只固化字段和引用，不执行排序优化。

## 测试要点

- 默认 final 能固化为 Direct action。
- 用户同名 outbound 能被显式引用。
- DNS server 覆盖关系不在运行时判断。
- 展平后规则顺序不变。

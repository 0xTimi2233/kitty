# 04 Context Enrich

## 职责

Context Enrich 根据 Runtime Model 中的 matcher 需求，按需补充昂贵上下文。

## 输入

- `NormalizedMatchContext`
- `RuntimeContextRequirement`

## 输出

- `EnrichedMatchContext`

## 可补充内容

- process name。
- process path。
- user name。
- user id。
- sniffed host。
- TLS SNI。
- HTTP host。
- network interface metadata。

## 按需原则

如果当前 Runtime Model 没有使用某类 matcher，则不得主动补充对应上下文。

示例：

- 没有 process matcher：不查询 process。
- 没有 user matcher：不查询 user。
- 没有 sniff 需求：不读取额外 payload。

## 错误处理

上下文补充失败时，根据 matcher 是否强依赖该字段决定：

- 非强依赖字段：记录缺失状态，继续匹配。
- 强依赖字段：该字段相关 matcher 视为不命中。

## 测试要点

- 未使用 process matcher 时不会查询 process。
- process 查询失败不会导致无关规则失败。
- sniff host 存在时可参与 domain matcher。

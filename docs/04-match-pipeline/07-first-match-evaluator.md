# 07 First Match Evaluator

## 职责

First Match Evaluator 按规则原始顺序检查候选规则，确认最终命中的第一条规则。

## 输入

- `CandidateRuleBitmap`
- `RuleOrderTable`
- `EnrichedMatchContext`
- `CompiledRuleList`

## 输出

- `MatchedRule`
- `NoMatch`

## 行为

1. 按 `RuleOrderTable` 顺序遍历候选规则。
2. 对每条候选规则执行完整条件检查。
3. 第一条完整命中的规则即为结果。
4. 没有规则命中时返回 fallback action。

## evaluator 要求

- evaluator 必须与 slow oracle 保持语义一致。
- evaluator 可以读取编译后的 condition group。
- evaluator 不得触发 rule_set 展开。
- evaluator 不得解析 action target 字符串。

## not 语义

`not` 节点在 evaluator 中按规则 IR 语义执行，确保和 compile 阶段的逻辑结构一致。

## 测试要点

- 多规则命中时返回顺序最靠前的规则。
- bitmap 候选顺序不影响 first-match。
- not 条件与 slow oracle 一致。
- fallback action 在无命中时生效。

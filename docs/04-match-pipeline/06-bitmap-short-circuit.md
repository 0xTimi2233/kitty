# 06 Bitmap Short Circuit

## 职责

Bitmap Short Circuit 根据条件组语义合并候选 bitmap，快速排除不可能命中的规则。

## 输入

- `ConditionBitmapSet`
- `RuleGroupLayout`

## 输出

- `CandidateRuleBitmap`

## 合并语义

- 同一 group 内条件为 OR。
- 不同 group 之间为 AND。
- 空 group 为 ANY。
- `not` 节点通过对应补集或 evaluator 处理。

## 短路策略

- 如果某个必需 group 得到空 bitmap，可立即返回无候选。
- bitmap 合并应优先处理候选规模更小的 group。
- 对于无法可靠 bitmap 化的条件，保留为 evaluator 阶段确认。

## first-match

Bitmap Short Circuit 只生成候选集合，不决定命中的第一条规则。最终顺序由 First Match Evaluator 保证。

## 测试要点

- OR group 合并正确。
- AND group 合并正确。
- ANY group 不影响结果。
- bitmap 短路不会改变 slow matcher 结果。

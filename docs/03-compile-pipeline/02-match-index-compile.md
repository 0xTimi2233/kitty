# Match Index Compile

目标是在保持 first-match 语义的前提下减少候选规则数量。

索引建议：

- exact domain 使用 hash set / hash map。
- suffix 使用反向 label 结构。
- keyword 使用多模式匹配自动机。
- regex 使用 lazy DFA。
- CIDR 使用 prefix table。
- 候选集使用 bitmap 表达。

最终结果仍交给 first-match evaluator 校验，确保索引只负责加速，不改变语义。

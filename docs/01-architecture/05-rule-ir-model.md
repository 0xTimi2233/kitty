# Rule IR Model

Rule IR 用于统一 leaf rule、logical rule、rule_set 展开结果和快捷 action 写法。

核心结构：

```text
And([...])
Or([...])
Not(...)
Leaf(condition groups, action)
```

rule_set 展开时，每个 headless rule 分支与外层 rule 做 AND 合并；多个分支保持 OR。条件组内 OR，条件组之间 AND。

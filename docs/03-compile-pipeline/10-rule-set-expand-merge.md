# Rule Set Expand Merge

## 职责

将 rule_set headless rule 分支与外层 rule 条件做 AND 合并；多个分支保持 OR。条件组合并时按组 union，不做笛卡尔积。

## 输入

上一节点输出的编译上下文。

## 输出

更新后的编译上下文或明确失败原因。

## 不变量

- 不在数据面热路径做配置解析。
- 不让 ACL schema 类型泄漏到 runtime model。
- 不跳过 first-match 语义。

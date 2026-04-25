# 08 Rule Set Reference Prune

## 职责

Rule Set Reference Prune 计算当前配置真正被引用的 rule_set 集合，并避免下载、解码、编译未被引用的 rule_set。

## 输入

- `LoopCheckedConfig`
- 规则 IR

## 输出

- `ReferencedRuleSetSet`
- `UnreferencedRuleSetSet`

## 行为

1. 扫描 DNS rule 和 Route rule 中的 rule_set 引用。
2. 只保留实际被引用的 rule_set tag 进入加载阶段。
3. 未被引用的 rule_set 只完成 schema 和 basic validate。
4. 未被引用的 remote rule_set 不下载、不写 cache、不生成 KSR。
5. 未被引用的 local rule_set 不读取、不写 meta。

## 设计原因

rule_set 可能很大，下载和解码成本高。引用剪枝可以让配置中暂存的 rule_set 不影响启动速度和 reload 成本。

## 测试要点

- 未引用 remote rule_set 不触发网络访问。
- 未引用 local rule_set 不触发文件读取。
- 被 DNS rule 引用的 rule_set 会进入加载阶段。
- 被 Route rule 引用的 rule_set 会进入加载阶段。

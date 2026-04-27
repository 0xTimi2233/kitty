# Batch 001 User Response

Recorded at: 2026-04-27T15:26:10Z

## Q1

用户决策：

本次是审计所有需求，不是需求立项，所以不划分 milestone，只澄清需求。本次只修改 `docs/` 文件；后续正式立项时会重新从 `docs/` 读取需求并立为真正的需求，同时参考本次 preflight 记录。

主线程解释：

- 本次 preflight 不进入 commit planning。
- 本次 preflight 不创建正式 milestone scope。
- 后续工作应围绕全量需求澄清和 `docs/` 更新展开。
- 本次 preflight 记录作为未来正式 `$plan commit` 的参考材料，而不是正式需求 package。

## Q2

选择：1

严格显式配置。`dns.servers`、`inbounds`、`outbounds` 至少各一个；运行时需要的 `dns.final` 和 `route.final` 必须显式配置；无匹配 rule 时走显式 final。

## Q3

选择：1，并追加约束。

保持现有 schema 形状作为当前事实来源，但把所有默认值写回 docs，并把危险默认放到 basic validate 中拦截。

追加约束：

- 需要删除 `listen_port` 默认值。
- 基础数据类型除了 `bool`，其他字段要么是必填，要么是 `Option`，要么指定具体默认值。

## Q4

选择：2，并指定 precedence。

允许 rule action object 与快捷字段同时出现，优先 `action`。同时出现会在 normalize 阶段处理。

## Q5

选择：1

早期只定义 benchmark harness 和数据规模占位，等数据面/匹配器 milestone 前再给硬阈值。

# Decision Batch 001

本批问题用于决定是否可以进入第一轮正式 `$plan` commit track。推荐先回答 Q1-Q4；Q5 可以在选择数据面 milestone 前再回答。

## Q1: 第一轮正式 milestone 应该切在哪个范围？

1. ACL/config contract first。先对齐 `docs/02-config/` 与 `crates/acl/`，补齐默认值、basic validate 边界、normalize/action shortcut 语义和 ACL 测试验收。  
   影响：范围最小，能把后续 compile/runtime 的输入契约固定下来；不会产出可运行 gateway。  
   推荐项：是。  
   是否阻塞推进：是。

2. End-to-end minimal DNS gateway。实现最小 DNS start/reload/runtime 路径，只选少量 DNS server/inbound/outbound。  
   影响：更快看到运行效果，但需要同时决定 runtime、listener、cache、final、错误和权限策略。  
   推荐项：否。  
   是否阻塞推进：是。

3. Full architecture skeleton。按 workspace 一次搭完整管理面、控制面、数据面、eBPF skeleton 和所有 pipeline 占位。  
   影响：看起来完整，但验收面太宽，容易变成不可用骨架。  
   推荐项：否。  
   是否阻塞推进：是。

## Q2: 最小有效配置和缺省 final 行为如何定义？

1. 严格显式配置。`dns.servers`、`inbounds`、`outbounds` 至少各一个；运行时需要的 `dns.final` 和 `route.final` 必须显式配置；无匹配 rule 时走显式 final。  
   影响：用户配置更啰嗦，但语义清晰，validate 和 runtime plan 容易验收。  
   推荐项：是。  
   是否阻塞推进：是。

2. 允许空配置。空数组和缺省 final 可以通过 decode/basic validate；只有真正引用或处理流量时才失败或 no-op。  
   影响：便于 schema 测试和空配置启动，但 start/runtime 行为不直观。  
   推荐项：否。  
   是否阻塞推进：是。

3. 隐式默认 target。缺省 final 时使用第一个 DNS server/outbound 或内置 direct。  
   影响：用户上手简单，但会引入隐式路由和安全风险。  
   推荐项：否。  
   是否阻塞推进：是。

## Q3: schema 与 docs 的默认值冲突如何处理？

1. 保持现有 schema 形状作为当前事实来源，但把所有默认值写回 docs，并把危险默认放到 basic validate 中拦截。例如 `lazy_cache.enable = false`、`dump.enable = false` 显式入文档；`listen_port = 0` 是否允许由 basic validate 决定；`update_interval < 15m` 由 basic validate 拒绝。  
   影响：改动最小，职责边界清晰，便于先完成 ACL/config contract。  
   推荐项：是。  
   是否阻塞推进：是。

2. 以 docs 为唯一权威，计划修改 schema 直到完全匹配文档。  
   影响：可以强化文档权威性，但需要逐项判断当前 schema 哪些是 bug，第一轮范围扩大。  
   推荐项：否。  
   是否阻塞推进：是。

3. 暂不处理默认值冲突，在实现中逐步发现再修。  
   影响：短期省事，但会把配置契约问题推迟到 compile/runtime，返工成本高。  
   推荐项：否。  
   是否阻塞推进：是。

## Q4: rule action object 与快捷字段冲突时如何处理？

1. Strict error。同一 rule 只能使用一种 action 表达：DNS rule 在 `action` 与 `server` 中二选一；route rule 在 `action` 与 `outbound` 中二选一；同时出现由 structural normalize 或 basic validate 报错。  
   影响：用户错误暴露早，Rule IR 简单，first-match 行为稳定。  
   推荐项：是。  
   是否阻塞推进：是。

2. 明确 precedence。允许同时出现，但规定 `action` 优先于快捷字段，或快捷字段优先于 `action`。  
   影响：兼容性强，但配置歧义可能被静默接受。  
   推荐项：否。  
   是否阻塞推进：是。

3. 保持 permissive。serde 接受，后续实现按字段出现顺序或内部转换自然结果处理。  
   影响：行为不可预测，不利于验收。  
   推荐项：否。  
   是否阻塞推进：是。

## Q5: 性能目标在早期如何定义？

1. 早期只定义 benchmark harness 和数据规模占位，等数据面/匹配器 milestone 前再给硬阈值。  
   影响：不阻塞 ACL/config contract，又保留性能验收入口。  
   推荐项：是。  
   是否阻塞推进：否，除非 Q1 选择数据面优先。

2. 现在就定义 QPS、延迟、内存、reload 延迟和 eBPF map sync 阈值。  
   影响：目标清晰，但需要测试硬件、baseline 和数据集，当前输入不足。  
   推荐项：否。  
   是否阻塞推进：否，除非 Q1 选择数据面优先。

3. 暂不定义性能相关验收。  
   影响：会削弱“高性能”目标，后续数据面难以判断是否达标。  
   推荐项：否。  
   是否阻塞推进：否，除非 Q1 选择数据面优先。

# Decision Batch 002

本批问题用于继续澄清全量需求，并决定后续 `docs/` 更新是否可以覆盖 API、eBPF/security、rule_set lifecycle、observability 和运行期上下文边界。

## Q1: API 管理面在当前 docs 中如何定位？

1. API 保留为 future scope。当前 docs 明确管理面当前只要求 CLI 和 signal；API 只作为未来能力，不进入当前需求细节。  
   影响：消除“未来 API”和 API reload/config 修改能力之间的冲突；当前 docs 更保守。  
   推荐项：是。  
   是否阻塞推进：是，阻塞全量 scope 一致性。

2. API 作为当前管理面需求，但只允许 reload、状态查询和配置修改，不能启动进程。  
   影响：保留现有 architecture/operations 描述，但需要补 API 权限、错误、鉴权和配置修改边界。  
   推荐项：否。  
   是否阻塞推进：是。

3. 暂时从产品 docs 中移除 API，后续重新设计时再加入。  
   影响：docs 最简单，但会删除已有“未来 API”方向。  
   推荐项：否。  
   是否阻塞推进：是。

## Q2: eBPF 和 privileged port 权限失败时的默认行为是什么？

1. eBPF 是可选加速；权限或 attach 失败时记录稳定事件并回退用户态路径。privileged port bind 失败仍导致对应 listener/start 失败。  
   影响：兼顾可用性和安全；eBPF 不阻塞基本网关能力，但端口绑定失败仍明确暴露。  
   推荐项：是。  
   是否阻塞推进：是，阻塞 eBPF/security docs。

2. eBPF 是必需能力；权限或 attach 失败时 start/reload 失败。privileged port bind 失败同样失败。  
   影响：性能路径更明确，但部署门槛更高，开发/普通用户环境更难运行。  
   推荐项：否。  
   是否阻塞推进：是。

3. 当前 docs 只保留 eBPF 架构和数据布局骨架，不定义运行时 attach、权限失败或降级行为。  
   影响：符合“第一阶段骨架”表述，但 DNS cache fast path 的运行期需求会继续悬空。  
   推荐项：否。  
   是否阻塞推进：是。

## Q3: 敏感字段在日志和错误中如何处理？

1. 默认全量脱敏。`password`、`uuid`、auth header、TLS private/secret-like 字段、完整 URL credential、token-like query 参数不得进入 `msg` 或 structured fields；错误只暴露 config path、字段名和安全摘要。  
   影响：安全边界清晰，但调试时需要额外工具或显式 debug dump。  
   推荐项：是。  
   是否阻塞推进：是，阻塞 logging/error docs。

2. 只脱敏明显 secret，例如 password/token；UUID、server、headers 默认可记录。  
   影响：调试方便，但 VLESS UUID、headers 等可能泄露身份或凭据。  
   推荐项：否。  
   是否阻塞推进：是。

3. 由 log.level 决定，debug 允许输出更完整配置。  
   影响：调试最方便，但误开 debug 的泄露风险高。  
   推荐项：否。  
   是否阻塞推进：是。

## Q4: remote rule_set 过期缓存刷新失败时如何处理？

1. 若存在上一次已验证缓存，启动和 external reload 可使用 stale cache 并记录稳定 warning；无缓存或缓存校验失败则失败；internal refresh 失败始终保留旧 runtime。cache hash 使用 canonical URL 和 rule_set tag，local meta hash 使用 canonical path。  
   影响：可用性最好，行为明确；需要在 docs 标明 stale cache 风险和事件。  
   推荐项：是。  
   是否阻塞推进：是，阻塞 rule_set/cache docs。

2. 启动和 external reload 遇到过期缓存且刷新失败就失败；只有 internal refresh 允许保留旧 runtime。cache hash 使用 URL/path。  
   影响：一致性更强，但网络短暂失败会阻塞启动或 reload。  
   推荐项：否。  
   是否阻塞推进：是。

3. 提供配置开关决定是否允许 stale cache。  
   影响：灵活，但配置契约和测试矩阵更复杂。  
   推荐项：否。  
   是否阻塞推进：是。

## Q5: process/user/SNI/HTTP host 等高成本上下文是否属于当前匹配需求？

1. 当前只把 schema 已有字段作为当前匹配需求：process、user 类字段保留为需要权限/平台能力的可选上下文；SNI/HTTP host 标记为 future。采集失败时该字段条件不匹配，并记录按需 debug 事件。  
   影响：与 schema 更一致，scope 可控；TLS/HTTP 特征匹配延后。  
   推荐项：是。  
   是否阻塞推进：是，阻塞 match/context docs。

2. process、user、SNI、HTTP host 都作为当前匹配需求。采集失败时按配置或默认策略决定 fail-close/fail-open。  
   影响：能力完整，但权限、隐私、协议解析和性能成本显著增加。  
   推荐项：否。  
   是否阻塞推进：是。

3. 当前 match pipeline 不承诺高成本上下文采集，只保留 domain/IP/port/network/rule_set 等低成本字段。  
   影响：热路径最简单，但 schema 中 process/user 字段需要标记为 future 或暂不可用。  
   推荐项：否。  
   是否阻塞推进：是。

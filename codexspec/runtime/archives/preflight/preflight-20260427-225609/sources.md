# Preflight Sources

## 审计范围

本轮只读取 dispatch 允许的协议文件、`codexspec/vision.md`、`codexspec/roadmap.md`、`docs/` 和 `crates/acl/`。未扩展到其他 `crates/` 实现路径。

## 协议与权威输入

| Source | 覆盖结论 |
|---|---|
| `.codex/prompts/subagent-contract.md` | 明确子代理只读 dispatch 授权范围、只写 dispatch 输出路径。 |
| `.codex/prompts/glossary.md` | 明确 preflight、planning session、dispatch、planning package 等工作流术语。 |
| `.codex/prompts/report-contract.md` | 明确标准报告、状态值和 Decision Request 格式。 |
| `codexspec/runtime/preflight/preflight-20260427-225609/dispatch/pm-001.md` | 明确本轮 PM preflight audit 目标、输入输出边界、审计产物和停止条件。 |
| `codexspec/vision.md` | 当前仍为空模板，不能作为稳定产品愿景依据。 |
| `codexspec/roadmap.md` | 只有 `M0 Initialize workflow`，还没有可执行产品 milestone。 |

## 需求文档源

| Source | 覆盖结论 |
|---|---|
| `docs/README.md` | 提供项目目标、workspace 划分和阅读顺序，是需求索引而非可验收规格。 |
| `docs/00-overview/00-reading-order.md` | 说明文档阅读方式。 |
| `docs/00-overview/01-product-goals.md` | 定义 Linux 6.1+ DNS + Proxy Gateway、控制面编译运行态、数据面加速、管理面能力和防腐边界。目标清晰但缺少可量化成功标准。 |
| `docs/00-overview/02-scope.md` | 定义当前协议、inbound/outbound、rule_set、eBPF 第一阶段范围和非目标。需要拆成 milestone。 |
| `docs/00-overview/03-terminology.md` | 定义 ACL、SchemaConfig、Runtime Model、Compile Pipeline、Match Pipeline、KSR 等术语。 |
| `docs/01-architecture/00-layered-ddd.md` | 定义分层 DDD 和防腐目标。 |
| `docs/01-architecture/01-workspace-and-crate-boundary.md` | 定义 crate 职责边界。 |
| `docs/01-architecture/02-dependency-direction.md` | 定义推荐依赖方向和 domain 外部类型隔离约束。 |
| `docs/01-architecture/03-management-control-data-plane.md` | 定义管理面、控制面、数据面职责。 |
| `docs/01-architecture/04-runtime-model.md` | 定义 Runtime Model 不可变、generation handle、编译期固化目标。 |
| `docs/01-architecture/05-rule-ir-model.md` | 定义 Rule IR 形态和 rule_set 合并语义。 |
| `docs/01-architecture/06-ebpf-architecture.md` | 定义 Aya 路线和第一阶段 eBPF DNS cache 边界。 |
| `docs/02-config/00-config-contract.md` | 定义根配置字段和 ACL schema 职责边界。 |
| `docs/02-config/01-acl-schema.md` | 定义 ACL schema 做/不做事项。 |
| `docs/02-config/02-default-values.md` | 列出关键默认值；部分默认启用行为未写全，需与 schema 对齐。 |
| `docs/02-config/03-normalize-rules.md` | 定义 decode-time local normalize 和 structural normalize 边界。 |
| `docs/02-config/04-logging-config.md` | 定义日志字段和反序列化错误输出要求。 |
| `docs/02-config/05-dns-config.md` | 定义 DNS server、rule、final、strategy、cache 和 DNS action 类型。 |
| `docs/02-config/06-route-config.md` | 定义 route rules、rule_set、final、default_domain_resolver 和 route action 类型。 |
| `docs/02-config/07-rule-set-config.md` | 定义 rule_set 类型、format、缓存路径和未引用 rule_set 行为。 |
| `docs/03-compile-pipeline/00-orchestration.md` | 定义 full compile 的 17 个节点顺序。 |
| `docs/03-compile-pipeline/01-config-decode.md` | 定义配置读取、ACL decode、local normalize 和默认值填充。 |
| `docs/03-compile-pipeline/02-basic-validate.md` | 定义局部合法性校验，不做跨对象引用。 |
| `docs/03-compile-pipeline/03-structural-normalize.md` | 定义快捷 action、logical/leaf、domain_resolver 覆盖和 Rule IR 雏形。 |
| `docs/03-compile-pipeline/04-semantic-collect.md` | 定义 tag 与引用收集。 |
| `docs/03-compile-pipeline/05-semantic-validate.md` | 定义重复 tag、引用缺失、引用类型、action/final/detour 固化校验。 |
| `docs/03-compile-pipeline/06-loop-check.md` | 定义 resolver、detour、rule_set 依赖回环检查。 |
| `docs/03-compile-pipeline/07-rule-set-reference-prune.md` | 定义只保留实际引用的 rule_set。 |
| `docs/03-compile-pipeline/08-rule-set-load-cache.md` | 定义 local/remote cache 加载和启动失败策略。 |
| `docs/03-compile-pipeline/09-rule-set-decode-verify.md` | 定义 source 解码和 KSR header 校验边界。 |
| `docs/03-compile-pipeline/10-rule-set-expand-merge.md` | 定义 headless rule 与外层 rule 的 AND/OR 合并。 |
| `docs/03-compile-pipeline/11-priority-flattening.md` | 定义配置优先级编译期固化。 |
| `docs/03-compile-pipeline/12-string-interning.md` | 定义高频字符串驻留。 |
| `docs/03-compile-pipeline/13-match-index-compile.md` | 定义 exact、suffix、keyword、regex、CIDR、port、process、user 索引编译。 |
| `docs/03-compile-pipeline/14-runtime-plan.md` | 定义 RuntimeModel、RuntimeMetadata、RefreshTaskPlan、CacheKeepSet 和回滚边界。 |
| `docs/03-compile-pipeline/15-listener-apply-plan.md` | 定义 listener 变更计划和失败不破坏旧 runtime。 |
| `docs/03-compile-pipeline/16-atomic-publish.md` | 定义 external reload 与 internal refresh 的发布失败策略差异。 |
| `docs/03-compile-pipeline/17-post-publish-maintenance.md` | 定义发布后缓存清理、generation 回收和维护任务。 |
| `docs/04-match-pipeline/00-orchestration.md` | 定义数据面热路径节点顺序和不读取 ACL/raw config。 |
| `docs/04-match-pipeline/01-ingress-classify.md` | 定义入口类型识别。 |
| `docs/04-match-pipeline/02-context-normalize.md` | 定义运行期轻量 normalize。 |
| `docs/04-match-pipeline/03-dns-cache-lookup.md` | 定义用户态 DNS cache 与 eBPF cache hit 边界。 |
| `docs/04-match-pipeline/04-context-enrich.md` | 定义按需补充 process/user/SNI/HTTP host。 |
| `docs/04-match-pipeline/05-index-probe.md` | 定义索引候选 rule id bitmap。 |
| `docs/04-match-pipeline/06-bitmap-short-circuit.md` | 定义 Roaring bitmap 交并差缩小候选集合。 |
| `docs/04-match-pipeline/07-first-match-evaluator.md` | 定义 first-match evaluator。 |
| `docs/04-match-pipeline/08-action-dispatch.md` | 定义 action dispatch 类型。 |
| `docs/04-match-pipeline/09-ebpf-map-sync.md` | 定义 DNS/route action 后用户态写 eBPF map。 |
| `docs/05-operations/00-process-bootstrap.md` | 定义进程启动职责和 API 不能启动进程。 |
| `docs/05-operations/01-start.md` | 定义 start 流程和启动阶段 remote rule_set 失败策略。 |
| `docs/05-operations/02-external-reload.md` | 定义 external reload 来源、diff precheck 和失败保留旧 runtime。 |
| `docs/05-operations/03-internal-rule-set-refresh.md` | 定义 internal refresh 调度、最小 interval、合并窗口和失败策略。 |
| `docs/05-operations/04-cache-cleanup.md` | 定义 cache cleanup 触发条件、不触发条件和删除范围。 |
| `docs/05-operations/05-logging-and-error.md` | 定义日志事件模型和错误/日志英文要求。 |
| `docs/06-implementation/00-project-directory.md` | 定义项目目录和测试目录约束。 |
| `docs/06-implementation/01-coding-standard.md` | 定义 Rust edition、热路径、注释和错误语言规范。 |
| `docs/06-implementation/02-dependency-selection.md` | 定义按需引入依赖和推荐库方向。 |
| `docs/06-implementation/03-acl-code-guide.md` | 定义 ACL crate 内部结构和 required/default/Option 规则。 |
| `docs/06-implementation/04-ksr-binary-format.md` | 定义 KSR header 最小字段和深层语义边界。 |
| `docs/07-testing/00-testing-standard.md` | 定义测试目录和 slow matcher oracle 要求。 |
| `docs/07-testing/01-acl-tests.md` | 定义 ACL 测试覆盖主题。 |
| `docs/07-testing/02-compile-pipeline-tests.md` | 定义 compile pipeline 测试覆盖主题。 |
| `docs/07-testing/03-match-pipeline-tests.md` | 定义 match pipeline 测试覆盖主题。 |
| `docs/07-testing/04-reload-and-cache-tests.md` | 定义 reload/cache 测试覆盖主题。 |
| `docs/07-testing/05-ebpf-tests.md` | 定义 eBPF 测试覆盖主题。 |
| `docs/07-testing/06-performance-testing.md` | 定义 benchmark 覆盖主题和需定义规模、baseline、阈值、采样方式。 |
| `docs/07-testing/07-fixtures.md` | 定义 fixture 分类和预期结果要求。 |

## Schema 源

| Source | 覆盖结论 |
|---|---|
| `crates/acl/Cargo.toml` | ACL crate 当前仅引入 serde、serde_with、serde_repr、humantime、humantime-serde 和 serde_json 测试依赖。 |
| `crates/acl/src/lib.rs` | ACL crate 仅公开 `helper` 与 `schema`。 |
| `crates/acl/src/helper.rs` | helper 模块索引。 |
| `crates/acl/src/helper/string.rs` | 支撑 trim、lowercase、domain_suffix normalize。 |
| `crates/acl/src/helper/serde_string.rs` | 支撑 Option 空串转 None、one-or-many 字符串 normalize。 |
| `crates/acl/src/helper/one_or_many.rs` | 支撑单值或数组反序列化为 Vec。 |
| `crates/acl/src/helper/duration.rs` | 支撑 rule_set update_interval 默认 1d 和 0/0s 禁用；未约束 15 分钟下限。 |
| `crates/acl/src/schema.rs` | schema 模块索引。 |
| `crates/acl/src/schema/root.rs` | 支撑根字段 `log` 默认、`dns`、`inbounds`、`outbounds`、`route` 必填。 |
| `crates/acl/src/schema/defaults.rs` | 支撑 DNS port、DoH path、TCP/UDP timeout、connect timeout、network、SOCKS/VLESS 默认值。 |
| `crates/acl/src/schema/log.rs` | 支撑 log level/format/timestamp 默认和 deny unknown fields。 |
| `crates/acl/src/schema/dns.rs` | 支撑 DNS servers、rules、final、strategy、cache。 |
| `crates/acl/src/schema/dns/server.rs` | 支撑 tcp/udp/tls/quic/https/h3 DNS server 和默认端口。 |
| `crates/acl/src/schema/dns/cache.rs` | 支撑 DNS cache 默认启用、lazy_cache 默认禁用、dump 默认禁用。 |
| `crates/acl/src/schema/dns/rule.rs` | 支撑 DNS leaf/logical rule、query_type、action/server shortcut 字段。 |
| `crates/acl/src/schema/dns/action.rs` | 支撑 DNS route/reject/predefined action。 |
| `crates/acl/src/schema/route.rs` | 支撑 route rules、rule_set、final、default_domain_resolver。 |
| `crates/acl/src/schema/route/rule.rs` | 支撑 route leaf/logical rule、action/outbound shortcut 字段。 |
| `crates/acl/src/schema/route/action.rs` | 支撑 route/reject action。 |
| `crates/acl/src/schema/rule_set.rs` | 支撑 local/remote rule_set、source/binary、remote update_interval。 |
| `crates/acl/src/schema/common.rs` | common 模块索引。 |
| `crates/acl/src/schema/common/rule.rs` | 支撑 logical mode、network、IP version、query type、DNS strategy、SOCKS/VLESS enum。 |
| `crates/acl/src/schema/common/shared.rs` | 支撑 domain_resolver、TLS、multiplex、V2Ray transport、UDP over TCP。 |
| `crates/acl/src/schema/common/action.rs` | 支撑 reject action 默认 method/no_drop。 |
| `crates/acl/src/schema/inbound.rs` | 支撑 direct、socks、vless、dns、tc inbound 及共享监听字段。 |
| `crates/acl/src/schema/outbound.rs` | 支撑 direct、socks、vless outbound 及共享拨号字段。 |
| `crates/acl/src/schema/macros.rs` | schema macro 模块索引。 |
| `crates/acl/src/schema/macros/dns_server.rs` | 定义 DNS server 共享字段和默认端口注解。 |
| `crates/acl/src/schema/macros/inbound.rs` | 定义 inbound 共享字段，`listen_port` 当前默认 0。 |
| `crates/acl/src/schema/macros/outbound.rs` | 定义 outbound 共享字段和 connect/TCP 默认值。 |
| `crates/acl/src/schema/macros/matcher.rs` | 定义 matcher 公共字段、one-or-many 和 normalize 注解。 |
| `crates/acl/tests/schema_decode.rs` | 当前覆盖部分默认值、必填字段、normalize、update_interval、DNS strategy 和 VLESS packet encoding；未覆盖全部 docs 声明的 ACL 测试主题。 |

## 覆盖总评

`docs/` 已覆盖愿景、分层、配置、pipeline、operations、实现规范和测试主题，但多数内容仍是职责/边界说明，缺少正式 planning 所需的 milestone 顺序、默认行为、冲突处理和可验收口径。`crates/acl/` schema 已覆盖大部分配置形状与默认值，但与文档在若干默认启用、危险默认和 validation 边界上需要显式对齐。

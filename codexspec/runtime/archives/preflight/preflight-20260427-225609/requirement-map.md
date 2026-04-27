# Requirement Map

## 功能需求

| ID | 需求 | 来源 | 清晰度 | `crates/acl/` schema 支撑 | 后续 planning 影响 |
|---|---|---|---|---|---|
| F-001 | Kitty 是 Linux 6.1+ DNS + Proxy Gateway，控制面编译配置，数据面执行分流。 | `docs/README.md`, `docs/00-overview/01-product-goals.md` | 中 | 部分支撑配置入口，不支撑 runtime 行为 | 需要拆成可执行 milestone；当前目标过大。 |
| F-002 | 当前支持 DNS server 类型 `tcp`、`udp`、`tls`、`quic`、`https`、`h3`。 | `docs/00-overview/02-scope.md`, `docs/02-config/05-dns-config.md` | 高 | 已支撑 | 需要明确首个 milestone 是否实现全部协议还是仅 schema。 |
| F-003 | 当前 inbound 类型为 `direct`、`socks`、`vless`、`dns`、`tc`。 | `docs/00-overview/02-scope.md` | 高 | 已支撑 | 需要明确哪些 inbound 进入首个可运行版本。 |
| F-004 | 当前 outbound 类型为 `direct`、`socks`、`vless`。 | `docs/00-overview/02-scope.md` | 高 | 已支撑 | 需要明确 SOCKS/VLESS 的认证、TLS、transport 是否进入首个可运行版本。 |
| F-005 | 管理面支持 CLI、signal，未来 API 可触发 reload、状态查询和配置修改。 | `docs/00-overview/01-product-goals.md`, `docs/01-architecture/03-management-control-data-plane.md`, `docs/05-operations/00-process-bootstrap.md` | 中 | 不适用 | API 被描述为未来能力但又出现在 operations，需要明确是否非目标。 |
| F-006 | eBPF 第一阶段处理 UDP/53 DNS cache hit，miss/TCP 回用户态，仅 A/AAAA 进入 eBPF DNS cache。 | `docs/00-overview/02-scope.md`, `docs/01-architecture/06-ebpf-architecture.md` | 中 | 不支撑 | 需要单独 milestone 和权限边界，不应默认进入配置/schema milestone。 |

## 配置与 Schema 需求

| ID | 需求 | 来源 | 清晰度 | `crates/acl/` schema 支撑 | 后续 planning 影响 |
|---|---|---|---|---|---|
| C-001 | 根配置字段为 `log`、`dns`、`inbounds`、`outbounds`、`route`。 | `docs/02-config/00-config-contract.md` | 高 | 已支撑；`log` 默认，其他根字段必填 | 需要明确空 `inbounds`/`outbounds`/`dns.servers` 是否可通过后续 validate。 |
| C-002 | ACL schema 负责 JSON 输入结构、serde、默认值、one-or-many、trim/lowercase；不做基础校验、语义校验、引用解析、下载和 runtime 构建。 | `docs/02-config/01-acl-schema.md` | 高 | 已支撑主要机制 | 后续 planning 应把 validate/normalize/semantic 阶段边界写成验收条件。 |
| C-003 | `log.level` 默认 `warn`、`format` 默认 `text`、`timestamp` 默认 `true`，允许枚举值明确。 | `docs/02-config/02-default-values.md`, `docs/02-config/04-logging-config.md` | 高 | 已支撑并有部分测试 | 可进入 planning。 |
| C-004 | DNS cache 默认启用，capacity 默认 4096，lazy ttl/reply_ttl、dump path/interval 有默认值。 | `docs/02-config/02-default-values.md`, `docs/02-config/05-dns-config.md` | 中 | 部分支撑；`lazy_cache.enable` 和 `dump.enable` 默认禁用但文档未声明 | 需要对齐默认启用策略，否则 runtime cache 行为不可验收。 |
| C-005 | DNS server 默认端口和 DoH path。 | `docs/02-config/02-default-values.md`, `crates/acl/src/schema/dns/server.rs` | 高 | 已支撑 | 可进入 planning。 |
| C-006 | inbound/outbound TCP/UDP keepalive、timeout、connect timeout 和 outbound network 默认。 | `docs/02-config/02-default-values.md` | 中 | 已部分支撑；inbound `listen_port` 默认 0 未在文档说明 | 需要决定 `listen_port = 0` 是允许、schema 默认还是 basic validate 错误。 |
| C-007 | DNS rule 支持 leaf/logical、`route`、`reject`、`predefined`，快捷 `server` 写法由 structural normalize 转为对象 action。 | `docs/02-config/05-dns-config.md` | 中 | 已支撑字段；未强制 action 与 shortcut 互斥 | 需要定义冲突/缺省 action 语义。 |
| C-008 | Route rule 支持 leaf/logical、`route`、`reject`，快捷 `outbound` 写法由 structural normalize 转为对象 action。 | `docs/02-config/06-route-config.md` | 中 | 已支撑字段；未强制 action 与 shortcut 互斥 | 需要定义冲突/缺省 action 语义。 |
| C-009 | rule_set 支持 local/remote、source/binary，remote `update_interval` 缺省 1d，0/0s 禁用。 | `docs/02-config/07-rule-set-config.md`, `docs/05-operations/03-internal-rule-set-refresh.md` | 中 | 已支撑类型和 1d/0；15 分钟下限未在 schema 支撑 | 需要明确 15 分钟下限归属 basic validate，并写入验收。 |
| C-010 | decode-time normalize 包括 tag/domain trim+lowercase、domain_suffix 去前导点、regex/path 只 trim、Option 空串为 None。 | `docs/02-config/03-normalize-rules.md` | 高 | 已支撑主要 string helper 和 matcher macro | 需要补齐测试覆盖清单。 |

## 运行时行为

| ID | 需求 | 来源 | 清晰度 | `crates/acl/` schema 支撑 | 后续 planning 影响 |
|---|---|---|---|---|---|
| R-001 | Compile Pipeline 从 config-decode 到 post-publish-maintenance 共 17 个节点。 | `docs/03-compile-pipeline/00-orchestration.md` | 中 | 仅支撑 config-decode 输入 | 需要决定首个 milestone 是 ACL/config contract 还是完整 pipeline 骨架。 |
| R-002 | basic validate、structural normalize、semantic collect/validate、loop-check 有明确职责边界。 | `docs/03-compile-pipeline/02-basic-validate.md` 至 `06-loop-check.md` | 中 | 不支撑；schema 明确不负责 | 需要定义可验收错误集合和配置路径定位规则。 |
| R-003 | rule_set 引用裁剪、cache 加载、decode/verify、expand/merge、priority flattening。 | `docs/03-compile-pipeline/07-rule-set-reference-prune.md` 至 `11-priority-flattening.md` | 中 | 部分支撑 rule_set shape | rule_set 行为复杂，建议独立 milestone。 |
| R-004 | Runtime Model 发布后不可变，generation handle 原子切换，旧 generation 无引用后释放。 | `docs/01-architecture/04-runtime-model.md`, `docs/03-compile-pipeline/16-atomic-publish.md` | 中 | 不支撑 | 需要 Architect 设计前先确认可靠性退出条件。 |
| R-005 | Match Pipeline 只依赖 RuntimeModel，不读取 ACL/raw config，保持 first-match 语义。 | `docs/04-match-pipeline/00-orchestration.md` | 中 | 不支撑 | 需要 first-match、候选 bitmap 与 slow oracle 验收口径。 |
| R-006 | action dispatch 执行 Direct、Reject、Outbound、DNS route、predefined。 | `docs/04-match-pipeline/08-action-dispatch.md` | 中 | 部分支撑 action schema | action target、reject 默认和 predefined 响应格式需要补充。 |
| R-007 | Start、external reload、internal refresh 的失败策略不同。 | `docs/05-operations/01-start.md` 至 `03-internal-rule-set-refresh.md` | 中 | 不支撑 | 需要作为 reload/cache milestone 的验收标准。 |

## 性能与可靠性

| ID | 需求 | 来源 | 清晰度 | `crates/acl/` schema 支撑 | 后续 planning 影响 |
|---|---|---|---|---|---|
| P-001 | 数据面热路径追求极致性能、低分支、低分配。 | `docs/README.md`, `docs/00-overview/01-product-goals.md`, `docs/06-implementation/01-coding-standard.md` | 低 | 不适用 | 缺少 QPS、延迟、内存、数据规模和平台基线；会阻塞性能验收。 |
| P-002 | 不在数据面热路径解析配置，不做临时解析、动态查找和不必要分支。 | 多个 compile/match pipeline 文档 | 中 | 通过 schema 边界间接支撑 | 可作为设计约束，但需要测试/benchmark 口径。 |
| P-003 | publish/listener 失败不破坏旧 runtime，maintenance 失败不回滚 runtime。 | `docs/03-compile-pipeline/15-listener-apply-plan.md`, `16-atomic-publish.md`, `17-post-publish-maintenance.md` | 中 | 不支撑 | 需要明确失败分类和用户可见结果。 |

## 安全与权限

| ID | 需求 | 来源 | 清晰度 | `crates/acl/` schema 支撑 | 后续 planning 影响 |
|---|---|---|---|---|---|
| S-001 | Linux 6.1+、Aya eBPF、监听 53/853/443 等能力可能需要权限。 | `docs/README.md`, `docs/01-architecture/06-ebpf-architecture.md`, `docs/02-config/02-default-values.md` | 低 | 不支撑 | 缺少运行权限模型、失败提示和降级策略。 |
| S-002 | SOCKS/VLESS 用户、UUID、TLS insecure、headers 等敏感配置字段。 | `crates/acl/src/schema/inbound.rs`, `outbound.rs`, `common/shared.rs` | 中 | 已支撑字段 | 缺少 secret 日志脱敏和配置错误暴露边界。 |

## 运维与可观测性

| ID | 需求 | 来源 | 清晰度 | `crates/acl/` schema 支撑 | 后续 planning 影响 |
|---|---|---|---|---|---|
| O-001 | 日志使用 `timestamp level event msg fields`，event 是稳定枚举，错误和日志消息英文。 | `docs/05-operations/05-logging-and-error.md` | 中 | 支撑 log 配置，不支撑 event catalog | 需要事件枚举和关键字段最小集合。 |
| O-002 | 反序列化错误要带 config path；非反序列化阶段也要维护可定位配置路径。 | `docs/02-config/04-logging-config.md` | 中 | serde 错误有基础能力，但路径策略不完整 | 需要统一 error model 前置决定。 |
| O-003 | Cache cleanup 删除不在 keep set 的 remote/local/KSR/meta/temp 文件。 | `docs/05-operations/04-cache-cleanup.md` | 中 | 不支撑 | 需要明确缓存根目录、hash 输入和保留策略。 |

## 测试与验收

| ID | 需求 | 来源 | 清晰度 | `crates/acl/` schema 支撑 | 后续 planning 影响 |
|---|---|---|---|---|---|
| T-001 | 测试不写在 `src/` 下，ACL、控制面、数据面、跨 crate 测试位置明确。 | `docs/07-testing/00-testing-standard.md`, `docs/06-implementation/00-project-directory.md` | 高 | 当前 ACL tests 路径符合 | 可进入 planning。 |
| T-002 | ACL tests 覆盖 required/default/Option、one-or-many、normalize、unknown field、enum decode。 | `docs/07-testing/01-acl-tests.md`, `crates/acl/tests/schema_decode.rs` | 中 | 当前只覆盖部分主题 | 需要补齐为 ACL/config contract milestone 的退出条件。 |
| T-003 | compile、match、reload/cache、eBPF、performance tests 有主题要求。 | `docs/07-testing/02-compile-pipeline-tests.md` 至 `06-performance-testing.md` | 低到中 | 不支撑 | 需要随 milestone 逐步细化，不能一次作为全项目验收。 |
| T-004 | 必须维护 slow matcher oracle，与 indexed matcher 做差分测试。 | `docs/07-testing/00-testing-standard.md` | 中 | 不支撑 | 是 match/index milestone 的关键验收，不阻塞 ACL milestone。 |

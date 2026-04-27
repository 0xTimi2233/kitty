# Docs Update Batch B Report

Status: pass

## Scope

本轮只修改 dispatch 允许的 9 个 compile/architecture/operations 文档：

- `docs/03-compile-pipeline/02-basic-validate.md`
- `docs/03-compile-pipeline/03-structural-normalize.md`
- `docs/03-compile-pipeline/05-semantic-validate.md`
- `docs/01-architecture/03-management-control-data-plane.md`
- `docs/05-operations/00-process-bootstrap.md`
- `docs/01-architecture/06-ebpf-architecture.md`
- `docs/04-match-pipeline/09-ebpf-map-sync.md`
- `docs/05-operations/03-internal-rule-set-refresh.md`
- `docs/05-operations/04-cache-cleanup.md`

未修改源码、`codexspec/vision.md`、`codexspec/roadmap.md`、Batch A 文件或其他 `docs/` 文件。

## Decisions Applied

- Basic validate 覆盖空数组、空字符串、`listen_port` 显式配置与合法范围、duration 下限、`update_interval` 下限等局部合法性。
- Structural normalize 处理 action precedence、shortcut action 展开和 domain_resolver 覆盖关系。
- Semantic validate 处理 `dns.final`、`route.final`、action target、detour、引用类型和重复 tag。
- 当前管理面只要求 CLI 和 signal；API 是 future scope。
- eBPF 是必需能力；权限、load 或 attach 失败导致 start/reload 失败。
- eBPF map sync 失败必须产生可观测事件；external reload 的 eBPF 失败不得替换旧 runtime。
- internal refresh 对齐 `update_interval` 默认、0/0s 禁用、15 分钟下限和失败保留旧 runtime。
- cache cleanup 明确 `CacheKeepSet`、hash 输入和过期缓存刷新失败不使用 stale-cache fallback。

## Notes

本轮更新仍属于 preflight docs update，不是 formal planning，也不表示正式需求冻结。后续正式立项时仍应从更新后的 `docs/` 重新读取需求。

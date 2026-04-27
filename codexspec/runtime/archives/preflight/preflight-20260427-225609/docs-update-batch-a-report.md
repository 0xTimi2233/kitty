# Docs Update Batch A Report

Status: pass

## Scope

本轮只修改 dispatch 允许的 8 个 scope/config 文档：

- `docs/00-overview/02-scope.md`
- `docs/02-config/00-config-contract.md`
- `docs/02-config/01-acl-schema.md`
- `docs/02-config/02-default-values.md`
- `docs/02-config/03-normalize-rules.md`
- `docs/02-config/05-dns-config.md`
- `docs/02-config/06-route-config.md`
- `docs/02-config/07-rule-set-config.md`

未修改源码、`codexspec/vision.md`、`codexspec/roadmap.md` 或其他 `docs/` 文件。

## Decisions Applied

- API 作为 future scope；当前管理面只要求 CLI 和 signal。
- eBPF 是当前必需能力；权限、load、attach 或 privileged port bind 失败会导致 start/reload 失败。
- SNI/HTTP host 是 future scope；process/user 是当前可选高成本上下文。
- 最小有效配置采用严格显式配置。
- `dns.final` 和 `route.final` 必须显式配置，无匹配 rule 时走显式 final。
- 现有 schema 形状作为当前事实来源；默认值写回 docs；危险默认和下限归 basic validate。
- `listen_port` 不应有默认值。
- DNS/route rule 同时出现对象 `action` 与快捷字段时，`action` 优先。
- remote rule_set 过期缓存刷新失败时，start/external reload 失败；internal refresh 可保留旧 runtime；cache hash 使用 URL/path。

## Notes

本轮更新仍属于 preflight docs update，不是 formal planning，也不表示正式需求冻结。后续正式立项时仍应从更新后的 `docs/` 重新读取需求。

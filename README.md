# Kitty Gateway

Kitty Gateway 是一个面向 Linux 网关场景的高性能 DNS + Proxy Gateway 项目骨架。

本仓库当前包含：

- Rust workspace 多 crate 骨架。
- DDD 风格模块边界。
- `schema` 输入防腐 crate。
- DNS / Route / RuleSet 配置 schema。
- Aya eBPF 分层骨架。
- compile-pipeline 与 match-pipeline 需求文档。

## 阅读顺序

1. `docs/README.md`
2. `docs/00-overview/`
3. `docs/01-architecture/`
4. `docs/02-config/`
5. `docs/03-compile-pipeline/`
6. `docs/04-match-pipeline/`
7. `docs/05-operations/`
8. `docs/06-implementation/`
9. `docs/07-testing/`

## Workspace

```text
crates/bin             管理面入口
crates/application     控制面编排
crates/domain          领域模型与运行态契约
crates/infrastructure  平台适配与外部库防腐
crates/schema          配置输入 schema
crates/ebpf-common     eBPF 用户态/内核态共享布局
crates/ebpf-programs   Aya eBPF program 骨架
```

## 当前边界

当前 ZIP 是干净骨架与 schema 初版，不包含数据面完整实现。
`schema` 只负责反序列化、默认值填充、输入 normalize 与重复字段宏生成，不负责校验、下载、展开、合并或运行态构建。

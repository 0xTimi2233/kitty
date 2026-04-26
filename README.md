# Kitty Gateway

Kitty Gateway 是一个面向 Linux 6.1+ 的高性能 DNS + Proxy Gateway 项目骨架。

项目目标：

- 数据面热路径追求极致性能、低分支、低分配。
- 控制面将用户配置编译为只读运行态模型。
- 管理面通过 CLI、signal、未来 API 调用控制面能力。
- 通过 ACL 与 infrastructure 隔离外部输入和外部库类型。

## Workspace

```text
crates/
  bin/                         # 最终二进制入口与 composition root
  interfaces/                  # CLI / API / signal 外部入口
  application/control-plane/   # compile-pipeline / reload / publish
  application/data-plane/      # match-pipeline / DNS session / proxy session
  domain/                      # 领域模型、Rule IR、Runtime Model、Matcher Model
  infrastructure/              # 文件、网络、DNS protocol、storage、logging、Aya loader 适配
  acl/                         # 配置输入防腐层：schema、serde、默认值、局部 normalize
  ebpf-common/                 # 用户态与 eBPF program 共享 repr(C) 类型
  ebpf-programs/               # Aya eBPF kernel-side program 骨架
```

## 阅读顺序

1. `docs/README.md`
2. `docs/00-overview/00-reading-order.md`
3. `docs/01-architecture/00-layered-ddd.md`
4. `docs/02-config/00-config-contract.md`
5. `docs/03-compile-pipeline/00-orchestration.md`
6. `docs/04-match-pipeline/00-orchestration.md`
7. `docs/05-operations/00-process-bootstrap.md`
8. `docs/06-implementation/00-project-directory.md`
9. `docs/07-testing/00-testing-standard.md`
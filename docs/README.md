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

## Kitty 文档索引

### 第一遍：了解项目

1. `00-overview/00-reading-order.md`
2. `00-overview/01-product-goals.md`
3. `00-overview/02-scope.md`
4. `00-overview/03-terminology.md`

### 第二遍：理解架构

1. `01-architecture/00-layered-ddd.md`
2. `01-architecture/01-workspace-and-crate-boundary.md`
3. `01-architecture/02-dependency-direction.md`
4. `01-architecture/03-management-control-data-plane.md`
5. `01-architecture/04-runtime-model.md`
6. `01-architecture/05-rule-ir-model.md`
7. `01-architecture/06-ebpf-architecture.md`

### 第三遍：理解配置契约

1. `02-config/00-config-contract.md`
2. `02-config/01-acl-schema.md`
3. `02-config/02-default-values.md`
4. `02-config/03-normalize-rules.md`
5. `02-config/04-logging-config.md`
6. `02-config/05-dns-config.md`
7. `02-config/06-route-config.md`
8. `02-config/07-rule-set-config.md`

### 第四遍：理解两个 pipeline

1. `03-compile-pipeline/00-orchestration.md`
2. `04-match-pipeline/00-orchestration.md`

### 第五遍：理解运行与实现

1. `05-operations/00-process-bootstrap.md`
2. `05-operations/01-start.md`
3. `05-operations/02-external-reload.md`
4. `05-operations/03-internal-rule-set-refresh.md`
5. `05-operations/04-cache-cleanup.md`
6. `05-operations/05-logging-and-error.md`
7. `06-implementation/00-project-directory.md`
8. `06-implementation/01-coding-standard.md`
9. `06-implementation/02-dependency-selection.md`
10. `06-implementation/03-acl-code-guide.md`
11. `06-implementation/04-ksr-binary-format.md`
12. `07-testing/00-testing-standard.md`
13. `07-testing/06-performance-testing.md`

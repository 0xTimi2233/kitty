# Kitty Gateway

Kitty Gateway 是一个面向 Linux 6.1+ 的高性能 DNS + Proxy Gateway 项目。

## File Structure

```text
crates/ 代码
docs/ 文档
```

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
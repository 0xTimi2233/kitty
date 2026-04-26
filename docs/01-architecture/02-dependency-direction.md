# 依赖方向

推荐依赖方向：

```text
bin -> interfaces
bin -> application/control-plane
bin -> application/data-plane
bin -> infrastructure

interfaces -> application/control-plane
interfaces -> application/data-plane

application/control-plane -> acl
application/control-plane -> domain

application/data-plane -> domain
application/data-plane -> ebpf-common

infrastructure -> acl
infrastructure -> domain
infrastructure -> ebpf-common

acl -> serde ecosystem only
domain -> no upper layer
ebpf-programs -> ebpf-common
```

`domain` 不依赖 `acl`、`infrastructure`、`interfaces` 或 `application`。外部库类型不能进入 `domain` 或 `application` 的公开模型。

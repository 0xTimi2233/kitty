# 分层 DDD 架构

Kitty 采用泛 DDD 分层，不追求形式化复杂度，核心目标是防腐和依赖方向清晰。

层次：

```text
bin
interfaces
application/control-plane
application/data-plane
domain
infrastructure
acl
ebpf-common
ebpf-programs
```

`domain` 保存核心模型。`acl` 隔离外部配置输入。`infrastructure` 隔离平台和外部库。`application` 负责用例编排。`interfaces` 负责管理面入口。`bin` 负责最终装配和进程入口。

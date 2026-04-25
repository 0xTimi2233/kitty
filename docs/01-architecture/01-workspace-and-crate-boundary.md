# Workspace 与 crate 边界

```text
crates/bin             管理面入口
crates/application     控制面编排
crates/domain          领域模型与运行态契约
crates/infrastructure  平台适配与外部库防腐
crates/schema          配置输入 schema
crates/ebpf-common     eBPF 共享布局
crates/ebpf-programs   Aya eBPF program 骨架
```

## 依赖方向

```text
bin -> application
bin -> infrastructure
application -> schema
application -> domain
infrastructure -> schema
infrastructure -> domain
infrastructure -> ebpf-common
ebpf-programs -> ebpf-common
```

`domain` 不依赖 `schema` 和 `infrastructure`。

`infrastructure -> application` 仅用于实现 application 层定义的端口 trait。

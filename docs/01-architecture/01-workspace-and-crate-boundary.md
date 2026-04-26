# Workspace 与 crate 边界

`crates/bin` 是 composition root，只负责启动、装配和关闭。

`crates/interfaces` 负责 CLI、API、signal，把外部请求转换成 application command/query。

`crates/application/control-plane` 负责 compile-pipeline、reload、runtime publish、cache cleanup 调度。

`crates/application/data-plane` 负责 match-pipeline、DNS session、proxy session、runtime handle。

`crates/domain` 保存 Rule IR、Runtime Model、Matcher Model、Action Model。

`crates/acl` 保存配置 schema、serde helper、默认值和局部 normalize。

`crates/infrastructure` 保存配置文件读取、DNS protocol adapter、storage、network、logging、Aya userspace loader。

`crates/ebpf-common` 保存共享 repr(C) 数据布局。

`crates/ebpf-programs` 保存 Aya kernel-side program 骨架。

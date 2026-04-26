# 管理面、控制面、数据面

管理面：`interfaces`。接收 CLI、API、signal 请求，并转成 application command/query。API 不能启动进程，但可以触发 reload 和配置修改。

控制面：`application/control-plane`。负责编译配置、刷新 rule_set、构建 runtime model、发布运行态、调度发布后维护任务。

数据面：`application/data-plane`。使用只读 runtime model 处理 DNS 与 proxy 流量，执行 matcher、缓存、action dispatch 和 eBPF map sync。

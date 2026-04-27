# 管理面、控制面、数据面

## 管理面

管理面位于 `interfaces`。

当前范围内，管理面只要求：

- CLI。
- signal。

CLI 和 signal 将外部请求转换成 application command/query。API 是 future scope，不进入当前需求细节，不作为当前 reload、状态查询或配置修改入口。

API 未来加入时仍不能启动进程；相关鉴权、错误、配置修改边界和运行期管理能力需要重新进入需求澄清。

## 控制面

控制面位于 `application/control-plane`。

控制面负责编译配置、刷新 rule_set、构建 runtime model、发布运行态、调度发布后维护任务。

## 数据面

数据面位于 `application/data-plane`。

数据面使用只读 runtime model 处理 DNS 与 proxy 流量，执行 matcher、缓存、action dispatch 和 eBPF map sync。

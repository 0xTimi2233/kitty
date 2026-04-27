# Process Bootstrap

Process bootstrap 是进程启动流程，不属于 compile-pipeline。

当前范围内，进程启动和运行期管理入口来自 CLI 和 signal。API 是 future scope，不进入当前 process bootstrap 需求。

职责：

1. 解析 CLI command。
2. 建立 minimal logger。
3. 获取 pid lock。
4. 装配 interfaces、application、infrastructure。
5. 执行 start 或运行期管理命令。
6. 安装 signal handler。

API 未来加入时不能启动进程。API 的 reload、状态查询、配置修改、鉴权和错误行为需要在未来需求中单独定义。

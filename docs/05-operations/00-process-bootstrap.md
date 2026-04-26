# Process Bootstrap

Process bootstrap 是进程启动流程，不属于 compile-pipeline。

职责：

1. 解析 CLI command。
2. 建立 minimal logger。
3. 获取 pid lock。
4. 装配 interfaces、application、infrastructure。
5. 执行 start 或运行期管理命令。
6. 安装 signal handler。

API 没有进程启动权。API 只能在进程已运行后执行 reload、状态查询和配置修改。

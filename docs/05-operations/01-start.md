# Start

Start 由 CLI 触发。

流程：

1. 获取 pid lock。
2. 读取配置。
3. 执行完整 compile-pipeline。
4. 对被引用 remote rule_set 检查缓存；无缓存或过期时同步刷新。
5. 发布 runtime。
6. 应用 listener。
7. 启动数据面。
8. 发布成功后异步调度 cache cleanup。

启动阶段 remote rule_set 无缓存且下载失败时启动失败。

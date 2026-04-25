# Start 与 Reload

## Start

- 获取 pid lock。
- 初始化 bootstrap log。
- 解码配置。
- 检查被引用 rule_set 缓存状态。
- remote rule_set 缓存缺失或过期时同步刷新。
- 编译 runtime。
- 应用 listener plan。
- 原子发布 runtime。

## External reload

- 使用 buffer=1 的 reload 队列。
- 先做配置差异判断。
- no-op 不触发。
- log-only 只替换日志配置。
- 其他变化全量编译。
- 编译失败时 reload 失败。

## Internal rule_set refresh

- 下载、校验、编译失败时记录错误。
- 旧 runtime 继续运行。
- 多个刷新任务可合并触发一次 reload。

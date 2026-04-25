# 04 Reload And Cache Tests

## 目标

验证 start、external reload、internal rule_set refresh 和 cache cleanup 的行为稳定。

## 测试范围

### Start

- remote rule_set 无缓存时同步下载。
- remote rule_set 缓存过期时同步刷新。
- remote rule_set 下载失败时启动失败。
- local rule_set 写入 meta。

### External reload

- no-op reload 不发布 Runtime Model。
- log-only reload 只替换日志配置。
- local rule_set 变化触发 FullCompile。
- 编译失败时旧 Runtime Model 继续运行。

### Internal rule_set refresh

- 下载失败时输出 error 日志。
- 编译失败时旧 Runtime Model 继续运行。
- 多个刷新任务在合并窗口内只触发一次 reload。

### Cache cleanup

- 当前 Runtime Model 不再引用的 remote cache 被删除。
- 当前 Runtime Model 不再引用的 local meta 被删除。
- 正在使用的 cache 不被删除。

## 测试工具

- 使用临时目录模拟 cache root。
- 使用本地 mock server 模拟 remote rule_set。
- 使用可控时钟模拟过期时间。

## 回归要求

修改 reload diff、rule_set cache、publish 或 cleanup 逻辑时必须运行本组测试。

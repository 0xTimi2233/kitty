# External Reload

External reload 可由 CLI、signal 或 API 触发。

reload controller 先执行 diff precheck：

- no-op：不触发编译。
- log-only：只重载日志。
- full-compile：执行完整 compile-pipeline。

外部 reload 编译或发布失败时返回失败，不替换旧 runtime。

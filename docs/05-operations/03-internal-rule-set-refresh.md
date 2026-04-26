# Internal Rule Set Refresh

Internal rule_set refresh 由控制面定时任务触发。

规则：

- update_interval 缺失时默认 1d。
- 0 或 "0s" 表示禁用自动刷新。
- 非 0 interval 不小于 15 分钟。
- 1 分钟窗口内多个 refresh 任务合并成一次 internal reload。
- 下载、编译或发布失败只记录错误，旧 runtime 继续运行。

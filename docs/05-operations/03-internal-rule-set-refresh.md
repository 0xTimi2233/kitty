# Internal Rule Set Refresh

Internal rule_set refresh 由控制面定时任务触发。

## 调度规则

- `update_interval` 缺失时默认 1d。
- `update_interval` 为 0 或 "0s" 表示禁用自动刷新。
- 非 0 `update_interval` 不小于 15 分钟。
- `update_interval` 小于 15 分钟由 basic validate 拒绝。
- 1 分钟窗口内多个 refresh 任务合并成一次 internal reload。

## 失败策略

internal refresh 下载、编译或发布失败时：

- 只记录错误。
- 保留旧 runtime。
- 不触发失败发布。
- 不替换当前 generation。

启动和 external reload 遇到 remote rule_set 过期缓存且刷新失败时失败；该行为不属于 internal refresh 的失败保留旧 runtime 规则。

# Cache Cleanup

缓存清理是发布成功后的异步维护任务。

触发时机：

- start 发布成功后。
- external reload 发布成功后。
- internal rule_set refresh 发布成功后。

不触发清理的情况：

- compile 失败。
- publish 失败。
- listener apply 失败。

控制面发布 RuntimeModel 时同时产出 `CacheKeepSet`。cache cleaner 扫描缓存目录，删除不在 keep set 中的 remote origin cache、local meta、孤儿 KSR、孤儿 meta 和临时文件。

清理失败只记录错误，不回滚 runtime，不影响数据面。

# Cache Cleanup

缓存清理是发布成功后的异步维护任务。

## 触发时机

- start 发布成功后。
- external reload 发布成功后。
- internal rule_set refresh 发布成功后。

不触发清理的情况：

- compile 失败。
- publish 失败。
- listener apply 失败。
- remote rule_set 过期缓存刷新失败导致 start/external reload 失败。

## CacheKeepSet

控制面发布 RuntimeModel 时同时产出 `CacheKeepSet`。cache cleaner 扫描缓存目录，删除不在 keep set 中的 remote origin cache、local meta、孤儿 KSR、孤儿 meta 和临时文件。

remote cache hash 使用 URL/path 输入。local meta hash 使用 path 输入。

## 过期缓存策略

启动和 external reload 遇到 remote rule_set 过期缓存且刷新失败时失败，不使用 stale-cache fallback。

internal rule_set refresh 失败时保留旧 runtime，不替换当前 generation。

## 失败策略

清理失败只记录错误，不回滚 runtime，不影响数据面。

# rule_set 加载与缓存

remote rule_set：

- 仅被引用时下载和编译。
- start 阶段缓存缺失或过期时同步刷新。
- start 刷新失败则启动失败。
- 自动刷新失败时保留旧 runtime。
- 缓存路径：`.cache/rule_set/origin/{url_hash}/rule.{json|ksr}`。
- meta 路径：`.cache/rule_set/origin/{url_hash}/meta.json`。

local rule_set：

- 不缓存源文件。
- 写入 `.cache/rule_set/local/{path_hash}/meta.json`。

缓存清理：

- 只保留当前运行所需缓存。
- 发布新 runtime 后清理未被引用缓存。

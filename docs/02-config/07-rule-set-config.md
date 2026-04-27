# Rule Set 配置

rule_set 类型：

- local。
- remote。

format：

- source。
- binary。

binary 后缀为 `.ksr`。

## update_interval

- `update_interval` 缺失时默认 1d。
- `update_interval` 为 0 或 "0s" 表示禁用自动刷新。
- 非 0 `update_interval` 小于 15 分钟时，由 basic validate 拒绝。

ACL schema 只负责 decode/default/local normalize，不负责检查 interval 下限。

## Cache path

remote rule_set 缓存路径：

```text
.cache/rule_set/origin/{url_hash}/rule.{json|ksr}
.cache/rule_set/origin/{url_hash}/meta.json
```

local rule_set meta 路径：

```text
.cache/rule_set/local/{path_hash}/meta.json
```

remote cache hash 使用 URL/path 输入；local meta hash 使用 path 输入。

## 引用与加载

未被引用的 rule_set 只做 schema/basic 校验，不下载、不解码、不生成 KSR。

启动和 external reload 遇到 remote rule_set 过期缓存且刷新失败时失败。无缓存且下载失败时失败。缓存校验失败时失败。

internal rule_set refresh 失败时只记录错误并保留旧 runtime。internal refresh 不替换旧 runtime，不触发失败发布。

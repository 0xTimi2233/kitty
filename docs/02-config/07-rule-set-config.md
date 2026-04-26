# Rule Set 配置

rule_set 类型：

- local。
- remote。

format：

- source。
- binary。

remote rule_set 缓存路径：

```text
.cache/rule_set/origin/{url_hash}/rule.{json|ksr}
.cache/rule_set/origin/{url_hash}/meta.json
```

local rule_set meta 路径：

```text
.cache/rule_set/local/{path_hash}/meta.json
```

未被引用的 rule_set 只做 schema/basic 校验，不下载、不解码、不生成 KSR。

# 09 Rule Set Load Cache

## 职责

Rule Set Load Cache 负责加载被引用的 local rule_set，刷新或复用被引用的 remote rule_set 缓存，并维护 cache meta。

## 输入

- `ReferencedRuleSetSet`
- rule_set 配置。
- cache root。
- reload trigger kind。

## 输出

- `LoadedRuleSetSource`
- `RuleSetMetaSet`

## 类型

rule_set 由两个维度描述：

- `type`：`local` 或 `remote`。
- `format`：`source` 或 `binary`。

`binary` 文件使用项目自定义 KSR 格式。

## remote cache 路径

```text
.cache/rule_set/origin/{url_hash}/rule.{json|ksr}
.cache/rule_set/origin/{url_hash}/meta.json
```

## local meta 路径

```text
.cache/rule_set/local/{path_hash}/meta.json
```

local rule_set 不缓存源文件，只写入 meta。

## remote 刷新规则

### start

- 无缓存：同步下载。
- 缓存过期：同步刷新。
- 下载或刷新失败：启动失败。

### external reload

- 只有配置变化或被引用缓存过期时才刷新。
- 刷新失败时 external reload 失败。

### internal rule_set refresh

- 刷新失败时输出 error 日志。
- 保留旧 Runtime Model 继续运行。

## meta 字段

remote meta 至少包含：

- `url`
- `format`
- `fetched_at`
- `update_interval`
- `rule_hash`
- `etag`
- `last_modified`

local meta 至少包含：

- `canonical_path`
- `modified_at`
- `file_len`
- `rule_hash`

## cache cleanup

发布新 Runtime Model 后，删除当前运行态不再需要的 rule_set cache 文件和 meta。

## 测试要点

- start 阶段 remote 缓存过期会同步刷新。
- start 阶段 remote 无缓存且下载失败会失败。
- internal refresh 失败不会替换旧 Runtime Model。
- 未引用 cache 会在 cleanup 阶段删除。

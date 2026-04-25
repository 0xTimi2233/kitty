# 07 Test Data And Fixtures

## 目标

测试数据和 fixture 需要覆盖 schema、compile pipeline、match pipeline、reload、cache 和性能测试场景。

## 目录建议

```text
tests/fixtures/
  config/
    minimal.json
    full.json
    invalid/
  rule_set/
    source/
    ksr/
  matcher/
    dns/
    route/
  reload/
    old/
    new/
  performance/
    small/
    medium/
    large/
```

## 生成要求

- 大规模 fixture 使用生成器生成，不手写大文件。
- 生成器必须固定随机种子。
- 每个 fixture 需要说明目标场景。
- invalid fixture 必须说明预期错误类型。

## matcher fixture

matcher fixture 至少包含：

- 规则列表。
- 请求上下文列表。
- slow oracle 预期结果。
- indexed matcher 预期结果。

## rule_set fixture

rule_set fixture 至少包含：

- local source。
- remote source。
- KSR binary。
- meta file。
- 过期 meta。
- 损坏 KSR header。

## performance fixture

performance fixture 至少包含：

- small。
- medium。
- large。
- fixed seed。
- expected scale description。

## 维护要求

修改 schema、规则语义或 Runtime Model 后，必须同步检查 fixture 是否仍能表达目标语义。

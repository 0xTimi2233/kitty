# 03 Match Pipeline Tests

## 目标

验证 Match Pipeline 的优化路径与 slow oracle 保持一致，并确保 first-match 语义不被 bitmap、索引或上下文补充破坏。

## 测试范围

### Slow oracle

为 DNS rule 和 Route rule 分别实现 slow sequential matcher。

slow oracle 必须：

- 不使用倒排索引。
- 不使用 bitmap。
- 按规则顺序逐条检查。
- 作为 optimized matcher 的语义基准。

### Indexed matcher

- exact domain 命中。
- suffix domain 命中。
- keyword domain 命中。
- regex domain 命中。
- IP CIDR 命中。
- port range 命中。
- process/user matcher 命中。

### Bitmap short-circuit

- OR group。
- AND group。
- ANY group。
- not 条件。

### First-match

- 多条规则同时命中时返回第一条。
- bitmap 候选顺序不影响结果。
- fallback action 在无命中时生效。

### Context enrich

- 未配置 process matcher 时不补充 process。
- 未配置 user matcher 时不补充 user。
- sniff 结果只在需要时读取。

## 差分测试

随机生成规则和请求上下文，比较：

```text
slow_oracle(context) == indexed_matcher(context)
```

失败时输出：

- seed。
- 规则列表。
- 请求上下文。
- slow result。
- indexed result。

## 回归要求

任何修改 index、bitmap 或 evaluator 的代码，都必须运行差分测试。

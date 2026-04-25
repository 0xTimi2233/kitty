# 06 Performance Testing

## 目标

性能测试用于验证 Kitty 在大规则集、高 QPS、高连接数和频繁 reload 场景下仍能保持低延迟、低分配和稳定吞吐。

## 指标

必须记录以下指标：

- throughput。
- p50 / p90 / p99 / p999 latency。
- CPU usage。
- memory usage。
- allocation count。
- peak RSS。
- compile duration。
- reload duration。
- Runtime publish pause。
- DNS cache hit ratio。
- eBPF fast path hit ratio。

## Benchmark 组

### Schema decode

测试不同大小配置的 decode 成本：

- minimal config。
- medium config。
- large config。
- large rule_set references。

### Compile Pipeline

测试完整编译成本：

- 1k rules。
- 10k rules。
- 100k rules。
- mixed DNS + Route rules。
- source rule_set。
- KSR rule_set。

### Rule set expand merge

测试 rule_set 展开合并成本：

- 单 rule_set。
- 多 rule_set。
- 多分支 rule_set。
- 多 condition group。

### Match Index Compile

测试索引构建成本：

- exact domain。
- suffix domain。
- keyword domain。
- regex domain。
- CIDR。
- mixed matcher。

### Match Pipeline

测试热路径匹配成本：

- DNS exact domain hit。
- DNS suffix hit。
- DNS cache hit。
- DNS cache miss。
- Route exact domain hit。
- Route IP CIDR hit。
- Route port hit。
- no match fallback。
- process/user matcher disabled。
- process/user matcher enabled。

### DNS runtime

测试：

- UDP query cache hit。
- UDP query upstream miss。
- TCP query user path。
- lazy cache hit。
- dump restore。

### Reload

测试：

- no-op reload。
- log-only reload。
- local rule_set changed reload。
- remote rule_set refresh reload。
- full compile reload under active traffic。

### eBPF userspace sync

测试：

- DNS A map update。
- DNS AAAA map update。
- route mark update。
- TTL cleanup。
- Runtime generation cleanup。

## 数据规模

性能测试必须固定以下规模档位：

| 档位     | 规则数量 | domain 数 | CIDR 数 | regex 数 | 并发连接 |
|--------|-----:|---------:|-------:|--------:|-----:|
| small  |   1k |      10k |     1k |     100 |   1k |
| medium |  10k |     100k |    10k |      1k |  10k |
| large  | 100k |       1m |   100k |     10k | 100k |

## 执行规则

- 每次 benchmark 必须记录 git revision、target、CPU、kernel version、优化参数。
- 每个 benchmark 至少 warm up 一轮。
- 每个 benchmark 输出机器可读结果。
- 性能回归需要和上一份 baseline 比较。

## 通过标准

具体数值由硬件基线确定。每次提交至少需要满足：

- hot path 无明显分配回归。
- p99 latency 不出现异常尖刺。
- full compile 时间不随无关字段变化而增长。
- no-op reload 不触发完整编译。
- log-only reload 不触发 match index compile。

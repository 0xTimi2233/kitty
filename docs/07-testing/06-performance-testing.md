# Performance Testing

覆盖方向：

- schema decode。
- rule_set compile。
- index compile。
- matcher QPS。
- DNS cache。
- eBPF map sync。
- reload 延迟。
- 内存占用。

早期性能文档只要求建立 benchmark harness 和数据规模占位。硬阈值等待数据面、匹配器和运行环境需求进一步明确后再定义。

每个 benchmark 必须记录：

- 数据规模。
- baseline 来源。
- 采样方式。
- 运行环境。
- 是否已定义硬阈值。

未定义硬阈值的 benchmark 不得作为正式 pass/fail 门槛，只能作为趋势和回归观察输入。

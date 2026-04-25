# 测试规范

测试目录：

- crate 级集成测试放在 `crates/<crate>/tests/`。
- 跨 crate 测试放在 workspace `tests/`。
- 生产代码目录不放测试模块。

核心测试：

1. schema decode 测试。
2. default value 测试。
3. normalize 测试。
4. rule_set 展开合并测试。
5. slow matcher 与 indexed matcher 差分测试。
6. start / reload 流程测试。
7. eBPF map layout 测试。

matcher 测试必须包含 slow oracle，确保优化不改变 first-match 语义。

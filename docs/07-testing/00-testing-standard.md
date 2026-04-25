# 测试规范

## 测试目录

- crate 级集成测试放在 `crates/<crate>/tests/`。
- 跨 crate 测试放在 workspace `tests/`。
- 生产代码目录不放测试模块。

## 文档索引

1. `01-schema-tests.md`
2. `02-compile-pipeline-tests.md`
3. `03-match-pipeline-tests.md`
4. `04-reload-and-cache-tests.md`
5. `05-ebpf-tests.md`
6. `06-performance-testing.md`
7. `07-test-data-and-fixtures.md`

## 测试分层

### Schema tests

验证 schema decode、默认值、one-or-many、normalize helper。

### Compile Pipeline tests

验证配置编译、rule_set 加载、规则展开、索引编译和发布计划。

### Match Pipeline tests

验证 slow oracle、indexed matcher、bitmap short-circuit 和 first-match evaluator 一致性。

### Operation tests

验证 start、external reload、internal rule_set refresh、cache cleanup。

### eBPF tests

验证 shared layout、map key/value、userspace sync 语义。

### Performance tests

验证吞吐、延迟、分配、内存和 reload 成本。

## 核心原则

- matcher 测试必须包含 slow oracle。
- 所有优化路径必须与 slow oracle 做差分测试。
- 性能测试必须固定数据规模、随机种子和 runtime 参数。
- 失败用例必须覆盖错误日志和错误路径。

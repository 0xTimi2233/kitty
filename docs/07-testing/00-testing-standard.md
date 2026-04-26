# 测试规范

测试不写在 `src/` 下。

测试目录：

- ACL crate 测试：`crates/acl/tests/`。
- 控制面测试：`crates/application/control-plane/tests/`。
- 数据面测试：`crates/application/data-plane/tests/`。
- 跨 crate 集成测试：根 `tests/`。

必须维护 slow matcher oracle，用于和 indexed matcher 做差分测试。

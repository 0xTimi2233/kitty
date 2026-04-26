# 项目目录结构

项目根目录：

```text
crates/
docs/
tests/
Cargo.toml
README.md
```

`src/` 下不放测试代码。跨 crate 集成测试放在各 crate 的 `tests/` 目录或根 `tests/` 目录。

`crates/bin` 的 package 名称用于 workspace 管理，最终二进制名为 `kitty`。

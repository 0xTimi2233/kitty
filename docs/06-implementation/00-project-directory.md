# 项目目录结构

```text
kitty-gateway/
  Cargo.toml
  crates/
    bin/
    application/
    domain/
    infrastructure/
    schema/
    ebpf-common/
    ebpf-programs/
  docs/
  tests/
```

模块原则：

- 不使用 `mod.rs`。
- 使用 `module.rs + module/` 目录。
- `schema` 目录按 JSON 层级组织。
- `infrastructure` 按外部适配领域组织。

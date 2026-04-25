# 自定义 Codex Agents 布局

Codex 官方 custom agents 放在：

```text
.codex/agents/
  pm.toml
  architect.toml
  qa.toml
  engineer.toml
  verifier.toml
  reviewer.toml
```

本目录下的 `02-roles/*.toml` 是示例文件。要启用时，复制到项目根 `.codex/agents/`。

每个 TOML 文件必须包含：

```toml
name = "agent_name"
description = "什么时候使用这个 agent"
developer_instructions = """
角色行为说明。
"""
```

建议：

- role prompt 用中文写，关键工程术语保留英文。
- role 只写职责、输入、输出、禁止事项。
- 项目事实放 docs，不放 role prompt。
- read-only 角色设置 `sandbox_mode = "read-only"`。
- execution 角色使用 `workspace-write`。
- `agents.max_depth` 保持 `1`，避免 recursive fan-out。

调用示例：

```text
Spawn pm、architect、qa 三个 subagents。
pm 负责需求和 roadmap。
architect 负责 ADR 和技术边界。
qa 负责 test-plan。
等待全部完成后，由主线程整合为 docs/product/roadmap.md。
```


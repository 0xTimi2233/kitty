---
name: mini-ralph
description: Use when the user says mini-ralph or wants one engineer to execute exactly one approved task to completion with verification and status update.
---

# Mini Ralph

使用 `docs/mini_omx/03-workflows/04-ralph.md`。

规则：

- 只选择一个 `approved` task。
- 执行前读取 spec、test-plan、status。
- 使用 `engineer` 实现。
- 使用 `verifier` 验证。
- 失败后在同一 scope 内修复并重跑验证。
- 同一失败重复 3 次后停止并标记 blocked。
- 完成后更新 status。


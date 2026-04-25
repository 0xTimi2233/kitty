---
name: mini-verify
description: Use when the user says mini-verify or wants read-only validation of changes against spec, test-plan, and acceptance criteria.
---

# Mini Verify

使用 `docs/mini_omx/03-workflows/05-verify.md`。

规则：

- 使用 `verifier` 做证据化验证。
- 使用 `reviewer` 做 correctness、security、maintainability、missing tests 审查。
- 不实现修复。
- 输出 PASS / PARTIAL / FAIL。


---
name: mini-intake
description: Use when the user says mini-intake, wants to clarify requirements, starts a new product/feature, or asks for PM-style requirement validation before planning or coding.
---

# Mini Intake

使用 `docs/mini_omx/03-workflows/00-intake.md`。

规则：

- 自动读取 `AGENTS.md` 和 `docs/mini_omx/readme.md`。
- 使用 `pm` 作为主角色。
- 需要架构边界时调用 `architect`。
- 需要验收标准时调用 `qa`。
- 只写需求文档，不写代码。

默认输出：

- `vision.md`
- `requirements.md`
- `non-goals.md`
- `acceptance.md`
- open questions


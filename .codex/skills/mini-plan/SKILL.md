---
name: mini-plan
description: Use when the user says mini-plan or wants to convert approved requirements into roadmap, ADRs, specs, test plans, and status without implementation.
---

# Mini Plan

使用 `docs/mini_omx/03-workflows/01-plan.md`。

规则：

- 只规划，不实现。
- 使用 `pm` 生成 milestone 和 task board。
- 使用 `architect` 生成 ADR 和 technical boundaries。
- 使用 `qa` 生成 test-plan 和 verification commands。
- 任何 task 只有具备 spec、test-plan、write scope、acceptance criteria 后才能标为 `approved`。

默认输出：

- `roadmap.md`
- `adr/*.md`
- `specs/*.md`
- `specs/*-test-plan.md`
- `status.md`


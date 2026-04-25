---
name: mini-team
description: Use when the user says mini-team or wants several approved independent tasks executed by subagents with tracked team state.
---

# Mini Team

使用 `docs/mini_omx/03-workflows/03-team.md`。

规则：

- 只选择 `approved` task。
- 每个 task 必须有 spec、test-plan 和 disjoint write scope。
- 默认最多并行 3 个 worker。
- 每个 worker 只执行一个 task。
- 主线程创建 team run 状态目录并维护 integration-report。
- 主线程负责最终 status update。

默认 run state：

```text
docs/mini_omx/runs/<run-id>/
  manifest.md
  tasks.md
  workers.md
  events.md
  integration-report.md
```


# Team State Protocol

Mini OMX 的 team 状态协议用于替代 OMX `.omx/state/team` 的核心价值：让主线程可以跟踪 worker、任务、证据和整合结果。

## Run ID

格式：

```text
TEAM-YYYYMMDD-NNN
```

示例：

```text
TEAM-20260425-001
```

## 状态目录

推荐：

```text
docs/mini_omx/runs/TEAM-YYYYMMDD-NNN/
  manifest.md
  tasks.md
  workers.md
  events.md
  integration-report.md
```

如果不希望 run state 入库，可以使用：

```text
.agent/state/team/TEAM-YYYYMMDD-NNN/
```

但 `status.md` 中必须记录实际路径。

## Task State

| State | Meaning |
|---|---|
| `pending` | 已进入 run，但未分配 |
| `claimed` | worker 已领取 |
| `in-progress` | worker 正在执行 |
| `blocked` | worker 被明确阻塞 |
| `done` | worker 完成本地验证 |
| `failed` | worker 无法完成 |
| `integrated` | 主线程已整合并验证 |

## Worker Report 必填字段

- task id
- role
- status
- last update
- files touched
- commands run
- result
- blockers
- remaining risks

## Leader 必做检查

- write scope 是否冲突。
- worker 是否修改了 forbidden files。
- 每个 done task 是否有验证证据。
- integration verification 是否运行。
- status 是否更新。


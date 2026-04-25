# Workflow: Team

用途：模拟 OMX `$team` 的状态跟踪，但不依赖 OMX runtime。适合多个互不冲突的 approved tasks 并行执行。

## 前置条件

- `status.md` 中存在多个 `approved` task。
- 每个 task 都有 spec 和 test-plan。
- 每个 task 都有 disjoint write scope。
- 主线程能在本轮持续跟踪 worker 结果。

## 禁止使用 Team 的情况

- 任务共享同一核心文件。
- 任务需要同一个未决 ADR。
- 任务没有测试计划。
- 主线程没有时间做 integration verification。

## Run State

每次 team run 创建状态目录：

```text
docs/mini_omx/runs/<run-id>/
  manifest.md
  tasks.md
  workers.md
  events.md
  integration-report.md
```

如果项目不希望把 run state 放在 `docs/mini_omx/runs/`，可以改为 `.agent/state/team/<run-id>/`，但必须在 `status.md` 中写明路径。

## Leader Protocol

1. 读取 `status.md`。
2. 选择最多 3 个 `approved` 且互不冲突的 task。
3. 为每个 task 分配一个 Engineer worker。
4. 写 `manifest.md`、`tasks.md`、`workers.md`、`events.md` 初始状态。
5. 对每个 worker 明确：
   - task id
   - spec path
   - test-plan path
   - write scope
   - forbidden files
   - expected output
6. 等待 worker 完成。
7. 收集 worker 报告。
8. 运行 integration verification。
9. 更新 `integration-report.md`。
10. 更新项目 `status.md`。

## Worker Prompt Template

```text
你是 engineer worker。
任务：<task-id>
读取：
- <spec-path>
- <test-plan-path>
- <status-path>

只允许修改：
- <write-scope>

禁止修改：
- <forbidden-files>

完成后报告：
- files touched
- commands run
- result
- blockers
- remaining risks
```

## 状态枚举

- `pending`
- `claimed`
- `in-progress`
- `blocked`
- `done`
- `failed`
- `integrated`


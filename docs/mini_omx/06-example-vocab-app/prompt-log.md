# Prompt Log: 背单词 Web App

本文件演示安装 Mini OMX 后，用户不需要每次重复长提示词。

## 1. 需求澄清

```text
mini-intake: 背单词 Web App，MVP 先做个人本地版。
```

系统应自动加载：

- `AGENTS.md`
- `docs/mini_omx/readme.md`
- `docs/mini_omx/03-workflows/00-intake.md`
- `pm` role

输出示例：

- `docs/mini_omx/06-example-vocab-app/vision.md`

## 2. 规划

```text
mini-plan: 基于背单词 Web App 的 vision 生成 M0-M3 roadmap、ADR、spec、test-plan 和 status。
```

系统应自动调用：

- `pm`
- `architect`
- `qa`

输出示例：

- `docs/mini_omx/06-example-vocab-app/roadmap.md`
- `docs/mini_omx/06-example-vocab-app/adr/ADR-0001-review-scheduler.md`
- `docs/mini_omx/06-example-vocab-app/specs/wordbook.md`
- `docs/mini_omx/06-example-vocab-app/specs/wordbook-test-plan.md`
- `docs/mini_omx/06-example-vocab-app/status.md`

## 3. 单任务开发

```text
mini-ralph: 执行 VOCAB-001。
```

系统应自动读取：

- `status.md`
- `specs/wordbook.md`
- `specs/wordbook-test-plan.md`

然后由 `engineer` 实现，由 `verifier` 验证。

## 4. 多任务并行

```text
mini-team: 并行执行 VOCAB-001 和 VOCAB-002。
```

系统应自动创建 run state：

- `docs/mini_omx/runs/TEAM-20260425-001/manifest.md`
- `docs/mini_omx/runs/TEAM-20260425-001/tasks.md`
- `docs/mini_omx/runs/TEAM-20260425-001/workers.md`
- `docs/mini_omx/runs/TEAM-20260425-001/events.md`
- `docs/mini_omx/runs/TEAM-20260425-001/integration-report.md`

## 5. 验证

```text
mini-verify: 验证 VOCAB-001 和 VOCAB-002 是否满足 spec/test-plan。
```

系统应自动使用 `verifier` 和 `reviewer`，输出 PASS / PARTIAL / FAIL。


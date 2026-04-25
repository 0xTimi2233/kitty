# Team Run Manifest

Run ID: TEAM-20260425-001
Project: 背单词 Web App
Mode: simulated mini-team
Started: 2026-04-25
Leader: main thread
Source status: `docs/mini_omx/06-example-vocab-app/status.md`

## Goal

演示 Mini OMX 的 team 状态跟踪：并行分配 Wordbook CRUD 和 Review Session 两个 approved tasks。

## Workers

| Worker | Role | Task | Write Scope |
|---|---|---|---|
| worker-1 | engineer | VOCAB-001 | wordbook module |
| worker-2 | engineer | VOCAB-002 | review module |

